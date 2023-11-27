#![cfg(all(feature = "derive", feature = "traits"))]

use enum_ordinalize::Ordinalize;

const FOUR: i8 = 4;

const fn eight() -> i8 {
    8
}

const TEN: i16 = 10;

#[test]
fn create_ordinalized_enum_5_1() {
    #[derive(Debug, PartialEq, Eq, Ordinalize)]
    #[repr(i8)]
    enum MyEnum {
        Two   = 1 + 1,
        Four  = FOUR,
        Eight = eight(),
        Ten   = TEN as i8,
    }

    assert_eq!(4, MyEnum::VARIANT_COUNT);
    assert_eq!([MyEnum::Two, MyEnum::Four, MyEnum::Eight, MyEnum::Ten], MyEnum::VARIANTS);
    assert_eq!([2i8, 4i8, 8i8, 10i8], MyEnum::VALUES);

    assert_eq!(2i8, MyEnum::Two.ordinal());
    assert_eq!(4i8, MyEnum::Four.ordinal());
    assert_eq!(8i8, MyEnum::Eight.ordinal());
    assert_eq!(10i8, MyEnum::Ten.ordinal());

    assert_eq!(Some(MyEnum::Two), MyEnum::from_ordinal(2i8));
    assert_eq!(Some(MyEnum::Four), MyEnum::from_ordinal(4i8));
    assert_eq!(Some(MyEnum::Eight), MyEnum::from_ordinal(8i8));
    assert_eq!(Some(MyEnum::Ten), MyEnum::from_ordinal(10i8));

    assert_eq!(MyEnum::Two, unsafe { MyEnum::from_ordinal_unsafe(2i8) });
    assert_eq!(MyEnum::Four, unsafe { MyEnum::from_ordinal_unsafe(4i8) });
    assert_eq!(MyEnum::Eight, unsafe { MyEnum::from_ordinal_unsafe(8i8) });
    assert_eq!(MyEnum::Ten, unsafe { MyEnum::from_ordinal_unsafe(10i8) });
}

#[test]
fn create_ordinalized_enum_5_2() {
    #[derive(Debug, PartialEq, Eq, Ordinalize)]
    #[repr(i8)]
    enum MyEnum {
        Zero,
        One,
        Two     = 1 + 1,
        Three,
        Four,
        Six     = FOUR + 2,
        Seven,
        Eight,
        Ten     = TEN as i8,
        Eleven,
        Twelve,
        Hundred = 100,
        HundredOne,
        HundredTwo,
    }

    assert_eq!(14, MyEnum::VARIANT_COUNT);
    assert_eq!(
        [
            MyEnum::Zero,
            MyEnum::One,
            MyEnum::Two,
            MyEnum::Three,
            MyEnum::Four,
            MyEnum::Six,
            MyEnum::Seven,
            MyEnum::Eight,
            MyEnum::Ten,
            MyEnum::Eleven,
            MyEnum::Twelve,
            MyEnum::Hundred,
            MyEnum::HundredOne,
            MyEnum::HundredTwo
        ],
        MyEnum::VARIANTS
    );
    assert_eq!(
        [0i8, 1i8, 2i8, 3i8, 4i8, 6i8, 7i8, 8i8, 10i8, 11i8, 12i8, 100i8, 101i8, 102i8],
        MyEnum::VALUES
    );

    assert_eq!(0i8, MyEnum::Zero.ordinal());
    assert_eq!(1i8, MyEnum::One.ordinal());
    assert_eq!(4i8, MyEnum::Four.ordinal());
    assert_eq!(8i8, MyEnum::Eight.ordinal());
    assert_eq!(12i8, MyEnum::Twelve.ordinal());
    assert_eq!(102i8, MyEnum::HundredTwo.ordinal());

    assert_eq!(Some(MyEnum::Zero), MyEnum::from_ordinal(0i8));
    assert_eq!(Some(MyEnum::One), MyEnum::from_ordinal(1i8));
    assert_eq!(Some(MyEnum::Four), MyEnum::from_ordinal(4i8));
    assert_eq!(Some(MyEnum::Eight), MyEnum::from_ordinal(8i8));
    assert_eq!(Some(MyEnum::Twelve), MyEnum::from_ordinal(12i8));
    assert_eq!(Some(MyEnum::HundredTwo), MyEnum::from_ordinal(102i8));

    assert_eq!(MyEnum::Zero, unsafe { MyEnum::from_ordinal_unsafe(0i8) });
    assert_eq!(MyEnum::One, unsafe { MyEnum::from_ordinal_unsafe(1i8) });
    assert_eq!(MyEnum::Four, unsafe { MyEnum::from_ordinal_unsafe(4i8) });
    assert_eq!(MyEnum::Eight, unsafe { MyEnum::from_ordinal_unsafe(8i8) });
    assert_eq!(MyEnum::Twelve, unsafe { MyEnum::from_ordinal_unsafe(12i8) });
    assert_eq!(MyEnum::HundredTwo, unsafe { MyEnum::from_ordinal_unsafe(102i8) });
}
