#![feature(start, lang_items, const_fn, panic_implementation)]
#![no_std]
#![allow(non_camel_case_types)]
#![allow(non_upper_case_globals)]
#![allow(non_snake_case)]
#![allow(dead_code)]

pub mod libnx {
    mod lang_items {
        pub enum c_void {}
        pub type c_char = i8;
        pub type c_int = i32;
        pub type c_long = i64;
        pub type c_longlong = i64;
        pub type c_schar = i8;
        pub type c_short = i16;
        pub type c_uchar = u8;
        pub type c_uint = u32;
        pub type c_ulong = u64;
        pub type c_ulonglong = u64;
        pub type c_ushort = u16;
    }
    include!(concat!(env!("OUT_DIR"), "/bindings.rs"));
}