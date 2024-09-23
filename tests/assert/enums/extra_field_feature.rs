use assert_type_match::assert_type_match;

enum OtherType {
    Unit,
    Tuple(i32, i32),
    Struct { x: i32, y: i32 },
}

#[assert_type_match(OtherType)]
//~^ ERROR: missing field `1` in initializer of `OtherType`
//~| ERROR: missing field `y` in initializer of `OtherType`
enum Test {
    Unit,
    Tuple(i32, #[cfg(feature = "foo")] i32),
    Struct { x: i32, #[cfg(feature = "foo")] y: i32 },
}

fn main() {}