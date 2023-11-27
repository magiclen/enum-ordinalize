#![cfg(all(feature = "derive", feature = "traits"))]

use enum_ordinalize::Ordinalize;

#[test]
fn create_ordinalized_enum_1_1() {
    #[derive(Debug, PartialEq, Eq, Ordinalize)]
    enum MyEnum {
        Zero,
        One,
        Two,
    }

    assert_eq!(3, MyEnum::VARIANT_COUNT);
    assert_eq!([MyEnum::Zero, MyEnum::One, MyEnum::Two], MyEnum::VARIANTS);
    assert_eq!([0i8, 1i8, 2i8], MyEnum::VALUES);

    assert_eq!(0i8, MyEnum::Zero.ordinal());
    assert_eq!(1i8, MyEnum::One.ordinal());
    assert_eq!(2i8, MyEnum::Two.ordinal());

    assert_eq!(Some(MyEnum::Zero), MyEnum::from_ordinal(0i8));
    assert_eq!(Some(MyEnum::One), MyEnum::from_ordinal(1i8));
    assert_eq!(Some(MyEnum::Two), MyEnum::from_ordinal(2i8));

    assert_eq!(MyEnum::Zero, unsafe { MyEnum::from_ordinal_unsafe(0i8) });
    assert_eq!(MyEnum::One, unsafe { MyEnum::from_ordinal_unsafe(1i8) });
    assert_eq!(MyEnum::Two, unsafe { MyEnum::from_ordinal_unsafe(2i8) });
}

#[test]
fn create_ordinalized_enum_1_2() {
    #[derive(Debug, PartialEq, Eq, Ordinalize)]
    #[repr(u64)]
    enum MyEnum {
        Zero,
        One,
        Two,
    }

    assert_eq!(3, MyEnum::VARIANT_COUNT);
    assert_eq!([MyEnum::Zero, MyEnum::One, MyEnum::Two], MyEnum::VARIANTS);
    assert_eq!([0u64, 1u64, 2u64], MyEnum::VALUES);

    assert_eq!(0u64, MyEnum::Zero.ordinal());
    assert_eq!(1u64, MyEnum::One.ordinal());
    assert_eq!(2u64, MyEnum::Two.ordinal());

    assert_eq!(Some(MyEnum::Zero), MyEnum::from_ordinal(0u64));
    assert_eq!(Some(MyEnum::One), MyEnum::from_ordinal(1u64));
    assert_eq!(Some(MyEnum::Two), MyEnum::from_ordinal(2u64));

    assert_eq!(MyEnum::Zero, unsafe { MyEnum::from_ordinal_unsafe(0u64) });
    assert_eq!(MyEnum::One, unsafe { MyEnum::from_ordinal_unsafe(1u64) });
    assert_eq!(MyEnum::Two, unsafe { MyEnum::from_ordinal_unsafe(2u64) });
}

#[test]
fn create_ordinalized_enum_2() {
    #[derive(Debug, PartialEq, Eq, Ordinalize)]
    enum MyEnum {
        Two   = 2,
        Four  = 4,
        Eight = 8,
    }

    assert_eq!(3, MyEnum::VARIANT_COUNT);
    assert_eq!([MyEnum::Two, MyEnum::Four, MyEnum::Eight], MyEnum::VARIANTS);
    assert_eq!([2i8, 4i8, 8i8], MyEnum::VALUES);

    assert_eq!(2i8, MyEnum::Two.ordinal());
    assert_eq!(4i8, MyEnum::Four.ordinal());
    assert_eq!(8i8, MyEnum::Eight.ordinal());

    assert_eq!(Some(MyEnum::Two), MyEnum::from_ordinal(2i8));
    assert_eq!(Some(MyEnum::Four), MyEnum::from_ordinal(4i8));
    assert_eq!(Some(MyEnum::Eight), MyEnum::from_ordinal(8i8));

    assert_eq!(MyEnum::Two, unsafe { MyEnum::from_ordinal_unsafe(2i8) });
    assert_eq!(MyEnum::Four, unsafe { MyEnum::from_ordinal_unsafe(4i8) });
    assert_eq!(MyEnum::Eight, unsafe { MyEnum::from_ordinal_unsafe(8i8) });
}

