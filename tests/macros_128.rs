#![cfg(feature = "nightly-test")]
#![feature(repr128)]

#[macro_use]
extern crate enum_ordinalize;

#[test]
fn create_ordinalized_enum_1_5() {
    create_ordinalized_enum!(MyEnum: u128, Zero, One, Two);

    assert_eq!(0u128, MyEnum::Zero.ordinal());
    assert_eq!(1u128, MyEnum::One.ordinal());
    assert_eq!(2u128, MyEnum::Two.ordinal());

    assert_eq!(Some(MyEnum::Zero), MyEnum::from_ordinal(0u128));
    assert_eq!(Some(MyEnum::One), MyEnum::from_ordinal(1u128));
    assert_eq!(Some(MyEnum::Two), MyEnum::from_ordinal(2u128));

    assert_eq!(MyEnum::Zero, unsafe { MyEnum::from_ordinal_unsafe(0u128) });
    assert_eq!(MyEnum::One, unsafe { MyEnum::from_ordinal_unsafe(1u128) });
    assert_eq!(MyEnum::Two, unsafe { MyEnum::from_ordinal_unsafe(2u128) });
}

#[test]
fn create_ordinalized_enum_1_6() {
    create_ordinalized_enum!(pub MyEnum: u128,
        Zero,
        One,
        Two
    );

    assert_eq!(0u128, MyEnum::Zero.ordinal());
    assert_eq!(1u128, MyEnum::One.ordinal());
    assert_eq!(2u128, MyEnum::Two.ordinal());

    assert_eq!(Some(MyEnum::Zero), MyEnum::from_ordinal(0u128));
    assert_eq!(Some(MyEnum::One), MyEnum::from_ordinal(1u128));
    assert_eq!(Some(MyEnum::Two), MyEnum::from_ordinal(2u128));

    assert_eq!(MyEnum::Zero, unsafe { MyEnum::from_ordinal_unsafe(0u128) });
    assert_eq!(MyEnum::One, unsafe { MyEnum::from_ordinal_unsafe(1u128) });
    assert_eq!(MyEnum::Two, unsafe { MyEnum::from_ordinal_unsafe(2u128) });
}

#[test]
fn create_ordinalized_enum_2_5() {
    create_ordinalized_enum!(MyEnum: u128, Two = 2, Four = 4, Eight = 8);

    assert_eq!(2u128, MyEnum::Two.ordinal());
    assert_eq!(4u128, MyEnum::Four.ordinal());
    assert_eq!(8u128, MyEnum::Eight.ordinal());

    assert_eq!(Some(MyEnum::Two), MyEnum::from_ordinal(2u128));
    assert_eq!(Some(MyEnum::Four), MyEnum::from_ordinal(4u128));
    assert_eq!(Some(MyEnum::Eight), MyEnum::from_ordinal(8u128));

    assert_eq!(MyEnum::Two, unsafe { MyEnum::from_ordinal_unsafe(2u128) });
    assert_eq!(MyEnum::Four, unsafe { MyEnum::from_ordinal_unsafe(4u128) });
    assert_eq!(MyEnum::Eight, unsafe { MyEnum::from_ordinal_unsafe(8u128) });
}

#[test]
fn create_ordinalized_enum_2_6() {
    create_ordinalized_enum!(pub MyEnum: u128,
        Two = 2,
        Four = 4,
        Eight = 8
    );

    assert_eq!(2u128, MyEnum::Two.ordinal());
    assert_eq!(4u128, MyEnum::Four.ordinal());
    assert_eq!(8u128, MyEnum::Eight.ordinal());

    assert_eq!(Some(MyEnum::Two), MyEnum::from_ordinal(2u128));
    assert_eq!(Some(MyEnum::Four), MyEnum::from_ordinal(4u128));
    assert_eq!(Some(MyEnum::Eight), MyEnum::from_ordinal(8u128));

    assert_eq!(MyEnum::Two, unsafe { MyEnum::from_ordinal_unsafe(2u128) });
    assert_eq!(MyEnum::Four, unsafe { MyEnum::from_ordinal_unsafe(4u128) });
    assert_eq!(MyEnum::Eight, unsafe { MyEnum::from_ordinal_unsafe(8u128) });
}
