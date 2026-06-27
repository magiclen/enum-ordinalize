#![cfg(all(feature = "derive", feature = "traits"))]

#[test]
fn derive_with_path_without_import() {
    #[derive(Debug, PartialEq, Eq, enum_ordinalize::Ordinalize)]
    enum MyEnum {
        A,
        B,
    }

    assert_eq!(2, <MyEnum as enum_ordinalize::Ordinalize>::VARIANT_COUNT);
    assert_eq!([MyEnum::A, MyEnum::B], <MyEnum as enum_ordinalize::Ordinalize>::VARIANTS);
    assert_eq!([0i8, 1i8], <MyEnum as enum_ordinalize::Ordinalize>::VALUES);
    assert_eq!(Some(MyEnum::A), <MyEnum as enum_ordinalize::Ordinalize>::from_ordinal(0i8));
}
