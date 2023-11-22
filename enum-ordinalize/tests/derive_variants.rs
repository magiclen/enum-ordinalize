#![cfg(feature = "derive")]

use enum_ordinalize::Ordinalize;

#[test]
fn get_variants_const_fn() {
    #[derive(Debug, PartialEq, Eq, Ordinalize)]
    #[ordinalize(variants(pub const fn get_variants, doc = "Retrieve the array of `MyEnum`'s variants."))]
    enum MyEnum {
        A,
        B,
        C,
    }

    assert_eq!([MyEnum::A, MyEnum::B, MyEnum::C], MyEnum::get_variants());
}

#[test]
fn get_variants_const() {
    #[derive(Debug, PartialEq, Eq, Ordinalize)]
    #[ordinalize(variants(pub const VARIANTS, doc = "The array of `MyEnum`'s variants."))]
    enum MyEnum {
        A,
        B,
        C,
    }

    assert_eq!([MyEnum::A, MyEnum::B, MyEnum::C], MyEnum::VARIANTS);
}
