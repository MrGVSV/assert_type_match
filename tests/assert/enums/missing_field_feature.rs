use assert_type_match::assert_type_match;

enum OtherType {
    Unit,
    Tuple(i32, #[cfg(feature = "foo")] i32),
    Struct {
        x: i32,
        #[cfg(feature = "foo")]
        y: i32,
    },
}

#[assert_type_match(OtherType)]
//~^ ERROR: variant `OtherType::Tuple` has no field named `1`
enum Test {
    Unit,
    Tuple(i32, i32),
    Struct { x: i32, y: i32 },
    //~^ ERROR: variant `OtherType::Struct` has no field named `y`
}

fn main() {}
