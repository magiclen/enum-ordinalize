#![cfg(feature = "derive")]

use enum_ordinalize::Ordinalize;

#[test]
fn from_ordinal_unsafe_1() {
    #[derive(Debug, PartialEq, Eq, Ordinalize)]
    #[ordinalize(from_ordinal_unsafe(
        pub const fn v,
        doc = "Obtain a variant based on an integer number.",
        doc = "# Safety",
        doc = "You have to ensure that the input integer number can correspond to a variant on your own.",
    ))]
    enum MyEnum {
        A,
        B,
        C,
    }

    assert_eq!(MyEnum::B, unsafe { MyEnum::v(1) });
}

#[test]
fn from_ordinal_unsafe_2() {
    #[derive(Debug, PartialEq, Eq, Ordinalize)]
    #[ordinalize(from_ordinal_unsafe(
        pub fn v,
        doc = "Obtain a variant based on an integer number.",
        doc = "# Safety",
        doc = "You have to ensure that the input integer number can correspond to a variant on your own.",
    ))]
    enum MyEnum {
        A,
        B,
        C,
    }

    assert_eq!(MyEnum::B, unsafe { MyEnum::v(1) });
}
