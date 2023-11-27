#![cfg(all(feature = "derive", feature = "traits", feature = "nightly-test"))]
#![allow(incomplete_features)]
#![feature(repr128)]

use enum_ordinalize::Ordinalize;

#[test]
fn create_ordinalized_enum_4_3() {
    #[derive(Debug, PartialEq, Eq, Ordinalize)]
    #[repr(i128)]
    enum MyEnum {
        A = 2i128,
        B = 4,
        C = 75557863725914323419136,
    }

    assert_eq!(3, MyEnum::VARIANT_COUNT);
    assert_eq!([MyEnum::A, MyEnum::B, MyEnum::C], MyEnum::VARIANTS);
    assert_eq!([2i128, 4i128, 75557863725914323419136i128], MyEnum::VALUES);

    assert_eq!(2i128, MyEnum::A.ordinal());
    assert_eq!(4i128, MyEnum::B.ordinal());
    assert_eq!(75557863725914323419136i128, MyEnum::C.ordinal());

    assert_eq!(Some(MyEnum::A), MyEnum::from_ordinal(2i128));
    assert_eq!(Some(MyEnum::B), MyEnum::from_ordinal(4i128));
    assert_eq!(Some(MyEnum::C), MyEnum::from_ordinal(75557863725914323419136i128));

    assert_eq!(MyEnum::A, unsafe { MyEnum::from_ordinal_unsafe(2i128) });
    assert_eq!(MyEnum::B, unsafe { MyEnum::from_ordinal_unsafe(4i128) });
    assert_eq!(MyEnum::C, unsafe { MyEnum::from_ordinal_unsafe(75557863725914323419136i128) });
}
