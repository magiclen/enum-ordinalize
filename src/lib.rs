//! # Enum Ordinalize
//! This crates provides `create_ordinalized_enum` macro to let enums can not only get its variants' ordinal but also be constructed from an ordinal.
//!
//! ## Create an Ordinalized Enum
//!
//! `create_ordinalized_enum` macro can create an enum and implement a `ordinal` method, as well as `from_ordinal` and `from_ordinal_unsafe` associated functions for it.
//! The new enum also implements `Debug`, `PartialEq`, and `Clone` traits.
//!
//! ```
//! #[macro_use] extern crate enum_ordinalize;
//!
//! create_ordinalized_enum!(MyEnum,
//!     Zero,
//!     One,
//!     Two
//! );
//!
//! assert_eq!(2, MyEnum::Two.ordinal());
//! assert_eq!(Some(MyEnum::One), MyEnum::from_ordinal(1));
//!
//! create_ordinalized_enum!(pub MyPublicEnum,
//!     A,
//!     B,
//!     C
//! );
//!
//! assert_eq!(2, MyPublicEnum::C.ordinal());
//! assert_eq!(Some(MyPublicEnum::B), MyPublicEnum::from_ordinal(1));
//!
//! create_ordinalized_enum!(MySpecialEnum,
//!     Two = 2,
//!     Four = 4,
//!     Eight = 8
//! );
//!
//! assert_eq!(2, MySpecialEnum::Two.ordinal());
//! assert_eq!(Some(MySpecialEnum::Four), MySpecialEnum::from_ordinal(4));
//! ```
//!
//! ## About an Ordinalized Enum
//!
//! An ordinalized enum is always sized **isize** in order to directly **transmute** into an **isize** value, or conversely.
//! There is a variant named *__DotNotUse* whose ordinal is always the maximum of an **isize** value for every ordinalized enum.
//!
//! If you are 100% sure that the **isize** value can transmute into a variant of your ordinalized enum. You can use the `from_ordinal_unsafe` associated function and the **unsafe** keyword to speed up.
//!
//! ```
//! #[macro_use] extern crate enum_ordinalize;
//!
//! create_ordinalized_enum!(MyEnum,
//!     Zero,
//!     One,
//!     Two
//! );
//!
//! assert_eq!(MyEnum::One, unsafe{MyEnum::from_ordinal_unsafe(1)});
//! ```

#[macro_export]
#[doc(hidden)]
macro_rules! ordinalize_enum_from_ordinal {
    ( $name:ident $( , $variants:ident )+ $(,)* ) => {
        fn from_ordinal(number: isize) -> Option<$name> {
            match number{
                $(
                    n if n == ($name::$variants as isize) => Some($name::$variants),
                )+
                _ => None
            }
        }

        unsafe fn from_ordinal_unsafe(number: isize) -> $name {
            ::std::mem::transmute(number)
        }
    }
}

#[cfg(not(all(windows, target_pointer_width = "32")))]
#[macro_export]
#[doc(hidden)]
macro_rules! ordinalize_enum_ordinal {
    ( $name:ident $( , $variants:ident )+ $(,)* ) => {
        fn ordinal(&self) -> isize {
            unsafe {
                ::std::mem::transmute(::std::mem::discriminant(self))
            }
        }
    }
}

#[cfg(all(windows, target_pointer_width = "32"))]
#[macro_export]
#[doc(hidden)]
macro_rules! ordinalize_enum_ordinal {
    ( $name:ident $( , $variants:ident )+ $(,)* ) => {
        fn ordinal(&self) -> isize {
            let n: i64 = unsafe {
                ::std::mem::transmute(::std::mem::discriminant(self))
            };

            n as isize
        }
    }
}

#[macro_export]
#[doc(hidden)]
macro_rules! ordinalize_enum_impl {
    ( $name:ident $( , $variants:ident )+ $(,)* ) => {
        impl $name {
            ordinalize_enum_ordinal!(
                $name,
                $(
                    $variants,
                )+
            );

            ordinalize_enum_from_ordinal!(
                $name,
                $(
                    $variants,
                )+
            );
        }
    }
}

/// Create an enum and implement a `ordinal` method, as well as `from_ordinal` and `from_ordinal_unsafe` associated functions for it. The new enum also implements `Debug`, `PartialEq`, and `Clone` traits.
#[macro_export]
macro_rules! create_ordinalized_enum {
    ( $name:ident $( ,$variants:ident )+ $(,)* ) => {
        #[derive(Debug, PartialEq, Clone)]
        enum $name {
            $(
                $variants,
            )+

            __DotNotUse = isize::max_value()
        }
        ordinalize_enum_impl!(
            $name,
            $(
                $variants,
            )+
        );
    };
    ( $name:ident $( ,$variants:ident = $values:expr )+ $(,)* ) => {
        #[derive(Debug, PartialEq, Clone)]
        enum $name {
            $(
                $variants = $values,
            )+

            __DotNotUse = isize::max_value()
        }
        ordinalize_enum_impl!(
            $name,
            $(
                $variants,
            )+
        );
    };
    ( pub $name:ident $( ,$variants:ident )+ $(,)* ) => {
        #[derive(Debug, PartialEq, Clone)]
        pub enum $name {
            $(
                $variants,
            )+

            __DotNotUse = isize::max_value()
        }
        ordinalize_enum_impl!(
            $name,
            $(
                $variants,
            )+
        );
    };
    ( pub $name:ident $( ,$variants:ident = $values:expr )+ $(,)* ) => {
        #[derive(Debug, PartialEq, Clone)]
        pub enum $name {
            $(
                $variants = $values,
            )+

            __DotNotUse = isize::max_value()
        }
        ordinalize_enum_impl!(
            $name,
            $(
                $variants,
            )+
        );
    };
}