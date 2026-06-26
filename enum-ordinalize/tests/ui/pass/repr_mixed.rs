#![allow(conflicting_repr_hints)]

use enum_ordinalize::Ordinalize;

#[derive(Debug, PartialEq, Eq, Ordinalize)]
#[repr(C, u8)]
enum CFirst {
    A = 1,
    B,
}

#[derive(Debug, PartialEq, Eq, Ordinalize)]
#[repr(u16, C)]
enum IntegerFirst {
    A = 300,
    B,
}

fn main() {
    assert_eq!([1u8, 2u8], CFirst::VALUES);
    assert_eq!([300u16, 301u16], IntegerFirst::VALUES);
}
