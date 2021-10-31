#![warn(clippy::all)]
#![cfg_attr(not(debug_assertions), deny(warnings))]

#[no_mangle]
pub extern "C" fn add_i32(a: i32, b: i32) -> i32 {
    a + b
}

#[no_mangle]
pub extern "C" fn sub_i32(a: i32, b: i32) -> i32 {
    a - b
}

#[no_mangle]
pub extern "C" fn mul_i32(a: i32, b: i32) -> i32 {
    a * b
}
