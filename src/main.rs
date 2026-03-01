#![allow(clippy::not_unsafe_ptr_arg_deref)]
use std::ffi::CStr;
use std::os::raw::c_char;

use mruby_compiler2_sys::MRubyCompiler2Context;

fn main() {}

// Function called from JavaScript
// Receives Ruby script, executes it, and outputs the result
#[unsafe(no_mangle)]
pub extern "C" fn eval_ruby_script_int(text_ptr: *const c_char) -> i32 {
    unsafe {
        // Convert C string to Rust string
        let c_str = CStr::from_ptr(text_ptr);
        let text = c_str.to_str().unwrap_or("");

        let mut context = MRubyCompiler2Context::new();

        // Compile the Ruby script
        let mrb = match context.compile(text) {
            Ok(bytecode) => bytecode,
            Err(e) => {
                eprintln!("Compilation error: {}", e);
                eprintln!("Please check your Ruby code and try again");
                return -1;
            }
        };

        // Load and execute the bytecode
        let mut rite = match mrubyedge::rite::load(&mrb) {
            Ok(r) => r,
            Err(e) => {
                eprintln!("Failed to load bytecode: {:?}", e);
                return -1;
            }
        };

        let mut vm = mrubyedge::yamrb::vm::VM::open(&mut rite);
        // mruby_math::init_math(&mut vm);

        // Execute the script and handle exceptions
        let result = match vm.run() {
            Ok(r) => r,
            Err(e) => {
                eprintln!("Runtime error: {:?}", e);
                return -1;
            }
        };

        result.as_ref().try_into().unwrap_or(-1)
    }
}
