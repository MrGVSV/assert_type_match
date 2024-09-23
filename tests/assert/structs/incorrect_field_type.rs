use assert_type_match::assert_type_match;

struct OtherType {
    x: i32,
    y: i32,
}

#[assert_type_match(OtherType)]
//~^ ERROR:  mismatched types
struct Test {
    x: i32,
    y: f32,
}

fn main() {}