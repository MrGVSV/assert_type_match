//@check-pass
use assert_type_match::assert_type_match;

struct OtherType(i32, i32);

#[assert_type_match(OtherType)]
struct Test(i32, i32);

fn main() {}
