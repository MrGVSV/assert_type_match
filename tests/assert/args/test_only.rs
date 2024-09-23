use assert_type_match::assert_type_match;

mod other {
    pub struct Test {
       pub x: i32,
       pub y: i32,
    }
}

#[assert_type_match(other::Test, test_only)]
struct Test {
    x: i32,
    y: i32,
}

type Test2 = Test;
//~^ ERROR: cannot find type `Test` in this scope

fn main() {}
