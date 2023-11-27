#![cfg(feature = "derive")]

use enum_ordinalize::Ordinalize;

#[test]
fn from_ordinal_1() {
    #[derive(Debug, PartialEq, Eq, Ordinalize)]
    #[ordinalize(from_ordinal(pub const fn v, doc = "Obtain a variant based on an integer number."))]
    enum MyEnum {
        A,
        B,
        C,
    }

    assert_eq!(Some(MyEnum::B), MyEnum::v(1));
}

#[test]
fn from_ordinal_2() {
    #[derive(Debug, PartialEq, Eq, Ordinalize)]
    #[ordinalize(from_ordinal(pub fn v, doc = "Obtain a variant based on an integer number."))]
    enum MyEnum {
        A,
        B,
        C,
    }

    assert_eq!(Some(MyEnum::B), MyEnum::v(1));
}
