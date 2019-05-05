#[macro_export]
#[doc(hidden)]
macro_rules! ordinalize_enum_from_ordinal {
    ( $name:ident $( , $variants:ident )+ $(,)* ) => {
        pub fn from_ordinal(number: isize) -> Option<$name> {
            match number{
                $(
                    n if n == ($name::$variants as isize) => Some($name::$variants),
                )+
                _ => None
            }
        }

        pub unsafe fn from_ordinal_unsafe(number: isize) -> $name {
            ::std::mem::transmute(number)
        }
    }
}

#[cfg(not(all(windows, target_pointer_width = "32")))]
#[macro_export]
#[doc(hidden)]
macro_rules! ordinalize_enum_ordinal {
    ( $name:ident $( , $variants:ident )+ $(,)* ) => {
        pub fn ordinal(&self) -> isize {
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
        pub fn ordinal(&self) -> isize {
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

/// Create an enum and implement a `ordinal` method, as well as `from_ordinal` and `from_ordinal_unsafe` associated functions for it. The new enum also implements `Debug`, `PartialOrd`, `Ord`, `PartialEq`, `Clone`, `Eq`, `Hash` and `Copy` traits.
#[macro_export]
macro_rules! create_ordinalized_enum {
    ( $name:ident $( ,$variants:ident )+ $(,)* ) => {
        #[repr(isize)]
        #[derive(Debug, PartialOrd, Ord, PartialEq, Clone, Eq, Hash, Copy)]
        enum $name {
            $(
                $variants,
            )+
        }
        ordinalize_enum_impl!(
            $name,
            $(
                $variants,
            )+
        );
    };
    ( $name:ident $( ,$variants:ident = $values:expr )+ $(,)* ) => {
        #[repr(isize)]
        #[derive(Debug, PartialOrd, Ord, PartialEq, Clone, Eq, Hash, Copy)]
        enum $name {
            $(
                $variants = $values,
            )+
        }
        ordinalize_enum_impl!(
            $name,
            $(
                $variants,
            )+
        );
    };
    ( $v:vis $name:ident $( ,$variants:ident )+ $(,)* ) => {
        #[repr(isize)]
        #[derive(Debug, PartialOrd, Ord, PartialEq, Clone, Eq, Hash, Copy)]
        $v enum $name {
            $(
                $variants,
            )+
        }
        ordinalize_enum_impl!(
            $name,
            $(
                $variants,
            )+
        );
    };
    ( $v:vis $name:ident $( ,$variants:ident = $values:expr )+ $(,)* ) => {
        #[repr(isize)]
        #[derive(Debug, PartialOrd, Ord, PartialEq, Clone, Eq, Hash, Copy)]
        $v enum $name {
            $(
                $variants = $values,
            )+
        }
        ordinalize_enum_impl!(
            $name,
            $(
                $variants,
            )+
        );
    };
}