use num_bigint::BigInt;
use num_traits::{Signed, ToPrimitive};
use proc_macro2::{Literal, TokenStream};
use quote::{quote, ToTokens, TokenStreamExt};
use syn::Expr;

pub(crate) enum BigIntWrapper {
    Integer(BigInt),
    Constant(Expr, usize),
}

impl From<BigInt> for BigIntWrapper {
    #[inline]
    fn from(v: BigInt) -> BigIntWrapper {
        BigIntWrapper::Integer(v)
    }
}

impl From<(&Expr, usize)> for BigIntWrapper {
    #[inline]
    fn from((expr, counter): (&Expr, usize)) -> BigIntWrapper {
        BigIntWrapper::Constant(expr.clone(), counter)
    }
}

impl ToTokens for BigIntWrapper {
    #[inline]
    fn to_tokens(&self, tokens: &mut TokenStream) {
        match self {
            BigIntWrapper::Integer(v) => {
                let lit = if v.is_negative() {
                    Literal::i128_unsuffixed(v.to_i128().unwrap())
                } else {
                    Literal::u128_unsuffixed(v.to_u128().unwrap())
                };

                tokens.append(lit);
            },
            BigIntWrapper::Constant(expr, counter) => {
                let counter = *counter;

                if counter > 0 {
                    tokens.extend(quote!(#expr +));
                    tokens.append(Literal::usize_unsuffixed(counter));
                } else {
                    tokens.extend(quote!(#expr));
                }
            },
        }
    }
}
