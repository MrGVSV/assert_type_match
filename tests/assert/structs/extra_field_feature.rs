use assert_type_match::assert_type_match;

struct OtherType {
    x: i32,
    y: i32,
}

#[assert_type_match(OtherType)]
//~^ ERROR: missing field `y` in initializer of `OtherType`
struct Test {
    x: i32,
    #[cfg(feature = "foo")]
    y: i32,
}

fn main() {}