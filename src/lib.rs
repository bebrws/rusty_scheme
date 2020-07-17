pub mod reader;
pub mod core;
pub mod interpreter;

use std::os::raw::{c_char};
use std::ffi::{CString, CStr};

use interpreter::ast_walk_interpreter::pixels;


#[no_mangle]
pub unsafe extern fn scheme(to: *const c_char, new_pixels: *mut c_char) -> *mut c_char {
    
    pixels = new_pixels;
    let c_str = unsafe { CStr::from_ptr(to) };
    let interp_string = match c_str.to_str() {
        Err(_) => "",
        Ok(string) => string,
    };
    let interpreter = interpreter::interpreter::new("cps");
    let result = interpreter.execute(interp_string);
    match result {
        Ok(v) => return CString::new(v).unwrap().into_raw(),
        Err(e) => return CString::new("Error").unwrap().into_raw(),
    }
}


//pub mod lexer;
//mod parser;
//mod interpreter;
//mod ast_walk_interpreter;
//mod cps_interpreter;



//#[cfg(not(test))]
//mod repl;