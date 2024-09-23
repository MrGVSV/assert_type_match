# assert_type_match

[![Crates.io](https://img.shields.io/crates/v/assert_type_match)](https://crates.io/crates/assert_type_match)
[![Docs](https://img.shields.io/docsrs/assert_type_match)](https://docs.rs/assert_type_match/)
[![License](https://img.shields.io/crates/l/assert_type_match)](./LICENSE.md)

A niche utility macro to statically assert that a type matches another type.

## Purpose

The primary purpose of this crate is to make copying and pasting types from
other crates into your own more future-proof by statically asserting that the
types match.

This situation happens sometimes when you want to add your own methods or documentation
on a foreign type.

By using this crate, you can ensure that changes made upstream will be caught
by the compiler, so you can update your code accordingly.

## Usage

```rust
// Pretend this type comes from a foreign crate:
mod foreign_crate {
    pub enum ColorSpace {
        Rgb,
        Rgba,
        Cmyk,
    }
}

mod my_crate {
    use assert_type_match::assert_type_match;

    // We can add our own trait implementations and documentation:
    #[derive(Default)]
    #[assert_type_match(foreign_crate::ColorSpace)]
    pub enum ColorSpace {
        #[default]
        Rgb,
        Rgba,
        Cmyk,
    }
}
```

If `foreign_crate::ColorSpace` ever changes, the compiler will catch it.

For example, if `foreign_crate::ColorSpace` adds a new variant `Hsla`, we'll get the following error:

```
error[E0004]: non-exhaustive patterns: `foreign_crate::ColorSpace::Hsla` not covered
```

### Configuration

The behavior of the macro can be configured.

One common pattern is implementing `From` to convert between the types.
This can be automatically generated by setting the `from` attribute:

```rust
#[assert_type_match(foreign_crate::ColorSpace, from)]
pub enum ColorSpace {
    Rgb,
    Rgba,
    Cmyk,
}

let my_space: my_crate::ColorSpace = my_crate::ColorSpace::Rgb;
// Convert to the foreign type:
let foreign_space: foreign_crate::ColorSpace = my_space.into();
// And back again:
let my_space: my_crate::ColorSpace = foreign_space.into();
```

There are other attributes available, such as `test_only` and `skip_name`,
as well as attributes to control the behavior of specific fields and variants.

See the [docs](https://docs.rs/assert_type_match/) for more information.