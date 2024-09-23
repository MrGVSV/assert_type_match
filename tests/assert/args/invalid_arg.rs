use assert_type_match::assert_type_match;

struct OtherType {
    x: i32,
    y: i32,
}

#[assert_type_match(OtherType, foo)]
//~^ ERROR: expected `test_only`
struct Test {
    x: i32,
    y: i32,
}

fn main() {}