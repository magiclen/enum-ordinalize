#[derive(Debug, PartialEq, Eq, enum_ordinalize::Ordinalize)]
#[repr(u8)]
#[ordinalize(impl_trait = false)]
#[ordinalize(variant_count(pub const VARIANT_COUNT))]
enum AttributeOrder {
    A,
    B,
}

fn main() {
    assert_eq!(2, AttributeOrder::VARIANT_COUNT);
}
