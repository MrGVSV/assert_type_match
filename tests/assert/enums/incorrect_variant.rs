use assert_type_match::assert_type_match;

enum OtherType {
    Unit,
    Tuple(i32, i32),
    Struct { x: i32, y: i32 },
}

#[assert_type_match(OtherType)]
enum Test {
    Empty,
    //~^ ERROR: no variant named `Empty` found for enum `OtherType`
    Tuple(i32, i32),
    Struct { x: i32, y: i32 },
}

fn main() {}
