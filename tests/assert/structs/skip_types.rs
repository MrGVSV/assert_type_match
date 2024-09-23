//@check-pass
use assert_type_match::assert_type_match;

struct OtherType {
    x: i32,
    y: i32,
}

#[assert_type_match(OtherType, skip_types)]
struct Test {
    x: f32,
    y: f32,
}

fn main() {}
