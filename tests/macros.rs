#[macro_use]
extern crate enum_ordinalize;

#[test]
fn create_ordinalized_enum_1_1() {
    create_ordinalized_enum!(MyEnum,
        Zero,
        One,
        Two
    );

    assert_eq!(0isize, MyEnum::Zero.ordinal());
    assert_eq!(1isize, MyEnum::One.ordinal());
    assert_eq!(2isize, MyEnum::Two.ordinal());

    assert_eq!(Some(MyEnum::Zero), MyEnum::from_ordinal(0isize));
    assert_eq!(Some(MyEnum::One), MyEnum::from_ordinal(1isize));
    assert_eq!(Some(MyEnum::Two), MyEnum::from_ordinal(2isize));

    assert_eq!(MyEnum::Zero, unsafe { MyEnum::from_ordinal_unsafe(0isize) });
    assert_eq!(MyEnum::One, unsafe { MyEnum::from_ordinal_unsafe(1isize) });
    assert_eq!(MyEnum::Two, unsafe { MyEnum::from_ordinal_unsafe(2isize) });
}

#[test]
fn create_ordinalized_enum_1_2() {
    create_ordinalized_enum!(pub MyEnum,
        Zero,
        One,
        Two
    );

    assert_eq!(0isize, MyEnum::Zero.ordinal());
    assert_eq!(1isize, MyEnum::One.ordinal());
    assert_eq!(2isize, MyEnum::Two.ordinal());

    assert_eq!(Some(MyEnum::Zero), MyEnum::from_ordinal(0isize));
    assert_eq!(Some(MyEnum::One), MyEnum::from_ordinal(1isize));
    assert_eq!(Some(MyEnum::Two), MyEnum::from_ordinal(2isize));

    assert_eq!(MyEnum::Zero, unsafe { MyEnum::from_ordinal_unsafe(0isize) });
    assert_eq!(MyEnum::One, unsafe { MyEnum::from_ordinal_unsafe(1isize) });
    assert_eq!(MyEnum::Two, unsafe { MyEnum::from_ordinal_unsafe(2isize) });
}

#[test]
fn create_ordinalized_enum_1_3() {
    create_ordinalized_enum!(MyEnum: u8,
        Zero,
        One,
        Two
    );

    assert_eq!(0u8, MyEnum::Zero.ordinal());
    assert_eq!(1u8, MyEnum::One.ordinal());
    assert_eq!(2u8, MyEnum::Two.ordinal());

    assert_eq!(Some(MyEnum::Zero), MyEnum::from_ordinal(0u8));
    assert_eq!(Some(MyEnum::One), MyEnum::from_ordinal(1u8));
    assert_eq!(Some(MyEnum::Two), MyEnum::from_ordinal(2u8));

    assert_eq!(MyEnum::Zero, unsafe { MyEnum::from_ordinal_unsafe(0u8) });
    assert_eq!(MyEnum::One, unsafe { MyEnum::from_ordinal_unsafe(1u8) });
    assert_eq!(MyEnum::Two, unsafe { MyEnum::from_ordinal_unsafe(2u8) });
}

#[test]
fn create_ordinalized_enum_1_4() {
    create_ordinalized_enum!(pub MyEnum: u8,
        Zero,
        One,
        Two
    );

    assert_eq!(0u8, MyEnum::Zero.ordinal());
    assert_eq!(1u8, MyEnum::One.ordinal());
    assert_eq!(2u8, MyEnum::Two.ordinal());

    assert_eq!(Some(MyEnum::Zero), MyEnum::from_ordinal(0u8));
    assert_eq!(Some(MyEnum::One), MyEnum::from_ordinal(1u8));
    assert_eq!(Some(MyEnum::Two), MyEnum::from_ordinal(2u8));

    assert_eq!(MyEnum::Zero, unsafe { MyEnum::from_ordinal_unsafe(0u8) });
    assert_eq!(MyEnum::One, unsafe { MyEnum::from_ordinal_unsafe(1u8) });
    assert_eq!(MyEnum::Two, unsafe { MyEnum::from_ordinal_unsafe(2u8) });
}

