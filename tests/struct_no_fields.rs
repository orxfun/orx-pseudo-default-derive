use orx_pseudo_default::PseudoDefault;
use orx_pseudo_default_derive::PseudoDefault;

#[test]
fn struct_unit() {
    #[derive(PseudoDefault, PartialEq, Eq, Debug)]
    struct Nothing;

    assert_eq!(Nothing, Nothing::pseudo_default());
}
