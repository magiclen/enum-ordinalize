use enum_ordinalize::Ordinalize;

#[derive(Ordinalize)]
#[ordinalize(unknown)]
enum BadAttribute {
    A,
}

fn main() {}
