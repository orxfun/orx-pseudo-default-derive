extern crate alloc;

use alloc::string::String;
use orx_pseudo_default::PseudoDefault;
use orx_pseudo_default_derive::PseudoDefault;

#[test]
fn struct_named_field() {
    #[derive(PseudoDefault)]
    struct NamedStruct {
        x: String,
    }

    assert_eq!(String::pseudo_default(), NamedStruct::pseudo_default().x);
}

#[test]
fn struct_multiple_named_fields() {
    #[derive(PseudoDefault)]
    struct NamedStruct {
        x: String,
        y: char,
        z: Vec<u32>,
    }

    assert_eq!(String::pseudo_default(), NamedStruct::pseudo_default().x);
    assert_eq!(char::pseudo_default(), NamedStruct::pseudo_default().y);
    assert_eq!(
        Vec::<u32>::pseudo_default(),
        NamedStruct::pseudo_default().z
    );
}

#[test]
fn struct_recursive_named_fields() {
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
}
