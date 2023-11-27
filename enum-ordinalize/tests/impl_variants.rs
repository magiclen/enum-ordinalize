#![cfg(feature = "derive")]

use enum_ordinalize::Ordinalize;

#[test]
fn variants_const_fn_1() {
    #[derive(Debug, PartialEq, Eq, Ordinalize)]
    #[ordinalize(variants(pub const fn v, doc = "Retrieve the array of `MyEnum`'s variants."))]
    enum MyEnum {
        A,
        B,
        C,
    }

    assert_eq!([MyEnum::A, MyEnum::B, MyEnum::C], MyEnum::v());
}

#[test]
fn variants_const_fn_2() {
    #[derive(Debug, PartialEq, Eq, Ordinalize)]
    #[ordinalize(variants(pub fn v, doc = "Retrieve the array of `MyEnum`'s variants."))]
    enum MyEnum {
        A,
        B,
        C,
    }

    assert_eq!([MyEnum::A, MyEnum::B, MyEnum::C], MyEnum::v());
}

#[test]
fn variants_const_1() {
    #[derive(Debug, PartialEq, Eq, Ordinalize)]
    #[ordinalize(variants(pub const V, doc = "The array of `MyEnum`'s variants."))]
    enum MyEnum {
        A,
        B,
        C,
    }

    assert_eq!([MyEnum::A, MyEnum::B, MyEnum::C], MyEnum::V);
}

#[test]
fn variants_const_2() {
    #[derive(Debug, PartialEq, Eq, Ordinalize)]
    #[ordinalize(variants(pub V, doc = "The array of `MyEnum`'s variants."))]
    enum MyEnum {
        A,
        B,
        C,
    }

    assert_eq!([MyEnum::A, MyEnum::B, MyEnum::C], MyEnum::V);
}
