/*!
# Enum Ordinalize

This crates provides a procedural macro to let enums not only get its variants' ordinal but also be constructed from an ordinal.

## Ordinalize

Use `#[derive(Ordinalize)]` to make an enum (which must only has unit variants) has `from_ordinal_unsafe`, `from_ordinal`, `variants`, and `variant_count` associated functions and a `ordinal` method.

```rust
#[macro_use] extern crate enum_ordinalize;

#[derive(Debug, PartialEq, Eq, Ordinalize)]
enum MyEnum {
    Zero,
    One,
    Two,
}

assert_eq!(0i8, MyEnum::Zero.ordinal());
assert_eq!(1i8, MyEnum::One.ordinal());
assert_eq!(2i8, MyEnum::Two.ordinal());

assert_eq!(Some(MyEnum::Zero), MyEnum::from_ordinal(0i8));
assert_eq!(Some(MyEnum::One), MyEnum::from_ordinal(1i8));
assert_eq!(Some(MyEnum::Two), MyEnum::from_ordinal(2i8));

assert_eq!(MyEnum::Zero, unsafe { MyEnum::from_ordinal_unsafe(0i8) });
assert_eq!(MyEnum::One, unsafe { MyEnum::from_ordinal_unsafe(1i8) });
assert_eq!(MyEnum::Two, unsafe { MyEnum::from_ordinal_unsafe(2i8) });
```

### Get Variants

```rust
#[macro_use] extern crate enum_ordinalize;

#[derive(Debug, PartialEq, Eq, Ordinalize)]
enum MyEnum {
    Zero,
    One,
    Two,
}

assert_eq!([MyEnum::Zero, MyEnum::One, MyEnum::Two], MyEnum::variants());
assert_eq!(3, MyEnum::variant_count());
```

`variants` and `variant_count` are constant functions.

## The (Ordinal) Size of an Enum

The ordinal value is an integer whose size is determined by the enum itself. The larger (or the smaller if it's negative) the variants' values are, the bigger the enum size is.

For example,

```rust
#[macro_use] extern crate enum_ordinalize;

#[derive(Debug, PartialEq, Eq, Ordinalize)]
enum MyEnum {
    Zero,
    One,
    Two,
    Thousand = 1000,
}

assert_eq!(0i16, MyEnum::Zero.ordinal());
assert_eq!(1i16, MyEnum::One.ordinal());
assert_eq!(2i16, MyEnum::Two.ordinal());

assert_eq!(Some(MyEnum::Zero), MyEnum::from_ordinal(0i16));
assert_eq!(Some(MyEnum::One), MyEnum::from_ordinal(1i16));
assert_eq!(Some(MyEnum::Two), MyEnum::from_ordinal(2i16));

assert_eq!(MyEnum::Zero, unsafe { MyEnum::from_ordinal_unsafe(0i16) });
assert_eq!(MyEnum::One, unsafe { MyEnum::from_ordinal_unsafe(1i16) });
assert_eq!(MyEnum::Two, unsafe { MyEnum::from_ordinal_unsafe(2i16) });
```

In order to store `1000`, the size of `MyEnum` grows. Thus, the ordinal is in `i16` instead of `i8`.

You can use the `#[repr(type)]` attribute to control the size explicitly. For instance,

```rust
#[macro_use] extern crate enum_ordinalize;

#[derive(Debug, PartialEq, Eq, Ordinalize)]
#[repr(usize)]
enum MyEnum {
    Zero,
    One,
    Two,
    Thousand = 1000,
}

assert_eq!(0usize, MyEnum::Zero.ordinal());
assert_eq!(1usize, MyEnum::One.ordinal());
assert_eq!(2usize, MyEnum::Two.ordinal());

assert_eq!(Some(MyEnum::Zero), MyEnum::from_ordinal(0usize));
assert_eq!(Some(MyEnum::One), MyEnum::from_ordinal(1usize));
assert_eq!(Some(MyEnum::Two), MyEnum::from_ordinal(2usize));

assert_eq!(MyEnum::Zero, unsafe { MyEnum::from_ordinal_unsafe(0usize) });
assert_eq!(MyEnum::One, unsafe { MyEnum::from_ordinal_unsafe(1usize) });
assert_eq!(MyEnum::Two, unsafe { MyEnum::from_ordinal_unsafe(2usize) });
```

## Useful Increment

The integers represented by variants are extended in successive increments and can be set explicitly from anywhere.

```rust
#[macro_use] extern crate enum_ordinalize;

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

assert_eq!(4i8, MyEnum::Four.ordinal());
assert_eq!(9i8, MyEnum::Nine.ordinal());
assert_eq!(-9i8, MyEnum::NegativeNine.ordinal());

assert_eq!(Some(MyEnum::Four), MyEnum::from_ordinal(4i8));
assert_eq!(Some(MyEnum::Nine), MyEnum::from_ordinal(9i8));
assert_eq!(Some(MyEnum::NegativeNine), MyEnum::from_ordinal(-9i8));

assert_eq!(MyEnum::Four, unsafe { MyEnum::from_ordinal_unsafe(4i8) });
assert_eq!(MyEnum::Nine, unsafe { MyEnum::from_ordinal_unsafe(9i8) });
assert_eq!(MyEnum::NegativeNine, unsafe { MyEnum::from_ordinal_unsafe(-9i8) });
```
*/

