#![recursion_limit = "128"]
#![no_std]
#![warn(unsafe_code)]
#![warn(rust_2018_idioms)]
#![allow(dead_code)]

use core::pin::Pin;
use pin_project::{pin_projectable, UnsafeUnpin};

#[test]
fn unsafe_unpin() {
    #[pin_projectable(unsafe_Unpin)]
    pub struct Blah<T> {
        field_1: u8,
        #[pin] field_2: Option<T>
    }

    fn blah<T>() where Blah<T>: Unpin {}

    unsafe impl<T> UnsafeUnpin for Blah<T> where T: Unpin + core::fmt::Display {}
}

