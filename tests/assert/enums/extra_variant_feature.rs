use assert_type_match::assert_type_match;

enum OtherType {
    Unit,
    Tuple(i32, i32),
    Struct { x: i32, y: i32 },
}

#[assert_type_match(OtherType)]
//~^ ERROR: non-exhaustive patterns: `OtherType::Unit` not covered
enum Test {
    #[cfg(feature = "foo")]
    Unit,
    Tuple(i32, i32),
    Struct {
        x: i32,
        y: i32,
    },
}

fn main() {}
