#[derive(Debug, PartialEq, Eq, enum_ordinalize::Ordinalize)]
enum PathDerive {
    A,
    B,
}

fn main() {
    assert_eq!(2, <PathDerive as enum_ordinalize::Ordinalize>::VARIANT_COUNT);
    assert_eq!(Some(PathDerive::A), <PathDerive as enum_ordinalize::Ordinalize>::from_ordinal(0i8));
}
