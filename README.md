# orx-pseudo-default-derive

[![orx-pseudo-default-derive crate](https://img.shields.io/crates/v/orx-pseudo-default-derive.svg)](https://crates.io/crates/orx-pseudo-default-derive)
[![orx-pseudo-default-derive documentation](https://docs.rs/orx-pseudo-default-derive/badge.svg)](https://docs.rs/orx-pseudo-default-derive)

Derives the [`PseudoDefault`](https://crates.io/crates/orx-pseudo-default) trait that allows to create a cheap default instance of a type, which does not claim to be useful.

# Example

```rust
use orx_pseudo_default::PseudoDefault;
use orx_pseudo_default_derive::PseudoDefault;

#[derive(PseudoDefault)]
struct ChildStruct {
    a: String,
    b: char,
    c: Vec<u32>,
}

#[derive(PseudoDefault)]
struct MyStruct {
    x: ChildStruct,
    y: bool,
    z: Option<usize>,
}

assert_eq!(String::pseudo_default(), MyStruct::pseudo_default().x.a);
assert_eq!(char::pseudo_default(), MyStruct::pseudo_default().x.b);
assert_eq!(Vec::<u32>::pseudo_default(), MyStruct::pseudo_default().x.c);

assert_eq!(bool::pseudo_default(), MyStruct::pseudo_default().y);
assert_eq!(
    Option::<usize>::pseudo_default(),
    MyStruct::pseudo_default().z
);
```
