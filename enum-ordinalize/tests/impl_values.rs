#![cfg(feature = "derive")]

use enum_ordinalize::Ordinalize;

#[test]
fn variants_const_fn_1() {
    #[derive(Debug, PartialEq, Eq, Ordinalize)]
    #[ordinalize(values(pub const fn v, doc = "Retrieve the array of `MyEnum`'s values."))]
    enum MyEnum {
        A,
        B,
        C,
    }

    assert_eq!([0i8, 1i8, 2i8], MyEnum::v());
}

#[test]
fn variants_const_fn_2() {
    #[derive(Debug, PartialEq, Eq, Ordinalize)]
    #[ordinalize(values(pub fn v, doc = "Retrieve the array of `MyEnum`'s values."))]
    enum MyEnum {
        A,
        B,
        C,
    }

    assert_eq!([0i8, 1i8, 2i8], MyEnum::v());
}

#[test]
fn variants_const_1() {
    #[derive(Debug, PartialEq, Eq, Ordinalize)]
    #[ordinalize(values(pub const V, doc = "The array of `MyEnum`'s values."))]
    enum MyEnum {
        A,
        B,
        C,
    }

    assert_eq!([0i8, 1i8, 2i8], MyEnum::V);
}

#[test]
fn variants_const_2() {
    #[derive(Debug, PartialEq, Eq, Ordinalize)]
    #[ordinalize(values(pub V, doc = "The array of `MyEnum`'s values."))]
    enum MyEnum {
        A,
        B,
        C,
    }

    assert_eq!([0i8, 1i8, 2i8], MyEnum::V);
}
