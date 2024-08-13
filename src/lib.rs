#![doc = include_str!("../README.md")]
#![no_std]
#![warn(
    missing_docs,
    clippy::unwrap_in_result,
    clippy::unwrap_used,
    clippy::panic,
    clippy::panic_in_result_fn,
    clippy::float_cmp,
    clippy::float_cmp_const,
    clippy::missing_panics_doc,
    clippy::todo
)]

extern crate alloc;

mod derive;

/// Derives `PseudoDefault` by recursively constructing members calling `pseudo_default()`.
#[proc_macro_derive(PseudoDefault)]
pub fn derive_pseudo_default(input: proc_macro::TokenStream) -> proc_macro::TokenStream {
    derive::derive_pseudo_default(input)
}
