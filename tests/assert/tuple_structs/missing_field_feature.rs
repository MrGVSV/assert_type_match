use assert_type_match::assert_type_match;

struct OtherType(i32, #[cfg(feature = "foo")] i32);

#[assert_type_match(OtherType)]
//~^ ERROR: struct `OtherType` has no field named `1`
struct Test(i32, i32);

fn main() {}
