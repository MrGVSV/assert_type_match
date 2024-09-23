use assert_type_match::assert_type_match;

enum OtherType {
    Unit,
    Tuple(i32, i32),
    Struct { x: i32, y: i32 },
}

#[assert_type_match(OtherType)]
//~^ ERROR: mismatched types
enum Test {
    Unit,
    Tuple(i32, f32),
    Struct { x: i32, y: f32 },
    //~^ ERROR: mismatched types
}

fn main() {}
