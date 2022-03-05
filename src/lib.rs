#![allow(non_upper_case_globals)]
#![allow(non_camel_case_types)]
#![allow(non_snake_case)]
use libc::*;
include!("bindings.rs");

#[cfg(test)]

mod tests {
    #[test]
    fn it() {
        println!("clic=/src/lib.rs");
    }
}