#![no_std]

extern crate alloc;
extern crate proc_macro;
extern crate syn;

#[macro_use]
extern crate quote;

extern crate num_bigint;

mod big_int_wrapper;
mod panic;
mod variant_type;

use core::ops::{AddAssign, Neg};
use core::str::FromStr;

use alloc::string::ToString;
use alloc::vec::Vec;

use proc_macro::TokenStream;
use quote::ToTokens;
use syn::{Data, DeriveInput, Expr, Fields, Ident, Lit, Meta, NestedMeta, UnOp};

use num_bigint::BigInt;

use big_int_wrapper::BigIntWrapper;
use variant_type::VariantType;

fn derive_input_handler(ast: DeriveInput) -> TokenStream {
    let mut variant_type = VariantType::default();

    for attr in ast.attrs.iter() {
        let attr_meta = attr.parse_meta().unwrap();

        let attr_meta_name = attr_meta.path().into_token_stream().to_string();

        if attr_meta_name.as_str() == "repr" {
            // #[repr(u8)], #[repr(u16)], ..., etc.
            if let Meta::List(list) = attr_meta {
                for p in &list.nested {
                    if let NestedMeta::Meta(meta) = p {
                        if let Meta::Path(path) = meta {
                            let meta_name = path.into_token_stream().to_string();

                            variant_type = VariantType::from_str(meta_name);

                            break;
                        }
                    }
                }
            }
        }
    }

    let name = &ast.ident;

    match ast.data {
        Data::Enum(data) => {
            if data.variants.is_empty() {
                panic::no_variant();
            }

            let mut min = BigInt::from(usize::max_value());
            let mut max = BigInt::from(isize::min_value());
            let mut counter = BigInt::default();
            let mut values: Vec<BigIntWrapper> = Vec::with_capacity(data.variants.len());
            let mut variant_idents: Vec<&Ident> = Vec::with_capacity(data.variants.len());

            for variant in data.variants.iter() {
                if let Fields::Unit = variant.fields {
                    let value = if let Some((_, exp)) = variant.discriminant.as_ref() {
                        match exp {
                            Expr::Lit(lit) => {
                                let lit = &lit.lit;

                                let value = match lit {
                                    Lit::Int(value) => {
                                        let value = value.base10_digits();
                                        BigInt::from_str(value).unwrap()
                                    }
                                    _ => panic::unsupported_discriminant(),
                                };

                                counter = value.clone();

                                value
                            }
                            Expr::Unary(unary) => {
                                match unary.op {
                                    UnOp::Neg(_) => {
                                        match unary.expr.as_ref() {
                                            Expr::Lit(lit) => {
                                                let lit = &lit.lit;

                                                let value = match lit {
                                                    Lit::Int(value) => {
                                                        let value = value.base10_digits();

                                                        BigInt::from_str(value).unwrap().neg()
                                                    }
                                                    _ => panic::unsupported_discriminant(),
                                                };

                                                counter = value.clone();

                                                value
                                            }
                                            _ => panic::unsupported_discriminant(),
                                        }
                                    }
                                    _ => panic::unsupported_discriminant(),
                                }
                            }
                            _ => panic::unsupported_discriminant(),
                        }
                    } else {
                        counter.clone()
                    };

                    if min > value {
                        min = value.clone();
                    }

                    if max < value {
                        max = value.clone();
                    }

                    values.push(BigIntWrapper::from(value));
                    variant_idents.push(&variant.ident);

                    counter.add_assign(1);
                } else {
                    panic::not_unit_variant();
                }
            }

            let repr128 = match variant_type {
                VariantType::Nondetermined => {
                    if min >= BigInt::from(i8::min_value()) && max <= BigInt::from(i8::max_value())
                    {
                        variant_type = VariantType::I8;

                        false
                    } else if min >= BigInt::from(i16::min_value())
                        && max <= BigInt::from(i16::max_value())
                    {
                        variant_type = VariantType::I16;

                        false
                    } else if min >= BigInt::from(i32::min_value())
                        && max <= BigInt::from(i32::max_value())
                    {
                        variant_type = VariantType::I32;

                        false
                    } else if min >= BigInt::from(i64::min_value())
                        && max <= BigInt::from(i64::max_value())
                    {
                        variant_type = VariantType::I64;

                        false
                    } else if min >= BigInt::from(i128::min_value())
                        && max <= BigInt::from(i128::max_value())
                    {
                        variant_type = VariantType::I128;

                        true
                    } else {
                        panic::unsupported_discriminant()
                    }
                }
                VariantType::I128 | VariantType::U128 => true,
                _ => false,
            };

            let ordinal = if repr128 {
                quote! {
                    #[inline]
                    pub fn ordinal(&self) -> #variant_type {
                        match self {
                            #(
                                Self::#variant_idents => #values,
                            )*
                        }
                    }
                }
            } else {
                quote! {
                    #[inline]
                    pub fn ordinal(&self) -> #variant_type {
                        let n: u64 = unsafe {
                            ::std::mem::transmute(::std::mem::discriminant(self))
                        };

                        n as #variant_type
                    }
                }
            };

            let (impl_generics, ty_generics, where_clause) = ast.generics.split_for_impl();

            let from_ordinal_unsafe = quote! {
                #[inline]
                pub unsafe fn from_ordinal_unsafe(number: #variant_type) -> #name #ty_generics {
                    ::std::mem::transmute(number)
                }
            };

            let from_ordinal = quote! {
                #[inline]
                pub fn from_ordinal(number: #variant_type) -> Option<#name #ty_generics> {
                    match number{
                        #(
                            #values => Some(Self::#variant_idents),
                        )*
                        _ => None
                    }
                }
            };

            let variant_count = variant_idents.len();

            let variants = quote! {
                #[inline]
                pub const fn variants() -> [#name #ty_generics; #variant_count] {
                    [#( Self::#variant_idents, )*]
                }
            };

            let variant_count = quote! {
                #[inline]
                pub const fn variant_count() -> usize {
                    #variant_count
                }
            };

            let ordinalize_impl = quote! {
                impl #impl_generics #name #ty_generics #where_clause {
                    #from_ordinal_unsafe

                    #from_ordinal

                    #ordinal

                    #variants

                    #variant_count
                }
            };

            ordinalize_impl.into()
        }
        _ => {
            panic::not_enum();
        }
    }
}

#[proc_macro_derive(Ordinalize)]
pub fn ordinalize_derive(input: TokenStream) -> TokenStream {
    derive_input_handler(syn::parse(input).unwrap())
}
