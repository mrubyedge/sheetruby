#![allow(clippy::not_unsafe_ptr_arg_deref)]
use std::ffi::{CStr, CString};
use std::os::raw::c_char;
use std::rc::Rc;

use mruby_compiler2_sys::MRubyCompiler2Context;
use mrubyedge::yamrb::value::RObject;
use mrubyedge::yamrb::vm::VM;

fn main() {}

/// Execute Ruby script and return the result as Rc<RObject>
/// Returns None if compilation or execution fails
fn eval_ruby_script(text: &str) -> Option<Rc<RObject>> {
    eval_ruby_script_with_setup(text, |_| {})
}

/// Execute Ruby script with VM setup callback
/// The callback can be used to set global variables before execution
fn eval_ruby_script_with_setup<F>(text: &str, setup: F) -> Option<Rc<RObject>>
where
    F: FnOnce(&mut VM),
{
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

    // Call setup callback to configure VM (e.g., set global variables)
    setup(&mut vm);

    // Execute the script and handle exceptions
    let result = vm.run();
    if let Err(e) = result {
        eprintln!("Runtime error: {:?}", e);
        return None;
    }
    mrubyedge_serde_json::mrb_json_class_dump(&mut vm, &[result.unwrap()])
        .unwrap_or_else(|_| RObject::string("null".to_string()).to_refcount_assigned())
        .into()
}

/// Convert C string pointer to Rust &str
unsafe fn cstr_to_str<'a>(text_ptr: *const c_char) -> &'a str {
    let c_str = CStr::from_ptr(text_ptr);
    c_str.to_str().unwrap_or("")
}

// Receives Ruby script, executes it, and returns string result
// Returns a pointer to a null-terminated C string (caller should NOT free it)
// The returned string is valid until the next call to this function
static mut LAST_STRING_RESULT: Option<CString> = None;

#[unsafe(no_mangle)]
pub extern "C" fn eval_ruby_script_returning_json(text_ptr: *const c_char) -> *const c_char {
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

#[unsafe(no_mangle)]
pub extern "C" fn eval_ruby_script_returning_json1(
    text_ptr: *const c_char,
    arg1_ptr: *const c_char,
) -> *const c_char {
    unsafe {
        let text = cstr_to_str(text_ptr);
        let arg1 = cstr_to_str(arg1_ptr);
        match eval_ruby_script_with_setup(text, |vm| {
            let arg1_str = RObject::string(arg1.to_string()).to_refcount_assigned();
            let arg1_json = mrubyedge_serde_json::mrb_json_class_load(vm, &[arg1_str])
                .unwrap_or_else(|_| RObject::nil().to_refcount_assigned());
            vm.globals.insert("$arg1".to_string(), arg1_json);
        }) {
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

#[unsafe(no_mangle)]
pub extern "C" fn eval_ruby_script_returning_json2(
    text_ptr: *const c_char,
    arg1_ptr: *const c_char,
    arg2_ptr: *const c_char,
) -> *const c_char {
    unsafe {
        let text = cstr_to_str(text_ptr);
        let arg1 = cstr_to_str(arg1_ptr);
        let arg2 = cstr_to_str(arg2_ptr);
        match eval_ruby_script_with_setup(text, |vm| {
            let arg1_str = RObject::string(arg1.to_string()).to_refcount_assigned();
            let arg1_json = mrubyedge_serde_json::mrb_json_class_load(vm, &[arg1_str])
                .unwrap_or_else(|_| RObject::nil().to_refcount_assigned());
            vm.globals.insert("$arg1".to_string(), arg1_json);

            let arg2_str = RObject::string(arg2.to_string()).to_refcount_assigned();
            let arg2_json = mrubyedge_serde_json::mrb_json_class_load(vm, &[arg2_str])
                .unwrap_or_else(|_| RObject::nil().to_refcount_assigned());
            vm.globals.insert("$arg2".to_string(), arg2_json);
        }) {
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

#[unsafe(no_mangle)]
pub extern "C" fn eval_ruby_script_returning_json3(
    text_ptr: *const c_char,
    arg1_ptr: *const c_char,
    arg2_ptr: *const c_char,
    arg3_ptr: *const c_char,
) -> *const c_char {
    unsafe {
        let text = cstr_to_str(text_ptr);
        let arg1 = cstr_to_str(arg1_ptr);
        let arg2 = cstr_to_str(arg2_ptr);
        let arg3 = cstr_to_str(arg3_ptr);
        match eval_ruby_script_with_setup(text, |vm| {
            let arg1_str = RObject::string(arg1.to_string()).to_refcount_assigned();
            let arg1_json = mrubyedge_serde_json::mrb_json_class_load(vm, &[arg1_str])
                .unwrap_or_else(|_| RObject::nil().to_refcount_assigned());
            vm.globals.insert("$arg1".to_string(), arg1_json);

            let arg2_str = RObject::string(arg2.to_string()).to_refcount_assigned();
            let arg2_json = mrubyedge_serde_json::mrb_json_class_load(vm, &[arg2_str])
                .unwrap_or_else(|_| RObject::nil().to_refcount_assigned());
            vm.globals.insert("$arg2".to_string(), arg2_json);

            let arg3_str = RObject::string(arg3.to_string()).to_refcount_assigned();
            let arg3_json = mrubyedge_serde_json::mrb_json_class_load(vm, &[arg3_str])
                .unwrap_or_else(|_| RObject::nil().to_refcount_assigned());
            vm.globals.insert("$arg3".to_string(), arg3_json);
        }) {
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
