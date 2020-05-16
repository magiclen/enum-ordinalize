#[inline]
pub fn not_enum() -> ! {
    panic!("Only enums can be ordinalized.")
}

#[inline]
pub fn not_unit_variant() -> ! {
    panic!("An ordinalized enum can only have unit variants.")
}

#[inline]
pub fn no_variant() -> ! {
    panic!("An ordinalized enum needs to have at least one variant.")
}

#[inline]
pub fn unsupported_discriminant() -> ! {
    panic!(
        "The discriminant of a variant of an ordinalized enum needs to be a legal literal integer."
    )
}
