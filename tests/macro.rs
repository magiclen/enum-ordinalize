#[macro_use]
extern crate enum_ordinalize;

#[test]
fn test_ordinalize_enum() {
    #[derive(Debug, PartialEq)]
    enum MyEnum {
        Zero,
        One,
        Two,
    }

    ordinalize_enum!(MyEnum,
        u8,
        Zero,
        One,
        Two
    );

    assert_eq!(2, MyEnum::Two.ordinal());
    assert_eq!(Some(MyEnum::One), MyEnum::from_ordinal(1));
}

#[test]
fn test_create_ordinalized_enum_1_1() {
    create_ordinalized_enum!(MyEnum,
        u8,
        Zero,
        One,
        Two
    );

    assert_eq!(2, MyEnum::Two.ordinal());
    assert_eq!(Some(MyEnum::One), MyEnum::from_ordinal(1));
}

#[test]
fn test_create_ordinalized_enum_1_2() {
    create_ordinalized_enum!(pub MyEnum,
        u8,
        Zero,
        One,
        Two
    );

    assert_eq!(2, MyEnum::Two.ordinal());
    assert_eq!(Some(MyEnum::One), MyEnum::from_ordinal(1));
}

#[test]
fn test_create_ordinalized_enum_2_1() {
    create_ordinalized_enum!(MyEnum,
        u8,
        Two = 2,
        Four = 4,
        Eight = 8
    );

    assert_eq!(2, MyEnum::Two.ordinal());
    assert_eq!(Some(MyEnum::Four), MyEnum::from_ordinal(4));
}

#[test]
fn test_create_ordinalized_enum_2_2() {
    create_ordinalized_enum!(pub MyEnum,
        u8,
        Two = 2,
        Four = 4,
        Eight = 8
    );

    assert_eq!(2, MyEnum::Two.ordinal());
    assert_eq!(Some(MyEnum::Four), MyEnum::from_ordinal(4));
}