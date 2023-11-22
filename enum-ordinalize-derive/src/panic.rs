use proc_macro2::Span;

#[inline]
pub fn not_enum(span: Span) -> syn::Error {
    syn::Error::new(span, "Only enums can be ordinalized.")
}

#[inline]
pub fn no_variant(span: Span) -> syn::Error {
    syn::Error::new(span, "An ordinalized enum needs to have at least one variant.")
}

#[inline]
pub fn not_unit_variant(span: Span) -> syn::Error {
    syn::Error::new(span, "An ordinalized enum can only have unit variants.")
}

#[inline]
pub fn unsupported_discriminant(span: Span) -> syn::Error {
    syn::Error::new(
        span,
        "The discriminant of a variant of an ordinalized enum needs to be a legal literal \
         integer, a constant variable/function or a constant expression.",
    )
}
#[inline]
pub fn constant_variable_on_non_determined_size_enum(span: Span) -> syn::Error {
    syn::Error::new(
        span,
        "The discriminant of a variant can be assigned not to a literal integer only when the \
         ordinalized enum is using the `repr` attribute to determine it's size before compilation.",
    )
}

#[inline]
pub fn ordinalize_attribute_usage(span: Span) -> syn::Error {
    syn::Error::new(
        span,
        "The `ordinalize` attribute should be written like: #[ordinalize(variants(pub fn \
         variants, doc = \"Returns an array of variants.\"))]",
    )
}
