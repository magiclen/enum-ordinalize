/*!
# Enum Ordinalize

This library enables enums to not only obtain the ordinal values of their variants but also allows for the construction of enums from an ordinal value.

## Usage

Use `#[derive(Ordinalize)]` to have an enum (which must only has unit variants) implement the `Ordinalize` trait.

```rust
# #[cfg(feature = "derive")]
# {
use enum_ordinalize::Ordinalize;

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
# }
```

#### The (Ordinal) Size of an Enum

The ordinal value is an integer whose size is determined by the enum itself. The size of the enum increases with the magnitude of the variants' values, whether larger (or smaller if negative).

For example,

```rust
# #[cfg(feature = "derive")]
# {
use enum_ordinalize::Ordinalize;

#[derive(Debug, PartialEq, Eq, Ordinalize)]
enum MyEnum {
    Zero,
    One,
    Two,
    Thousand = 1000,
}

assert_eq!(4, MyEnum::VARIANT_COUNT);
assert_eq!([MyEnum::Zero, MyEnum::One, MyEnum::Two, MyEnum::Thousand], MyEnum::VARIANTS);
assert_eq!([0i16, 1i16, 2i16, 1000i16], MyEnum::VALUES);

assert_eq!(0i16, MyEnum::Zero.ordinal());
assert_eq!(1i16, MyEnum::One.ordinal());
assert_eq!(2i16, MyEnum::Two.ordinal());

assert_eq!(Some(MyEnum::Zero), MyEnum::from_ordinal(0i16));
assert_eq!(Some(MyEnum::One), MyEnum::from_ordinal(1i16));
assert_eq!(Some(MyEnum::Two), MyEnum::from_ordinal(2i16));

assert_eq!(MyEnum::Zero, unsafe { MyEnum::from_ordinal_unsafe(0i16) });
assert_eq!(MyEnum::One, unsafe { MyEnum::from_ordinal_unsafe(1i16) });
assert_eq!(MyEnum::Two, unsafe { MyEnum::from_ordinal_unsafe(2i16) });
# }
```

In order to accommodate the value `1000`, the size of `MyEnum` increases. Consequently, the ordinal is represented in `i16` instead of `i8`.

You can utilize the `#[repr(type)]` attribute to explicitly control the size. For instance,

```rust
# #[cfg(feature = "derive")]
# {
use enum_ordinalize::Ordinalize;

#[derive(Debug, PartialEq, Eq, Ordinalize)]
#[repr(usize)]
enum MyEnum {
    Zero,
    One,
    Two,
    Thousand = 1000,
}

assert_eq!(4, MyEnum::VARIANT_COUNT);
assert_eq!([MyEnum::Zero, MyEnum::One, MyEnum::Two, MyEnum::Thousand], MyEnum::VARIANTS);
assert_eq!([0usize, 1usize, 2usize, 1000usize], MyEnum::VALUES);

assert_eq!(0usize, MyEnum::Zero.ordinal());
assert_eq!(1usize, MyEnum::One.ordinal());
assert_eq!(2usize, MyEnum::Two.ordinal());

assert_eq!(Some(MyEnum::Zero), MyEnum::from_ordinal(0usize));
assert_eq!(Some(MyEnum::One), MyEnum::from_ordinal(1usize));
assert_eq!(Some(MyEnum::Two), MyEnum::from_ordinal(2usize));

assert_eq!(MyEnum::Zero, unsafe { MyEnum::from_ordinal_unsafe(0usize) });
assert_eq!(MyEnum::One, unsafe { MyEnum::from_ordinal_unsafe(1usize) });
assert_eq!(MyEnum::Two, unsafe { MyEnum::from_ordinal_unsafe(2usize) });
# }
```

#### Useful Increment

The integers represented by variants can be extended in successive increments and set explicitly from any value.

```rust
# #[cfg(feature = "derive")]
# {
use enum_ordinalize::Ordinalize;

#[derive(Debug, PartialEq, Eq, Ordinalize)]
enum MyEnum {
    Two   = 2,
    Three,
    Four,
    Eight = 8,
    Nine,
    NegativeTen = -10,
    NegativeNine,
}

assert_eq!(7, MyEnum::VARIANT_COUNT);
assert_eq!([MyEnum::Two, MyEnum::Three, MyEnum::Four, MyEnum::Eight, MyEnum::Nine, MyEnum::NegativeTen, MyEnum::NegativeNine], MyEnum::VARIANTS);
assert_eq!([2i8, 3i8, 4i8, 8i8, 9i8, -10i8, -9i8], MyEnum::VALUES);

assert_eq!(4i8, MyEnum::Four.ordinal());
assert_eq!(9i8, MyEnum::Nine.ordinal());
assert_eq!(-9i8, MyEnum::NegativeNine.ordinal());

assert_eq!(Some(MyEnum::Four), MyEnum::from_ordinal(4i8));
assert_eq!(Some(MyEnum::Nine), MyEnum::from_ordinal(9i8));
assert_eq!(Some(MyEnum::NegativeNine), MyEnum::from_ordinal(-9i8));

assert_eq!(MyEnum::Four, unsafe { MyEnum::from_ordinal_unsafe(4i8) });
assert_eq!(MyEnum::Nine, unsafe { MyEnum::from_ordinal_unsafe(9i8) });
assert_eq!(MyEnum::NegativeNine, unsafe { MyEnum::from_ordinal_unsafe(-9i8) });
# }
```
*/

#![no_std]
#![cfg_attr(docsrs, feature(doc_auto_cfg))]

mod traits;

#[cfg(feature = "derive")]
pub use enum_ordinalize_derive::Ordinalize;
pub use traits::Ordinalize;
