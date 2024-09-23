//@check-pass
use assert_type_match::assert_type_match;

mod other {
    pub enum Test {
        Unit,
        Tuple(i32, i32),
        Struct {
            x: i32,
            y: i32,
        },
        #[cfg(feature = "foo")]
        FeatureVariant(i32),
        FeatureField(#[cfg(feature = "foo")] i32),
    }
}

#[assert_type_match(other::Test, from)]
enum Test {
    Unit,
    Tuple(i32, i32),
    Struct {
        x: i32,
        y: i32,
    },
    #[cfg(feature = "foo")]
    FeatureVariant(i32),
    FeatureField(#[cfg(feature = "foo")] i32),
}

fn convert_this(test: Test) -> other::Test {
    test.into()
}

fn convert_that(test: other::Test) -> Test {
    test.into()
}

fn main() {}