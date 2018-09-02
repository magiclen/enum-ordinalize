//! # Enum Ordinalize
//! This crates provides `ordinalize_enum` and `create_ordinalized_enum` macros to let enums can not only get its variants' ordinal but also be constructed with an ordinal.
//!
//! ## Make an Enum Ordinalized
//!
//! `ordinalize_enum` macro can implement a `ordinal` method and a `from_ordinal` associated function for an existing `enum`.
//!
//! ```
//! #[macro_use] extern crate enum_ordinalize;
//!
//! #[derive(Debug, PartialEq)]
//! enum MyEnum {
//!     Zero,
//!     One,
//!     Two,
//! }
//!
//! ordinalize_enum!(MyEnum,
//!     u8,
//!     Zero,
//!     One,
//!     Two
//! );
//!
//! assert_eq!(2, MyEnum::Two.ordinal());
//! assert_eq!(Some(MyEnum::One), MyEnum::from_ordinal(1));
//! ```
//!
//! ## Create an Ordinalized Enum
//!
//! `create_ordinalized_enum` macro can create an enum and implement a `ordinal` method and a `from_ordinal` associated function for it.
//! The new enum also implements `Debug`, `PartialEq`, and `Clone` traits.
//!
//! ```
//! #[macro_use] extern crate enum_ordinalize;
//!
//! create_ordinalized_enum!(MyEnum,
//!     u8,
//!     Zero,
//!     One,
//!     Two
//! );
//!
//! assert_eq!(2, MyEnum::Two.ordinal());
//! assert_eq!(Some(MyEnum::One), MyEnum::from_ordinal(1));
//!
//! create_ordinalized_enum!(pub MyPublicEnum,
//!     u8,
//!     A,
//!     B,
//!     C
//! );
//!
//! assert_eq!(2, MyPublicEnum::C.ordinal());
//! assert_eq!(Some(MyPublicEnum::B), MyPublicEnum::from_ordinal(1));
//!
//! create_ordinalized_enum!(MySpecialEnum,
//!     u8,
//!     Two = 2,
//!     Four = 4,
//!     Eight = 8
//! );
//!
//! assert_eq!(2, MySpecialEnum::Two.ordinal());
//! assert_eq!(Some(MySpecialEnum::Four), MySpecialEnum::from_ordinal(4));
//! ```

/// Implement a `ordinal` method and a `from_ordinal` associated function for an existing `enum`.
#[macro_export]
macro_rules! ordinalize_enum {
    ( $name:ident, $t:ty $( , $variants:ident )+ $(,)* ) => {
        impl $name {
            fn ordinal(&self) -> $t {
                match self {
                    $(
                        $name::$variants => $name::$variants as $t,
                    )+
                }
            }

            fn from_ordinal(number: $t) -> Option<$name> {
                match number{
                    $(
                        n if n == ($name::$variants as $t) => Some($name::$variants),
                    )+
                    _ => None
                }
            }
        }
    }
}

/// Create an enum and implement a `ordinal` method and a `from_ordinal` associated function for it. The new enum also implements `Debug`, `PartialEq`, and `Clone` traits.
#[macro_export]
macro_rules! create_ordinalized_enum {
    ( $name:ident, $t:ty $( ,$variants:ident )+ $(,)* ) => {
        #[derive(Debug, PartialEq, Clone)]
        enum $name {
            $(
                $variants,
            )+
        }
        ordinalize_enum!(
            $name,
            $t,
            $(
                $variants,
            )+
        );
    };
    ( $name:ident, $t:ty $( ,$variants:ident = $values:expr )+ $(,)* ) => {
        #[derive(Debug, PartialEq, Clone)]
        enum $name {
            $(
                $variants = $values,
            )+
        }
        ordinalize_enum!(
            $name,
            $t,
            $(
                $variants,
            )+
        );
    };
    ( pub $name:ident, $t:ty $( ,$variants:ident )+ $(,)* ) => {
        #[derive(Debug, PartialEq, Clone)]
        pub enum $name {
            $(
                $variants,
            )+
        }
        ordinalize_enum!(
            $name,
            $t,
            $(
                $variants,
            )+
        );
    };
    ( pub $name:ident, $t:ty $( ,$variants:ident = $values:expr )+ $(,)* ) => {
        #[derive(Debug, PartialEq, Clone)]
        pub enum $name {
            $(
                $variants = $values,
            )+
        }
        ordinalize_enum!(
            $name,
            $t,
            $(
                $variants,
            )+
        );
    };
}