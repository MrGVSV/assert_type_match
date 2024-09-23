use assert_type_match::assert_type_match;

enum OtherType {
    #[cfg(feature = "foo")]
    Unit,
    Tuple(i32, i32),
    Struct { x: i32, y: i32 },
}

#[assert_type_match(OtherType)]
enum Test {
    Unit,
    //~^ ERROR: no variant named `Unit` found for enum `OtherType`
    Tuple(i32, i32),
    Struct { x: i32, y: i32 },
}

fn main() {}