#[test]
fn create_ordinalized_enum_3() {
    #[derive(Debug, PartialEq, Eq, Ordinalize)]
    enum MyEnum {
        Two    = 2,
        Three,
        Four,
        Ten    = 10,
        Eleven = 11,
    }

    assert_eq!(5, MyEnum::VARIANT_COUNT);
    assert_eq!(
        [MyEnum::Two, MyEnum::Three, MyEnum::Four, MyEnum::Ten, MyEnum::Eleven],
        MyEnum::VARIANTS
    );
    assert_eq!([2i8, 3i8, 4i8, 10i8, 11i8], MyEnum::VALUES);

    assert_eq!(2i8, MyEnum::Two.ordinal());
    assert_eq!(3i8, MyEnum::Three.ordinal());
    assert_eq!(4i8, MyEnum::Four.ordinal());
    assert_eq!(10i8, MyEnum::Ten.ordinal());
    assert_eq!(11i8, MyEnum::Eleven.ordinal());

    assert_eq!(Some(MyEnum::Two), MyEnum::from_ordinal(2i8));
    assert_eq!(Some(MyEnum::Three), MyEnum::from_ordinal(3i8));
    assert_eq!(Some(MyEnum::Four), MyEnum::from_ordinal(4i8));
    assert_eq!(Some(MyEnum::Ten), MyEnum::from_ordinal(10i8));
    assert_eq!(Some(MyEnum::Eleven), MyEnum::from_ordinal(11i8));

    assert_eq!(MyEnum::Two, unsafe { MyEnum::from_ordinal_unsafe(2i8) });
    assert_eq!(MyEnum::Three, unsafe { MyEnum::from_ordinal_unsafe(3i8) });
    assert_eq!(MyEnum::Four, unsafe { MyEnum::from_ordinal_unsafe(4i8) });
    assert_eq!(MyEnum::Ten, unsafe { MyEnum::from_ordinal_unsafe(10i8) });
    assert_eq!(MyEnum::Eleven, unsafe { MyEnum::from_ordinal_unsafe(11i8) });
}

#[test]
fn create_ordinalized_enum_4_1() {
    #[derive(Debug, PartialEq, Eq, Ordinalize)]
    enum MyEnum {
        Zero,
        Thousand = 1000,
        ThousandZeroOne,
    }

    assert_eq!(3, MyEnum::VARIANT_COUNT);
    assert_eq!([MyEnum::Zero, MyEnum::Thousand, MyEnum::ThousandZeroOne], MyEnum::VARIANTS);
    assert_eq!([0i16, 1000i16, 1001i16], MyEnum::VALUES);

    assert_eq!(0i16, MyEnum::Zero.ordinal());
    assert_eq!(1000i16, MyEnum::Thousand.ordinal());
    assert_eq!(1001i16, MyEnum::ThousandZeroOne.ordinal());

    assert_eq!(Some(MyEnum::Zero), MyEnum::from_ordinal(0i16));
    assert_eq!(Some(MyEnum::Thousand), MyEnum::from_ordinal(1000i16));
    assert_eq!(Some(MyEnum::ThousandZeroOne), MyEnum::from_ordinal(1001i16));

    assert_eq!(MyEnum::Zero, unsafe { MyEnum::from_ordinal_unsafe(0i16) });
    assert_eq!(MyEnum::Thousand, unsafe { MyEnum::from_ordinal_unsafe(1000i16) });
    assert_eq!(MyEnum::ThousandZeroOne, unsafe { MyEnum::from_ordinal_unsafe(1001i16) });
}

#[test]
fn create_ordinalized_enum_4_2() {
    #[derive(Debug, PartialEq, Eq, Ordinalize)]
    enum MyEnum {
        Zero,
        NegativeThousand = -1000,
        NegativeNineHundredNinetyNine,
    }

    assert_eq!(3, MyEnum::VARIANT_COUNT);
    assert_eq!(
        [MyEnum::Zero, MyEnum::NegativeThousand, MyEnum::NegativeNineHundredNinetyNine],
        MyEnum::VARIANTS
    );
    assert_eq!([0i16, -1000i16, -999i16], MyEnum::VALUES);

    assert_eq!(0i16, MyEnum::Zero.ordinal());
    assert_eq!(-1000i16, MyEnum::NegativeThousand.ordinal());
    assert_eq!(-999i16, MyEnum::NegativeNineHundredNinetyNine.ordinal());

    assert_eq!(Some(MyEnum::Zero), MyEnum::from_ordinal(0i16));
    assert_eq!(Some(MyEnum::NegativeThousand), MyEnum::from_ordinal(-1000i16));
    assert_eq!(Some(MyEnum::NegativeNineHundredNinetyNine), MyEnum::from_ordinal(-999i16));

    assert_eq!(MyEnum::Zero, unsafe { MyEnum::from_ordinal_unsafe(0i16) });
    assert_eq!(MyEnum::NegativeThousand, unsafe { MyEnum::from_ordinal_unsafe(-1000i16) });
    assert_eq!(MyEnum::NegativeNineHundredNinetyNine, unsafe {
        MyEnum::from_ordinal_unsafe(-999i16)
    });
}

#[test]
fn create_ordinalized_enum_5() {
    #[derive(Debug, PartialEq, Eq, Ordinalize)]
    enum MyEnum {
        Zero,
    }

    assert_eq!(1, MyEnum::VARIANT_COUNT);
    assert_eq!([MyEnum::Zero], MyEnum::VARIANTS);
    assert_eq!([0i8], MyEnum::VALUES);

    assert_eq!(0i8, MyEnum::Zero.ordinal());

    assert_eq!(Some(MyEnum::Zero), MyEnum::from_ordinal(0i8));

    assert_eq!(MyEnum::Zero, unsafe { MyEnum::from_ordinal_unsafe(0i8) });
}
