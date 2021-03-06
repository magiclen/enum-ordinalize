#![cfg(feature = "nightly-test")]
#![feature(repr128)]

#[macro_use]
extern crate enum_ordinalize;

#[test]
fn create_ordinalized_enum_4_3() {
    #[derive(Debug, PartialEq, Eq, Ordinalize)]
    #[repr(i128)]
    enum MyEnum {
        A = 2i128,
        B = 4,
        C = 75557863725914323419136,
    }

    assert_eq!(2i128, MyEnum::A.ordinal());
    assert_eq!(4i128, MyEnum::B.ordinal());
    assert_eq!(75557863725914323419136i128, MyEnum::C.ordinal());

    // TODO: Comment out the following cases because they would cause an internal compiler error (1.45.0-nightly, should be a compiler bug)
    // assert_eq!(Some(MyEnum::A), MyEnum::from_ordinal(2i128));
    // assert_eq!(Some(MyEnum::B), MyEnum::from_ordinal(4i128));
    // assert_eq!(Some(MyEnum::C), MyEnum::from_ordinal(75557863725914323419136i128));

    assert_eq!(MyEnum::A, unsafe { MyEnum::from_ordinal_unsafe(2i128) });
    assert_eq!(MyEnum::B, unsafe { MyEnum::from_ordinal_unsafe(4i128) });
    assert_eq!(MyEnum::C, unsafe { MyEnum::from_ordinal_unsafe(75557863725914323419136i128) });
}
