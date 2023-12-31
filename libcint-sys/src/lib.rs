#![allow(non_upper_case_globals, non_camel_case_types, non_snake_case)]
#![allow(improper_ctypes)] // this one is probably a bad idea

include!(concat!(env!("OUT_DIR"), "/bindings.rs"));
