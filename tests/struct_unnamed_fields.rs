extern crate alloc;

use alloc::string::String;
use orx_pseudo_default::PseudoDefault;
use orx_pseudo_default_derive::PseudoDefault;

#[test]
fn struct_unnamed_field() {
    #[derive(PseudoDefault)]
    struct NamedStruct(String);

    assert_eq!(String::pseudo_default(), NamedStruct::pseudo_default().0);
}

#[test]
fn struct_multiple_unnamed_fields() {
    #[derive(PseudoDefault)]
    struct NamedStruct(String, char, Vec<u32>);

    assert_eq!(String::pseudo_default(), NamedStruct::pseudo_default().0);
    assert_eq!(char::pseudo_default(), NamedStruct::pseudo_default().1);
    assert_eq!(
        Vec::<u32>::pseudo_default(),
        NamedStruct::pseudo_default().2
    );
}

#[test]
fn struct_recursive_unnamed_fields() {
    #[derive(PseudoDefault)]
    struct NamedStruct1(String, char, Vec<u32>);

    #[derive(PseudoDefault)]
    struct NamedStruct2(NamedStruct1, char, Vec<u32>);

    assert_eq!(
        String::pseudo_default(),
        NamedStruct2::pseudo_default().0 .0
    );
    assert_eq!(char::pseudo_default(), NamedStruct2::pseudo_default().0 .1);
    assert_eq!(
        Vec::<u32>::pseudo_default(),
        NamedStruct2::pseudo_default().0 .2
    );
    assert_eq!(char::pseudo_default(), NamedStruct2::pseudo_default().1);
    assert_eq!(
        Vec::<u32>::pseudo_default(),
        NamedStruct2::pseudo_default().2
    );
}
