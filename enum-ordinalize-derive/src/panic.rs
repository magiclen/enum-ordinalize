use core::fmt::{self, Display, Formatter};

use quote::ToTokens;
use syn::{Ident, Variant};

struct DisplayStringSlice<'a>(&'a [&'static str]);

impl<'a> Display for DisplayStringSlice<'a> {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        for &s in self.0 {
            f.write_str("\n    ")?;
            f.write_str(s)?;
        }

        Ok(())
    }
}

#[inline]
pub(crate) fn not_enum<T: ToTokens>(tokens: T) -> syn::Error {
    syn::Error::new_spanned(tokens, "only enums can be ordinalized")
}

#[inline]
pub(crate) fn no_variant<T: ToTokens>(tokens: T) -> syn::Error {
    syn::Error::new_spanned(tokens, "an ordinalized enum needs to have at least one variant")
}

#[inline]
pub(crate) fn not_unit_variant(variant: &Variant) -> syn::Error {
    syn::Error::new_spanned(variant, "an ordinalized enum can only have unit variants")
}

#[inline]
pub(crate) fn unsupported_discriminant<T: ToTokens>(tokens: T) -> syn::Error {
    syn::Error::new_spanned(
        tokens,
        "the discriminant of a variant of an ordinalized enum needs to be a legal literal \
         integer, a constant variable/function or a constant expression",
    )
}
#[inline]
pub(crate) fn constant_variable_on_non_determined_size_enum<T: ToTokens>(tokens: T) -> syn::Error {
    syn::Error::new_spanned(
        tokens,
        "the discriminant of a variant can be assigned not to a literal integer only when the \
         ordinalized enum is using the `repr` attribute to determine it's size before compilation",
    )
}

#[inline]
pub fn list_attribute_usage<T: ToTokens>(name: &Ident, tokens: T) -> syn::Error {
    syn::Error::new_spanned(tokens, format!("the `{name}` attribute should be a list"))
}

#[inline]
pub(crate) fn bool_attribute_usage<T: ToTokens>(name: &Ident, tokens: T) -> syn::Error {
    syn::Error::new_spanned(
        tokens,
        format!("the `{name}` attribute should be a name-value pair. The value type is boolean"),
    )
}

#[inline]
pub(crate) fn sub_attributes_for_ordinalize<T: ToTokens>(tokens: T) -> syn::Error {
    syn::Error::new_spanned(
        tokens,
        format!(
            "available sub-attributes for the `ordinalize` attribute:{}",
            DisplayStringSlice(&[
                "impl_trait",
                "variant_count",
                "variants",
                "values",
                "ordinal",
                "from_ordinal_unsafe",
                "from_ordinal",
            ])
        ),
    )
}
