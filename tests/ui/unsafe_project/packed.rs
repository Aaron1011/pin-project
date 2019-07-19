// compile-fail
#![allow(dead_code)]

use pin_project::unsafe_project;

#[unsafe_project(Unpin)]
#[repr(packed, C)] //~ ERROR may not be used on #[repr(packed)] type
struct Foo {
    #[pin]
    field: u8
}

#[unsafe_project(Unpin)]
#[repr(packed, C)] //~ ERROR may not be used on #[repr(packed)] type
enum Blah {
    Tuple(#[pin] u8)
}


fn main() {}
