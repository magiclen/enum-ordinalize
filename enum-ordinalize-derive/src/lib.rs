/*!
# Enum Ordinalize Derive

This library enables enums to not only obtain the ordinal values of their variants but also allows for the construction of enums from an ordinal value. See the [`enum-ordinalize`](https://crates.io/crates/enum-ordinalize) crate.
 */

mod big_int_wrapper;
mod panic;
mod variant_type;

use std::str::FromStr;

use num_bigint::BigInt;
use proc_macro::TokenStream;
use quote::quote;
use syn::{
    parse::{Parse, ParseStream},
    parse_macro_input,
    punctuated::Punctuated,
    spanned::Spanned,
    Data, DeriveInput, Expr, Fields, Ident, Lit, Meta, MetaList, Token, UnOp, Visibility,
};
use variant_type::VariantType;

use crate::big_int_wrapper::BigIntWrapper;

#[proc_macro_derive(Ordinalize, attributes(ordinalize))]
pub fn ordinalize_derive(input: TokenStream) -> TokenStream {
    struct MyVariantsParser {
        vis:   Option<Visibility>,
        ident: Ident,
        meta:  Vec<Meta>,
    }

    impl Parse for MyVariantsParser {
        #[inline]
        fn parse(input: ParseStream) -> Result<Self, syn::Error> {
            let vis = input.parse::<Visibility>().ok();

            let _ = input.parse::<Token![const]>();

            input.parse::<Token![fn]>()?;

            let ident = input.parse::<Ident>()?;

            let mut meta = Vec::new();

            if !input.is_empty() {
                input.parse::<Token![,]>()?;

                if !input.is_empty() {
                    let result = Punctuated::<Meta, Token![,]>::parse_terminated(input)?;

                    let mut has_inline = false;

                    for m in result {
                        if m.path().is_ident("inline") {
                            has_inline = true;
                        }

                        meta.push(m);
                    }

                    if !has_inline {
                        meta.push(syn::parse_str("inline")?);
                    }
                }
            }

            Ok(MyVariantsParser {
                vis,
                ident,
                meta,
            })
        }
    }

    struct MyDeriveInput {
        ast:                 DeriveInput,
        variant_count:       usize,
        variant_type:        VariantType,
        ordinal:             proc_macro2::TokenStream,
        from_ordinal_unsafe: proc_macro2::TokenStream,
        from_ordinal:        proc_macro2::TokenStream,
        variants:            Option<proc_macro2::TokenStream>,
    }

    impl Parse for MyDeriveInput {
        fn parse(input: ParseStream) -> Result<Self, syn::Error> {
            let ast = input.parse::<DeriveInput>()?;

            let mut variant_type = VariantType::default();

            let mut my_variants_parser: Option<MyVariantsParser> = None;

            for attr in ast.attrs.iter() {
                let path = attr.path();

                if path.is_ident("repr") {
                    // #[repr(u8)], #[repr(u16)], ..., etc.
                    if let Meta::List(list) = &attr.meta {
                        let result =
                            list.parse_args_with(Punctuated::<Ident, Token![,]>::parse_terminated)?;

                        if let Some(value) = result.into_iter().next() {
                            variant_type = VariantType::from_str(value.to_string());
                        }
                    }

                    break;
                } else if path.is_ident("ordinalize") {
                    if let Meta::List(list) = &attr.meta {
                        let result = list
                            .parse_args_with(Punctuated::<MetaList, Token![,]>::parse_terminated)?;

                        for list in result {
                            let path = &list.path;

                            if list.path.is_ident("variants") {
                                my_variants_parser = Some(list.parse_args()?);
                            } else {
                                return Err(panic::ordinalize_attribute_usage(path.span()));
                            }
                        }
                    } else {
                        return Err(panic::ordinalize_attribute_usage(path.span()));
                    }
                }
            }

            let name = &ast.ident;

            match &ast.data {
                Data::Enum(data) => {
                    let variant_count = data.variants.len();

                    if variant_count == 0 {
                        return Err(panic::no_variant(name.span()));
                    }

                    let mut values: Vec<BigIntWrapper> = Vec::with_capacity(variant_count);
                    let mut variant_idents: Vec<&Ident> = Vec::with_capacity(variant_count);

                    let mut use_constant_counter = false;

                    match variant_type {
                        VariantType::NonDetermined => {
                            let mut min = BigInt::from(u128::MAX);
                            let mut max = BigInt::from(i128::MIN);
                            let mut counter = BigInt::default();

                            for variant in data.variants.iter() {
                                if let Fields::Unit = variant.fields {
                                    let value = if let Some((_, exp)) =
                                        variant.discriminant.as_ref()
                                    {
                                        match exp {
                                            Expr::Lit(lit) => {
                                                let lit = &lit.lit;

                                                match lit {
                                                    Lit::Int(value) => {
                                                        let value = value.base10_digits();

                                                        counter = BigInt::from_str(value).unwrap();

                                                        counter.clone()
                                                    }
                                                    _ => {
                                                        return Err(panic::unsupported_discriminant(lit.span()));
                                                    }
                                                }
                                            },
                                            Expr::Unary(unary) => match unary.op {
                                                UnOp::Neg(_) => match unary.expr.as_ref() {
                                                    Expr::Lit(lit) => {
                                                        let lit = &lit.lit;

                                                        match lit {
                                                            Lit::Int(value) => {
                                                                let value = value.base10_digits();

                                                                counter = -BigInt::from_str(value).unwrap();

                                                                counter.clone()
                                                            }
                                                            _ => {
                                                                return Err(panic::unsupported_discriminant(lit.span()));
                                                            }
                                                        }
                                                    },
                                                    Expr::Path(_)
                                                    | Expr::Cast(_)
                                                    | Expr::Binary(_)
                                                    | Expr::Call(_) => {
                                                        return Err(panic::constant_variable_on_non_determined_size_enum(unary.expr.span()))
                                                    },
                                                    _ => return Err(panic::unsupported_discriminant(unary.expr.span())),
                                                },
                                                _ => return Err(panic::unsupported_discriminant(unary.op.span())),
                                            },
                                            Expr::Path(_) | Expr::Cast(_) | Expr::Binary(_) | Expr::Call(_) => {
                                                return Err(panic::constant_variable_on_non_determined_size_enum(exp.span()))
                                            },
                                            _ => return Err(panic::unsupported_discriminant(exp.span())),
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

                                    variant_idents.push(&variant.ident);

                                    counter += 1;

                                    values.push(BigIntWrapper::from(value));
                                } else {
                                    return Err(panic::not_unit_variant(variant.span()));
                                }
                            }

                            if min >= BigInt::from(i8::MIN) && max <= BigInt::from(i8::MAX) {
                                variant_type = VariantType::I8;
                            } else if min >= BigInt::from(i16::MIN) && max <= BigInt::from(i16::MAX)
                            {
                                variant_type = VariantType::I16;
                            } else if min >= BigInt::from(i32::MIN) && max <= BigInt::from(i32::MAX)
                            {
                                variant_type = VariantType::I32;
                            } else if min >= BigInt::from(i64::MIN) && max <= BigInt::from(i64::MAX)
                            {
                                variant_type = VariantType::I64;
                            } else if min >= BigInt::from(i128::MIN)
                                && max <= BigInt::from(i128::MAX)
                            {
                                variant_type = VariantType::I128;
                            } else {
                                return Err(panic::unsupported_discriminant(name.span()));
                            }
                        },
                        _ => {
                            let mut counter = BigInt::default();
                            let mut constant_counter = 0;
                            let mut last_exp: Option<&Expr> = None;

                            for variant in data.variants.iter() {
                                if let Fields::Unit = variant.fields {
                                    if let Some((_, exp)) = variant.discriminant.as_ref() {
                                        match exp {
                                            Expr::Lit(lit) => {
                                                let lit = &lit.lit;

                                                match lit {
                                                    Lit::Int(value) => {
                                                        let value = value.base10_digits();

                                                        counter = BigInt::from_str(value).unwrap();

                                                        values.push(BigIntWrapper::from(
                                                            counter.clone(),
                                                        ));

                                                        counter += 1;

                                                        last_exp = None;
                                                    },
                                                    _ => {
                                                        return Err(
                                                            panic::unsupported_discriminant(
                                                                lit.span(),
                                                            ),
                                                        );
                                                    },
                                                }
                                            },
                                            Expr::Unary(unary) => match unary.op {
                                                UnOp::Neg(_) => match unary.expr.as_ref() {
                                                    Expr::Lit(lit) => {
                                                        let lit = &lit.lit;

                                                        match lit {
                                                            Lit::Int(value) => {
                                                                let value = value.base10_digits();

                                                                counter = -BigInt::from_str(value)
                                                                    .unwrap();

                                                                values.push(BigIntWrapper::from(
                                                                    counter.clone(),
                                                                ));

                                                                counter += 1;

                                                                last_exp = None;
                                                            },
                                                            _ => {
                                                                return Err(
                                                                    panic::unsupported_discriminant(
                                                                        lit.span(),
                                                                    ),
                                                                );
                                                            },
                                                        }
                                                    },
                                                    Expr::Path(_) => {
                                                        values.push(BigIntWrapper::from((exp, 0)));

                                                        last_exp = Some(exp);
                                                        constant_counter = 1;
                                                    },
                                                    Expr::Cast(_)
                                                    | Expr::Binary(_)
                                                    | Expr::Call(_) => {
                                                        values.push(BigIntWrapper::from((exp, 0)));

                                                        last_exp = Some(exp);
                                                        constant_counter = 1;

                                                        use_constant_counter = true;
                                                    },
                                                    _ => {
                                                        return Err(
                                                            panic::unsupported_discriminant(
                                                                exp.span(),
                                                            ),
                                                        )
                                                    },
                                                },
                                                _ => {
                                                    return Err(panic::unsupported_discriminant(
                                                        unary.op.span(),
                                                    ))
                                                },
                                            },
                                            Expr::Path(_) => {
                                                values.push(BigIntWrapper::from((exp, 0)));

                                                last_exp = Some(exp);
                                                constant_counter = 1;
                                            },
                                            Expr::Cast(_) | Expr::Binary(_) | Expr::Call(_) => {
                                                values.push(BigIntWrapper::from((exp, 0)));

                                                last_exp = Some(exp);
                                                constant_counter = 1;

                                                use_constant_counter = true;
                                            },
                                            _ => {
                                                return Err(panic::unsupported_discriminant(
                                                    exp.span(),
                                                ))
                                            },
                                        }
                                    } else if let Some(exp) = last_exp {
                                        values.push(BigIntWrapper::from((exp, constant_counter)));

                                        constant_counter += 1;

                                        use_constant_counter = true;
                                    } else {
                                        values.push(BigIntWrapper::from(counter.clone()));

                                        counter += 1;
                                    }

                                    variant_idents.push(&variant.ident);
                                } else {
                                    return Err(panic::not_unit_variant(variant.span()));
                                }
                            }
                        },
                    }

                    let ordinal = quote! {
                        #[inline]
                        fn ordinal(&self) -> #variant_type {
                            match self {
                                #(
                                    Self::#variant_idents => #values,
                                )*
                            }
                        }
                    };

                    let from_ordinal_unsafe = if variant_count == 1 {
                        let variant_ident = &data.variants[0].ident;

                        quote! {
                            #[inline]
                            unsafe fn from_ordinal_unsafe(_number: #variant_type) -> Self {
                                Self::#variant_ident
                            }
                        }
                    } else {
                        quote! {
                            #[inline]
                            unsafe fn from_ordinal_unsafe(number: #variant_type) -> Self {
                                ::core::mem::transmute(number)
                            }
                        }
                    };

                    let from_ordinal = if use_constant_counter {
                        quote! {
                            #[inline]
                            fn from_ordinal(number: #variant_type) -> Option<Self> {
                                if false {
                                    unreachable!()
                                } #( else if number == #values {
                                    Some(Self::#variant_idents)
                                } )* else {
                                    None
                                }
                            }
                        }
                    } else {
                        quote! {
                            #[inline]
                            fn from_ordinal(number: #variant_type) -> Option<Self> {
                                match number{
                                    #(
                                        #values => Some(Self::#variant_idents),
                                    )*
                                    _ => None
                                }
                            }
                        }
                    };

                    let variants = if let Some(MyVariantsParser {
                        vis,
                        ident,
                        meta,
                    }) = my_variants_parser
                    {
                        Some(quote! {
                            #(#[#meta])*
                            #vis const fn #ident () -> [Self; #variant_count] {
                                [#( Self::#variant_idents, )*]
                            }
                        })
                    } else {
                        None
                    };

                    Ok(MyDeriveInput {
                        ast,
                        variant_count,
                        variant_type,
                        ordinal,
                        from_ordinal_unsafe,
                        from_ordinal,
                        variants,
                    })
                },
                _ => Err(panic::not_enum(ast.ident.span())),
            }
        }
    }

    // Parse the token stream
    let derive_input = parse_macro_input!(input as MyDeriveInput);

    let MyDeriveInput {
        ast,
        variant_count,
        variant_type,
        ordinal,
        from_ordinal_unsafe,
        from_ordinal,
        variants,
    } = derive_input;

    // Get the identifier of the type.
    let name = &ast.ident;

    let (impl_generics, ty_generics, where_clause) = ast.generics.split_for_impl();

    // Build the code
    let mut expanded = quote! {
        impl #impl_generics Ordinalize for #name #ty_generics #where_clause {
            type VariantType = #variant_type;

            const VARIANT_COUNT:usize = #variant_count;

            #ordinal

            #from_ordinal_unsafe

            #from_ordinal
        }
    };

    if let Some(variants) = variants {
        expanded.extend(quote! {
            impl #impl_generics #name #ty_generics #where_clause {
                #variants
            }
        });
    }

    expanded.into()
}
