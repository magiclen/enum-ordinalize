#![cfg(feature = "derive")]

use enum_ordinalize::Ordinalize;

#[test]
fn ordinal_1() {
    #[derive(Debug, PartialEq, Eq, Ordinalize)]
    #[ordinalize(ordinal(pub const fn v, doc = "Retrieve the integer number of this variant."))]
    enum MyEnum {
        A,
        B,
        C,
    }

    assert_eq!(1, MyEnum::B.v());
}

#[test]
fn ordinal_2() {
    #[derive(Debug, PartialEq, Eq, Ordinalize)]
    #[ordinalize(ordinal(pub fn v, doc = "Retrieve the integer number of this variant."))]
    enum MyEnum {
        A,
        B,
        C,
    }

    assert_eq!(1, MyEnum::B.v());
}