#[test]
fn create_ordinalized_enum_2_1() {
    create_ordinalized_enum!(MyEnum,
        Two = 2,
        Four = 4,
        Eight = 8
    );

    assert_eq!(2isize, MyEnum::Two.ordinal());
    assert_eq!(4isize, MyEnum::Four.ordinal());
    assert_eq!(8isize, MyEnum::Eight.ordinal());

    assert_eq!(Some(MyEnum::Two), MyEnum::from_ordinal(2isize));
    assert_eq!(Some(MyEnum::Four), MyEnum::from_ordinal(4isize));
    assert_eq!(Some(MyEnum::Eight), MyEnum::from_ordinal(8isize));

    assert_eq!(MyEnum::Two, unsafe { MyEnum::from_ordinal_unsafe(2isize) });
    assert_eq!(MyEnum::Four, unsafe { MyEnum::from_ordinal_unsafe(4isize) });
    assert_eq!(MyEnum::Eight, unsafe { MyEnum::from_ordinal_unsafe(8isize) });
}

#[test]
fn create_ordinalized_enum_2_2() {
    create_ordinalized_enum!(pub MyEnum,
        Two = 2,
        Four = 4,
        Eight = 8
    );

    assert_eq!(2isize, MyEnum::Two.ordinal());
    assert_eq!(4isize, MyEnum::Four.ordinal());
    assert_eq!(8isize, MyEnum::Eight.ordinal());

    assert_eq!(Some(MyEnum::Two), MyEnum::from_ordinal(2isize));
    assert_eq!(Some(MyEnum::Four), MyEnum::from_ordinal(4isize));
    assert_eq!(Some(MyEnum::Eight), MyEnum::from_ordinal(8isize));

    assert_eq!(MyEnum::Two, unsafe { MyEnum::from_ordinal_unsafe(2isize) });
    assert_eq!(MyEnum::Four, unsafe { MyEnum::from_ordinal_unsafe(4isize) });
    assert_eq!(MyEnum::Eight, unsafe { MyEnum::from_ordinal_unsafe(8isize) });
}

#[test]
fn create_ordinalized_enum_2_3() {
    create_ordinalized_enum!(MyEnum: u8,
        Two = 2,
        Four = 4,
        Eight = 8
    );

    assert_eq!(2u8, MyEnum::Two.ordinal());
    assert_eq!(4u8, MyEnum::Four.ordinal());
    assert_eq!(8u8, MyEnum::Eight.ordinal());

    assert_eq!(Some(MyEnum::Two), MyEnum::from_ordinal(2u8));
    assert_eq!(Some(MyEnum::Four), MyEnum::from_ordinal(4u8));
    assert_eq!(Some(MyEnum::Eight), MyEnum::from_ordinal(8u8));

    assert_eq!(MyEnum::Two, unsafe { MyEnum::from_ordinal_unsafe(2u8) });
    assert_eq!(MyEnum::Four, unsafe { MyEnum::from_ordinal_unsafe(4u8) });
    assert_eq!(MyEnum::Eight, unsafe { MyEnum::from_ordinal_unsafe(8u8) });
}

#[test]
fn create_ordinalized_enum_2_4() {
    create_ordinalized_enum!(pub MyEnum: u8,
        Two = 2,
        Four = 4,
        Eight = 8
    );

    assert_eq!(2u8, MyEnum::Two.ordinal());
    assert_eq!(4u8, MyEnum::Four.ordinal());
    assert_eq!(8u8, MyEnum::Eight.ordinal());

    assert_eq!(Some(MyEnum::Two), MyEnum::from_ordinal(2u8));
    assert_eq!(Some(MyEnum::Four), MyEnum::from_ordinal(4u8));
    assert_eq!(Some(MyEnum::Eight), MyEnum::from_ordinal(8u8));

    assert_eq!(MyEnum::Two, unsafe { MyEnum::from_ordinal_unsafe(2u8) });
    assert_eq!(MyEnum::Four, unsafe { MyEnum::from_ordinal_unsafe(4u8) });
    assert_eq!(MyEnum::Eight, unsafe { MyEnum::from_ordinal_unsafe(8u8) });
}