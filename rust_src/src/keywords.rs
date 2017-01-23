use std::os::raw::c_char;
use std::ptr;

extern crate libc;

use lisp::LispObject;

fn Fkeywordp(object: LispObject) -> LispObject {
    LispObject::from_bool(object.is_keyword())
}

defun!("keywordp",
       Fkeywordp,
       Skeywordp,
       1,
       1,
       ptr::null(),
       "Return t if OBJECT is a keyword.
       This means that it is a symbol with a print name beginning with `:'
       interned in the initial obarray.  

(fn OBJECT)");
