#![cfg(feature = "derive")]

use enum_ordinalize::Ordinalize;

#[test]
fn create_ordinalized_enum_1_3() {
    #[allow(dead_code)]
    #[derive(Debug, PartialEq, Eq, Ordinalize)]
    #[ordinalize(impl_trait = false)]
    #[repr(u8)]
    enum MyEnum {
        A,
        B,
    }

    #[cfg(feature = "traits")]
    impl Ordinalize for MyEnum {
        type VariantType = u8;

        const VALUES: &'static [Self::VariantType] = &[0, 1];
        const VARIANTS: &'static [Self] = &[MyEnum::A, MyEnum::B];
        const VARIANT_COUNT: usize = 2;

        #[inline]
        unsafe fn from_ordinal_unsafe(number: Self::VariantType) -> Self {
            unsafe { ::core::mem::transmute(number) }
        }

        #[inline]
        fn from_ordinal(number: Self::VariantType) -> Option<Self> {
            match number {
                0 => Some(Self::A),
                1 => Some(Self::B),
                _ => None,
            }
        }

        #[inline]
        fn ordinal(&self) -> Self::VariantType {
            match self {
                Self::A => 0,
                Self::B => 1,
            }
        }
    }
}

#[test]
fn create_ordinalized_enum_1_4() {
    #[allow(dead_code)]
    #[derive(Debug, PartialEq, Eq, Ordinalize)]
    #[repr(u8)]
    #[ordinalize(impl_trait = false)]
    #[ordinalize(variant_count(pub const VARIANT_COUNT))]
    enum MyEnum {
        A,
        B,
    }

    assert_eq!(2, MyEnum::VARIANT_COUNT);
}
