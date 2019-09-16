#[macro_export]
#[doc(hidden)]
macro_rules! ordinalize_enum_from_ordinal {
    ( $name:ident: $t:ident $( , $variants:ident )+ $(,)* ) => {
        pub fn from_ordinal(number: $t) -> Option<$name> {
            match number{
                $(
                    n if n == ($name::$variants as $t) => Some($name::$variants),
                )+
                _ => None
            }
        }

        pub unsafe fn from_ordinal_unsafe(number: $t) -> $name {
            ::std::mem::transmute(number)
        }
    }
}

#[cfg(feature = "repr128")]
#[macro_export]
#[doc(hidden)]
macro_rules! ordinalize_enum_ordinal {
    ( $name:ident: $t:ident) => {
        pub fn ordinal(&self) -> $t {
            let n: u128 = unsafe {
                ::std::mem::transmute(::std::mem::discriminant(self))
            };

            n as $t
        }
    }
}

#[cfg(not(feature = "repr128"))]
#[macro_export]
#[doc(hidden)]
macro_rules! ordinalize_enum_ordinal {
    ( $name:ident: $t:ident) => {
        pub fn ordinal(&self) -> $t {
            let n: u64 = unsafe {
                ::std::mem::transmute(::std::mem::discriminant(self))
            };

            n as $t
        }
    }
}

#[macro_export]
#[doc(hidden)]
macro_rules! ordinalize_enum_impl {
    ( $name:ident: $t:ident $( , $variants:ident )+ $(,)* ) => {
        impl $name {
            ordinalize_enum_ordinal!($name: $t);

            ordinalize_enum_from_ordinal!(
                $name: $t,
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
            $name: isize,
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
            $name: isize,
            $(
                $variants,
            )+
        );
    };
    ( $name:ident: $t:ident $( ,$variants:ident )+ $(,)* ) => {
        #[repr($t)]
        #[derive(Debug, PartialOrd, Ord, PartialEq, Clone, Eq, Hash, Copy)]
        enum $name {
            $(
                $variants,
            )+
        }
        ordinalize_enum_impl!(
            $name: $t,
            $(
                $variants,
            )+
        );
    };
    ( $name:ident: $t:ident $( ,$variants:ident = $values:expr )+ $(,)* ) => {
        #[repr($t)]
        #[derive(Debug, PartialOrd, Ord, PartialEq, Clone, Eq, Hash, Copy)]
        enum $name {
            $(
                $variants = $values,
            )+
        }
        ordinalize_enum_impl!(
            $name: $t,
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
            $name: isize,
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
            $name: isize,
            $(
                $variants,
            )+
        );
    };
    ( $v:vis $name:ident: $t:ident $( ,$variants:ident )+ $(,)* ) => {
        #[repr($t)]
        #[derive(Debug, PartialOrd, Ord, PartialEq, Clone, Eq, Hash, Copy)]
        $v enum $name {
            $(
                $variants,
            )+
        }
        ordinalize_enum_impl!(
            $name: $t,
            $(
                $variants,
            )+
        );
    };
    ( $v:vis $name:ident: $t:ident $( ,$variants:ident = $values:expr )+ $(,)* ) => {
        #[repr($t)]
        #[derive(Debug, PartialOrd, Ord, PartialEq, Clone, Eq, Hash, Copy)]
        $v enum $name {
            $(
                $variants = $values,
            )+
        }
        ordinalize_enum_impl!(
            $name: $t,
            $(
                $variants,
            )+
        );
    };
}
