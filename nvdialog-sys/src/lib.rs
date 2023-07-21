pub mod ffi {
    #![allow(non_snake_case)]
    #![allow(non_camel_case_types)]
    #![allow(non_upper_case_globals)]
    #![allow(unused, dead_code)]
    include!(concat!(env!("OUT_DIR"), "/bindings.rs"));
}