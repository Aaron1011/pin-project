//! An attribute that creates a projection struct covering all the fields.
//!
//! ## Examples
//!
//! [`pin_projectable`] attribute creates a projection struct covering all the fields.
//!
//! ```rust
//! use pin_project::pin_projectable;
//! use std::pin::Pin;
//!
//! #[pin_projectable] // `(Unpin)` is optional (create the appropriate conditional Unpin implementation)
//! struct Foo<T, U> {
//!     #[pin]
//!     future: T,
//!     field: U,
//! }
//!
//! impl<T, U> Foo<T, U> {
//!     fn baz(self: Pin<&mut Self>) {
//!         let this = self.project();
//!         let _: Pin<&mut T> = this.future; // Pinned reference to the field
//!         let _: &mut U = this.field; // Normal reference to the field
//!     }
//! }
//!
//! // Automatically create the appropriate conditional Unpin implementation (optional).
//! // impl<T: Unpin, U> Unpin for Foo<T, U> {}
//! ```
//!
//! <details>
//! <summary>Code like this will be generated:</summary>
//!
//! ```rust
//! struct Foo<T, U> {
//!     future: T,
//!     field: U,
//! }
//!
//! struct __FooProjection<'__a, T, U> {
//!     future: ::core::pin::Pin<&'__a mut T>,
//!     field: &'__a mut U,
//! }
//!
//! impl<T, U> Foo<T, U> {
//!     fn project<'__a>(self: ::core::pin::Pin<&'__a mut Self>) -> __FooProjection<'__a, T, U> {
//!         unsafe {
//!             let this = ::core::pin::Pin::get_unchecked_mut(self);
//!             __FooProjection {
//!                 future: ::core::pin::Pin::new_unchecked(&mut this.future),
//!                 field: &mut this.field,
//!             }
//!         }
//!     }
//! }
//!
//! // Automatically create the appropriate conditional Unpin implementation (optional).
//! impl<T, U> Unpin for Foo<T, U> where T: Unpin {}
//! ```
//!
//! </details>
//!
//! [`pin_projectable`] also supports enums, but to use it ergonomically, you need
//! to use the [`project`] attribute.
//!
//! ```rust
//! # #[cfg(feature = "project_attr")]
//! use pin_project::{project, pin_projectable};
//! # #[cfg(feature = "project_attr")]
//! use std::pin::Pin;
//!
//! # #[cfg(feature = "project_attr")]
//! #[pin_projectable] // `(Unpin)` is optional (create the appropriate conditional Unpin implementation)
//! enum Foo<T, U> {
//!     Future(#[pin] T),
//!     Done(U),
//! }
//!
//! # #[cfg(feature = "project_attr")]
//! impl<T, U> Foo<T, U> {
//!     #[project] // Nightly does not need a dummy attribute to the function.
//!     fn baz(self: Pin<&mut Self>) {
//!         #[project]
//!         match self.project() {
//!             Foo::Future(future) => {
//!                 let _: Pin<&mut T> = future;
//!             }
//!             Foo::Done(value) => {
//!                 let _: &mut U = value;
//!             }
//!         }
//!     }
//! }
//!
//! // Automatically create the appropriate conditional Unpin implementation (optional).
//! // impl<T, U> Unpin for Foo<T, U> where T: Unpin {}
//! ```
//!
//! <details>
//! <summary>Code like this will be generated:</summary>
//!
//! ```rust
//! enum Foo<T, U> {
//!     Future(T),
//!     Done(U),
//! }
//!
//! enum __FooProjection<'__a, T, U> {
//!     Future(::core::pin::Pin<&'__a mut T>),
//!     Done(&'__a mut U),
//! }
//!
//! impl<T, U> Foo<T, U> {
//!     fn project<'__a>(self: ::core::pin::Pin<&'__a mut Self>) -> __FooProjection<'__a, T, U> {
//!         unsafe {
//!             match ::core::pin::Pin::get_unchecked_mut(self) {
//!                 Foo::Future(_x0) => __FooProjection::Future(::core::pin::Pin::new_unchecked(_x0)),
//!                 Foo::Done(_x0) => __FooProjection::Done(_x0),
//!             }
//!         }
//!     }
//! }
//!
//! // Automatically create the appropriate conditional Unpin implementation (optional).
//! impl<T, U> Unpin for Foo<T, U> where T: Unpin {}
//! ```
//!
//! </details>
//!
//! See [`pin_projectable`] and [`project`] for more details.
//!
//! [`pin_projectable`]: ./attr.pin_projectable.html
//! [`project`]: ./attr.project.html
//!
#![recursion_limit = "256"]
#![doc(html_root_url = "https://docs.rs/pin-project/0.3.3")]
#![doc(test(attr(deny(warnings), allow(dead_code, unused_assignments, unused_variables))))]
#![warn(unsafe_code)]
#![warn(rust_2018_idioms, unreachable_pub)]
#![warn(single_use_lifetimes)]
#![warn(clippy::all, clippy::pedantic)]
#![warn(clippy::nursery)]
#![feature(proc_macro_hygiene)]

pub use pin_project_internal::*;

pub unsafe trait UnsafeUnpin {}
