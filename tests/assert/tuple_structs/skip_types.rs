//@check-pass
use assert_type_match::assert_type_match;

struct OtherType(i32, i32);

#[assert_type_match(OtherType, skip_types)]
struct Test(f32, f32);

fn main() {}
