use assert_type_match::assert_type_match;

struct OtherType {
    x: i32,
    y: i32,
}

#[assert_type_match(OtherType, test_only)]
struct Test {
    x: i32,
    y: i32,
}

type Test2 = Test;
//~^ ERROR: cannot find type `Test` in this scope

fn main() {}
