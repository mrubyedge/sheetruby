#![allow(clippy::not_unsafe_ptr_arg_deref)]
use std::ffi::{CStr, CString};
use std::os::raw::c_char;
use std::rc::Rc;

use mruby_compiler2_sys::MRubyCompiler2Context;
use mrubyedge::yamrb::value::RObject;

fn main() {}

/// Execute Ruby script and return the result as Rc<RObject>
/// Returns None if compilation or execution fails
fn eval_ruby_script(text: &str) -> Option<Rc<RObject>> {
    let mut context = unsafe { MRubyCompiler2Context::new() };

    // Compile the Ruby script
    let mrb = match unsafe { context.compile(text) } {
        Ok(bytecode) => bytecode,
        Err(e) => {
            eprintln!("Compilation error: {}", e);
            eprintln!("Please check your Ruby code and try again");
            return None;
        }
    };

    // Load and execute the bytecode
    let mut rite = match mrubyedge::rite::load(&mrb) {
        Ok(r) => r,
        Err(e) => {
            eprintln!("Failed to load bytecode: {:?}", e);
            return None;
        }
    };

    let mut vm = mrubyedge::yamrb::vm::VM::open(&mut rite);
    mrubyedge_math::init_math(&mut vm);

    // Execute the script and handle exceptions
    match vm.run() {
        Ok(r) => Some(r),
        Err(e) => {
            eprintln!("Runtime error: {:?}", e);
            None
        }
    }
}

/// Convert C string pointer to Rust &str
unsafe fn cstr_to_str<'a>(text_ptr: *const c_char) -> &'a str {
    let c_str = CStr::from_ptr(text_ptr);
    c_str.to_str().unwrap_or("")
}

// Function called from JavaScript
// Receives Ruby script, executes it, and returns integer result
#[unsafe(no_mangle)]
pub extern "C" fn eval_ruby_script_int(text_ptr: *const c_char) -> i32 {
    unsafe {
        let text = cstr_to_str(text_ptr);
        match eval_ruby_script(text) {
            Some(result) => (&*result).try_into().unwrap_or(-1),
            None => -1,
        }
    }
}

// Receives Ruby script, executes it, and returns float result
#[unsafe(no_mangle)]
pub extern "C" fn eval_ruby_script_float(text_ptr: *const c_char) -> f64 {
    unsafe {
        let text = cstr_to_str(text_ptr);
        match eval_ruby_script(text) {
            Some(result) => (&*result).try_into().unwrap_or(f64::NAN),
            None => f64::NAN,
        }
    }
}

// Receives Ruby script, executes it, and returns boolean result (0 or 1)
#[unsafe(no_mangle)]
pub extern "C" fn eval_ruby_script_bool(text_ptr: *const c_char) -> i32 {
    unsafe {
        let text = cstr_to_str(text_ptr);
        match eval_ruby_script(text) {
            Some(result) => {
                let b: bool = (&*result).try_into().unwrap_or(false);
                if b { 1 } else { 0 }
            }
            None => 0,
        }
    }
}

// Receives Ruby script, executes it, and returns string result
// Returns a pointer to a null-terminated C string (caller should NOT free it)
// The returned string is valid until the next call to this function
static mut LAST_STRING_RESULT: Option<CString> = None;

#[unsafe(no_mangle)]
pub extern "C" fn eval_ruby_script_string(text_ptr: *const c_char) -> *const c_char {
    unsafe {
        let text = cstr_to_str(text_ptr);
        match eval_ruby_script(text) {
            Some(result) => {
                let s: String = (&*result).try_into().unwrap_or_default();
                match CString::new(s) {
                    Ok(cstring) => {
                        let ptr = cstring.as_ptr();
                        LAST_STRING_RESULT = Some(cstring);
                        ptr
                    }
                    Err(_) => std::ptr::null(),
                }
            }
            None => std::ptr::null(),
        }
    }
}
