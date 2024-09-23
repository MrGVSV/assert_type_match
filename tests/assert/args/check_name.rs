use assert_type_match::assert_type_match;

struct OtherType {
    x: i32,
    y: i32,
}

#[assert_type_match(OtherType, check_name)]
//~v ERROR: type name does not match: expected `OtherType`
struct Test {
    x: i32,
    y: i32,
}

fn main() {}