use enum_ordinalize::Ordinalize;

#[derive(Ordinalize)]
enum NonUnitVariant {
    A(u8),
}

fn main() {}
