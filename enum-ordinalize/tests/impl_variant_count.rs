#![cfg(feature = "derive")]

use enum_ordinalize::Ordinalize;

#[test]
fn variant_count_const_fn_1() {
    #[derive(Debug, PartialEq, Eq, Ordinalize)]
    #[ordinalize(variant_count(pub const fn v, doc = "Retrieve the count of variants."))]
    enum MyEnum {
        A,
        B,
        C,
    }

    assert_eq!(3, MyEnum::v());
}

#[test]
fn variant_count_const_fn_2() {
    #[derive(Debug, PartialEq, Eq, Ordinalize)]
    #[ordinalize(variant_count(pub fn v, doc = "Retrieve the count of variants."))]
    enum MyEnum {
        A,
        B,
        C,
    }

    assert_eq!(3, MyEnum::v());
}

#[test]
fn variant_count_const_1() {
    #[derive(Debug, PartialEq, Eq, Ordinalize)]
    #[ordinalize(variant_count(pub const V, doc = "The count of variants."))]
    enum MyEnum {
        A,
        B,
        C,
    }

    assert_eq!(3, MyEnum::V);
}

#[test]
fn variant_count_const_2() {
    #[derive(Debug, PartialEq, Eq, Ordinalize)]
    #[ordinalize(variant_count(pub V, doc = "The count of variants."))]
    enum MyEnum {
        A,
        B,
        C,
    }

    assert_eq!(3, MyEnum::V);
}
