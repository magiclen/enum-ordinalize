use enum_ordinalize::Ordinalize;

const VALUE: isize = 1;

#[derive(Ordinalize)]
enum ConstantWithoutRepr {
    A = VALUE,
}

fn main() {}
