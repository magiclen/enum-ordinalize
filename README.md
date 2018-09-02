Enum Ordinalize
====================

[![Build Status](https://travis-ci.org/magiclen/enum-ordinalize.svg?branch=master)](https://travis-ci.org/magiclen/enum-ordinalize)

## Make an Enum Ordinalized

`ordinalize_enum` macro can implement a `ordinal` method and a `from_ordinal` associated function for an existing `enum`.

```rust
#[macro_use] extern crate enum_ordinalize;

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
```

## Create an Ordinalized Enum

`create_ordinalized_enum` macro can create an enum and implement a `ordinal` method and a `from_ordinal` associated function for it.
The new enum also implements `Debug`, `PartialEq`, and `Clone` traits.

```rust
#[macro_use] extern crate enum_ordinalize;

create_ordinalized_enum!(MyEnum,
    u8,
    Zero,
    One,
    Two
);

assert_eq!(2, MyEnum::Two.ordinal());
assert_eq!(Some(MyEnum::One), MyEnum::from_ordinal(1));

create_ordinalized_enum!(pub MyPublicEnum,
    u8,
    A,
    B,
    C
);

assert_eq!(2, MyPublicEnum::C.ordinal());
assert_eq!(Some(MyPublicEnum::B), MyPublicEnum::from_ordinal(1));

create_ordinalized_enum!(MySpecialEnum,
    u8,
    Two = 2,
    Four = 4,
    Eight = 8
);

assert_eq!(2, MySpecialEnum::Two.ordinal());
assert_eq!(Some(MySpecialEnum::Four), MySpecialEnum::from_ordinal(4));
```

## Crates.io

https://crates.io/crates/enum-ordinalize

## Documentation

https://docs.rs/enum-ordinalize

## License

[MIT](LICENSE)