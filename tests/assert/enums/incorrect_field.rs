use assert_type_match::assert_type_match;

enum OtherType {
    Unit,
    Tuple(i32, i32),
    Struct { x: i32, y: i32 },
}

#[assert_type_match(OtherType)]
enum Test {
    Unit,
    Tuple(i32, i32),
    Struct { x: i32, z: i32 },
    //~^ ERROR: variant `OtherType::Struct` has no field named `z`
}

fn main() {}