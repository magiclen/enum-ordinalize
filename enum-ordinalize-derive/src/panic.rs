use proc_macro2::Span;
use syn::Ident;

#[inline]
pub fn not_enum(span: Span) -> syn::Error {
    syn::Error::new(span, "only enums can be ordinalized")
}

#[inline]
pub fn no_variant(span: Span) -> syn::Error {
    syn::Error::new(span, "an ordinalized enum needs to have at least one variant")
}

#[inline]
pub fn not_unit_variant(span: Span) -> syn::Error {
    syn::Error::new(span, "an ordinalized enum can only have unit variants")
}

#[inline]
pub fn unsupported_discriminant(span: Span) -> syn::Error {
    syn::Error::new(
        span,
        "the discriminant of a variant of an ordinalized enum needs to be a legal literal \
         integer, a constant variable/function or a constant expression",
    )
}
#[inline]
pub fn constant_variable_on_non_determined_size_enum(span: Span) -> syn::Error {
    syn::Error::new(
        span,
        "the discriminant of a variant can be assigned not to a literal integer only when the \
         ordinalized enum is using the `repr` attribute to determine it's size before compilation",
    )
}

#[inline]
pub fn list_attribute_usage(name: &Ident, span: Span) -> syn::Error {
    syn::Error::new(span, format!("the \"{name}\" attribute should be a list", name = name))
    // use `name = name` to support Rust 1.56
}

#[inline]
pub fn bool_attribute_usage(name: &Ident, span: Span) -> syn::Error {
    syn::Error::new(
        span,
        format!(
            "the \"{name}\" attribute should be a name-value pair. The value type is boolean",
            name = name
        ),
    )
    // use `name = name` to support Rust 1.56
}

#[inline]
pub fn sub_attributes_for_ordinalize(span: Span) -> syn::Error {
    syn::Error::new(
        span,
        format!("Available sub-attributes for the `ordinalize` attribute:\n{:#?}", [
            "impl_trait",
            "variant_count",
            "variants",
            "values",
            "ordinal",
            "from_ordinal_unsafe",
            "from_ordinal",
        ]),
    )
}
