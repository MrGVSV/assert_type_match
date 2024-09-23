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
    Struct { x: i32, y: i32 },
    Extra,
    //~^ ERROR: no variant named `Extra` found for enum `OtherType`
}

fn main() {}