use assert_type_match::assert_type_match;

union OtherType {
    x: i32,
    y: i32,
}

#[assert_type_match(OtherType)]
//~v ERROR: unions are not supported
union Test {
    x: i32,
    y: i32,
}

fn main() {}
