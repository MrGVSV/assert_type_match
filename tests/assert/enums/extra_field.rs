use assert_type_match::assert_type_match;

enum OtherType {
    Unit,
    Tuple(i32, i32),
    Struct { x: i32, y: i32 },
}

#[assert_type_match(OtherType)]
//~^ ERROR: variant `OtherType::Unit` has no field named `0`
//~| ERROR: variant `OtherType::Tuple` has no field named `2`
enum Test {
    Unit(i32),
    Tuple(i32, i32, i32),
    Struct { x: i32, y: i32, z: i32 },
    //~^ ERROR: variant `OtherType::Struct` has no field named `z`
}

fn main() {}
