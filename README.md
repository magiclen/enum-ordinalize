Enum Ordinalize
====================

[![Build Status](https://travis-ci.org/magiclen/enum-ordinalize.svg?branch=master)](https://travis-ci.org/magiclen/enum-ordinalize)
[![Build status](https://ci.appveyor.com/api/projects/status/s4378vfk760xtqlf/branch/master?svg=true)](https://ci.appveyor.com/project/magiclen/enum-ordinalize/branch/master)

This crates provides `create_ordinalized_enum` macro to let enums can not only get its variants' ordinal but also be constructed from an ordinal.

## Create an Ordinalized Enum

`create_ordinalized_enum` macro can create an enum and implement a `ordinal` method, as well as `from_ordinal` and `from_ordinal_unsafe` associated functions for it.
The new enum also implements `Debug`, `PartialOrd`, `Ord`, `PartialEq`, `Clone`, `Eq`, `Hash` and `Copy` traits.

```rust
#[macro_use] extern crate enum_ordinalize;

create_ordinalized_enum!(MyEnum,
    Zero,
    One,
    Two
);

assert_eq!(2, MyEnum::Two.ordinal());
assert_eq!(Some(MyEnum::One), MyEnum::from_ordinal(1));

create_ordinalized_enum!(pub MyPublicEnum,
    A,
    B,
    C
);

assert_eq!(2, MyPublicEnum::C.ordinal());
assert_eq!(Some(MyPublicEnum::B), MyPublicEnum::from_ordinal(1));

create_ordinalized_enum!(MySpecialEnum,
    Two = 2,
    Four = 4,
    Eight = 8
);

assert_eq!(2, MySpecialEnum::Two.ordinal());
assert_eq!(Some(MySpecialEnum::Four), MySpecialEnum::from_ordinal(4));
```

## About an Ordinalized Enum

An ordinalized enum is always sized **isize** in order to directly **transmute** into an **isize** value, or conversely.

If you are 100% sure that the **isize** value can transmute into a variant of your ordinalized enum. You can use the `from_ordinal_unsafe` associated function and the **unsafe** keyword to speed up.

```rust
#[macro_use] extern crate enum_ordinalize;

create_ordinalized_enum!(MyEnum,
    Zero,
    One,
    Two
);

assert_eq!(MyEnum::One, unsafe{MyEnum::from_ordinal_unsafe(1)});
```

## Crates.io

https://crates.io/crates/enum-ordinalize

## Documentation

https://docs.rs/enum-ordinalize

## License

[MIT](LICENSE)