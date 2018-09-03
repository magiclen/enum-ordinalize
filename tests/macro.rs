#[macro_use]
extern crate enum_ordinalize;

#[test]
fn test_create_ordinalized_enum_1_1() {
    create_ordinalized_enum!(MyEnum,
        Zero,
        One,
        Two
    );

    assert_eq!(0, MyEnum::Zero.ordinal());
    assert_eq!(1, MyEnum::One.ordinal());
    assert_eq!(2, MyEnum::Two.ordinal());

    assert_eq!(Some(MyEnum::Zero), MyEnum::from_ordinal(0));
    assert_eq!(Some(MyEnum::One), MyEnum::from_ordinal(1));
    assert_eq!(Some(MyEnum::Two), MyEnum::from_ordinal(2));

    assert_eq!(MyEnum::Zero, unsafe { MyEnum::from_ordinal_unsafe(0) });
    assert_eq!(MyEnum::One, unsafe { MyEnum::from_ordinal_unsafe(1) });
    assert_eq!(MyEnum::Two, unsafe { MyEnum::from_ordinal_unsafe(2) });
}

#[test]
fn test_create_ordinalized_enum_1_2() {
    create_ordinalized_enum!(pub MyEnum,
        Zero,
        One,
        Two
    );

    assert_eq!(0, MyEnum::Zero.ordinal());
    assert_eq!(1, MyEnum::One.ordinal());
    assert_eq!(2, MyEnum::Two.ordinal());

    assert_eq!(Some(MyEnum::Zero), MyEnum::from_ordinal(0));
    assert_eq!(Some(MyEnum::One), MyEnum::from_ordinal(1));
    assert_eq!(Some(MyEnum::Two), MyEnum::from_ordinal(2));

    assert_eq!(MyEnum::Zero, unsafe { MyEnum::from_ordinal_unsafe(0) });
    assert_eq!(MyEnum::One, unsafe { MyEnum::from_ordinal_unsafe(1) });
    assert_eq!(MyEnum::Two, unsafe { MyEnum::from_ordinal_unsafe(2) });
}

#[test]
fn test_create_ordinalized_enum_2_1() {
    create_ordinalized_enum!(MyEnum,
        Two = 2,
        Four = 4,
        Eight = 8
    );

    assert_eq!(2, MyEnum::Two.ordinal());
    assert_eq!(4, MyEnum::Four.ordinal());
    assert_eq!(8, MyEnum::Eight.ordinal());

    assert_eq!(Some(MyEnum::Two), MyEnum::from_ordinal(2));
    assert_eq!(Some(MyEnum::Four), MyEnum::from_ordinal(4));
    assert_eq!(Some(MyEnum::Eight), MyEnum::from_ordinal(8));

    assert_eq!(MyEnum::Two, unsafe { MyEnum::from_ordinal_unsafe(2) });
    assert_eq!(MyEnum::Four, unsafe { MyEnum::from_ordinal_unsafe(4) });
    assert_eq!(MyEnum::Eight, unsafe { MyEnum::from_ordinal_unsafe(8) });
}

#[test]
fn test_create_ordinalized_enum_2_2() {
    create_ordinalized_enum!(pub MyEnum,
        Two = 2,
        Four = 4,
        Eight = 8
    );

    assert_eq!(2, MyEnum::Two.ordinal());
    assert_eq!(4, MyEnum::Four.ordinal());
    assert_eq!(8, MyEnum::Eight.ordinal());

    assert_eq!(Some(MyEnum::Two), MyEnum::from_ordinal(2));
    assert_eq!(Some(MyEnum::Four), MyEnum::from_ordinal(4));
    assert_eq!(Some(MyEnum::Eight), MyEnum::from_ordinal(8));

    assert_eq!(MyEnum::Two, unsafe { MyEnum::from_ordinal_unsafe(2) });
    assert_eq!(MyEnum::Four, unsafe { MyEnum::from_ordinal_unsafe(4) });
    assert_eq!(MyEnum::Eight, unsafe { MyEnum::from_ordinal_unsafe(8) });
}