#![cfg(all(feature = "derive", feature = "variants"))]

use enum_ordinalize::{Ordinalize, Variants};

#[test]
fn get_variants_trait_1() {
    #[derive(Debug, PartialEq, Eq, Variants)]
    enum MyEnum {
        A,
        B,
        C,
    }

    assert_eq!([MyEnum::A, MyEnum::B, MyEnum::C], MyEnum::VARIANTS);
    assert_eq!(3usize, MyEnum::COUNT);
}

#[test]
fn get_variants_trait_2() {
    #[derive(Debug, PartialEq, Eq, Ordinalize, Variants)]
    enum MyEnum {
        A,
        B,
        C,
    }

    assert_eq!([MyEnum::A, MyEnum::B, MyEnum::C], MyEnum::VARIANTS);
    assert_eq!(3usize, MyEnum::COUNT);
}

#[test]
fn get_variants_trait_3() {
    #[derive(Debug, PartialEq, Eq, Variants)]
    enum MyEnum {}

    assert_eq!([] as [MyEnum; 0], MyEnum::VARIANTS);
    assert_eq!(0usize, MyEnum::COUNT);
}
