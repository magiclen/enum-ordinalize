extern crate num_traits;
extern crate proc_macro2;

use crate::num_bigint::BigInt;
use crate::quote::{ToTokens, TokenStreamExt};

use num_traits::{Signed, ToPrimitive};
use proc_macro2::{Literal, TokenStream};

#[derive(Debug)]
pub(crate) struct BigIntWrapper(BigInt);

impl From<BigInt> for BigIntWrapper {
    #[inline]
    fn from(v: BigInt) -> BigIntWrapper {
        BigIntWrapper(v)
    }
}

impl Into<BigInt> for BigIntWrapper {
    #[inline]
    fn into(self) -> BigInt {
        self.0
    }
}

impl ToTokens for BigIntWrapper {
    #[inline]
    fn to_tokens(&self, tokens: &mut TokenStream) {
        let lit = if self.0.is_negative() {
            Literal::i128_unsuffixed(self.0.to_i128().unwrap())
        } else {
            Literal::u128_unsuffixed(self.0.to_u128().unwrap())
        };

        tokens.append(lit);
    }
}
