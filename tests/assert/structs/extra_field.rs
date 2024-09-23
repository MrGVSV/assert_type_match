use assert_type_match::assert_type_match;

struct OtherType {
    x: i32,
    y: i32,
}

#[assert_type_match(OtherType)]
struct Test {
    x: i32,
    y: i32,
    z: i32,
    //~^ ERROR: struct `OtherType` has no field named `z`
}

fn main() {}
