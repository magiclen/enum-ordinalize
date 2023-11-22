#![cfg(all(feature = "derive", feature = "variants"))]

use enum_ordinalize::Variants;
#[test]
fn get_variants_trait_1() {
    #[derive(Debug, PartialEq, Eq, Variants)]
    enum MyEnum {
        A,
        B,
        C,
    }

    assert_eq!([MyEnum::A, MyEnum::B, MyEnum::C], MyEnum::VARIANTS);
}

#[test]
fn get_variants_trait_2() {
    #[derive(Debug, PartialEq, Eq, Variants)]
    enum MyEnum {}

    assert_eq!([] as [MyEnum; 0], MyEnum::VARIANTS);
}
