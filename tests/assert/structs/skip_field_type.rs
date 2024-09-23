//@check-pass
use assert_type_match::assert_type_match;

mod other {
    pub struct Test {
        pub x: i32,
        pub y: i32,
    }
}

#[assert_type_match(other::Test)]
struct Test {
    x: i32,
    #[assert_type_match(skip_type)]
    y: f32,
}

fn main() {}
