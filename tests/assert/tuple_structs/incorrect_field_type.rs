use assert_type_match::assert_type_match;

struct OtherType(i32, i32);

#[assert_type_match(OtherType)]
//~^ ERROR: mismatched types
struct Test(i32, f32);

fn main() {}
