#![allow(non_camel_case_types, non_snake_case, non_upper_case_globals)]

pub mod ffi {
    include!(concat!(env!("OUT_DIR"), "/bindings.rs"));
}
