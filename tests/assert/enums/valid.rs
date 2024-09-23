//@check-pass
use assert_type_match::assert_type_match;

enum OtherType {
    Unit,
    Tuple(i32, i32),
    Struct { x: i32, y: i32 },
    #[cfg(feature = "foo")]
    FeatureVariant(i32),
    FeatureField(#[cfg(feature = "foo")] i32),
}

#[assert_type_match(OtherType)]
enum Test {
    Unit,
    Tuple(i32, i32),
    Struct { x: i32, y: i32 },
    #[cfg(feature = "foo")]
    FeatureVariant(i32),
    FeatureField(#[cfg(feature = "foo")] i32),
}

fn main() {}