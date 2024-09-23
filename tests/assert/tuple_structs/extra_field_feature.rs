use assert_type_match::assert_type_match;

struct OtherType(i32, i32);

#[assert_type_match(OtherType)]
//~^ ERROR: missing field `1` in initializer of `OtherType`
struct Test(i32, #[cfg(feature = "foo")] i32);

fn main() {}
