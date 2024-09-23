mod args;
mod enums;
mod parse_bool;
mod structs;
mod utils;

use crate::args::Args;
use crate::enums::enum_assert;
use crate::structs::struct_assert;
use proc_macro::TokenStream;
use quote::quote;
use syn::spanned::Spanned;
use syn::{parse_macro_input, Data, DeriveInput};

/// An attribute macro that can be used to statically verify that the annotated struct or enum
/// matches the structure of a foreign type.
///
/// For structs, it will verify that the fields of the annotated struct match the fields of the
/// foreign struct.
///
/// For enums, it will verify that the variants of the annotated enum match the variants of the
/// foreign enum, and that the fields of each variant match the fields of the corresponding variant
/// in the foreign enum.
///
/// This will also output the original annotated struct or enum,
/// unless the `test_only` argument is set to `true`.
///
/// # Arguments
///
/// This macro accepts arguments to control its behavior.
/// These arguments are passed as a comma-separated list after the foreign type.
///
/// All boolean arguments may be set to true by using either the `foo` or `foo = true` syntax.
///
/// ## `test_only`
///
/// Type: `bool`
///
/// Controls whether to output the annotated struct or enum in the generated code.
///
/// ## `skip_name`
///
/// Type: `bool`
///
/// Controls whether checking that the name of the annotated struct or enum matches
/// the name of the foreign type should be skipped.
///
/// For example, comparing `struct Foo(u32)` to `struct Bar(u32)` would pass
/// when this argument is set to `true`.
///
/// ## `skip_types`
///
/// Type: `bool`
///
/// Controls whether checking field types should be skipped.
///
/// For example, comparing `struct Foo(i32)` to `struct Foo(f32)` would pass
/// when this argument is set to `true`.
///
/// # Example
///
/// A passing example:
///
/// ```
/// # use assert_type_match::assert_type_match;
/// mod other {
///     pub struct Test {
///         pub x: i32,
///         pub y: i32,
///     }
/// }
///
/// #[assert_type_match(other::Test)]
/// struct Test {
///     x: i32,
///     y: i32,
/// }
/// ```
///
/// A failing example:
///
/// ```compile_fail
/// # use assert_type_match::assert_type_match;
/// mod other {
///     pub struct Test {
///         pub x: i32,
///         pub y: i32,
///     }
/// }
///
/// #[assert_type_match(other::Test)]
/// struct Test {
///     x: i32,
///     z: i32, // Error: struct `other::Test` has no field named `z`
/// }
/// ```
///
#[proc_macro_attribute]
pub fn assert_type_match(args: TokenStream, input: TokenStream) -> TokenStream {
    let args = parse_macro_input!(args as Args);

    let mut output = if args.test_only() {
        TokenStream::new()
    } else {
        input.clone()
    };

    let input = parse_macro_input!(input as DeriveInput);

    let ident = &input.ident;
    let foreign_ty = args.foreign_ty();

    if !args.skip_name() {
        let Some(segment) = foreign_ty.path.segments.last() else {
            return syn::Error::new(foreign_ty.span(), "expected a type path")
                .to_compile_error()
                .into();
        };

        if &segment.ident != ident {
            return syn::Error::new(
                ident.span(),
                format_args!("type name does not match: expected `{}`", segment.ident),
            )
            .to_compile_error()
            .into();
        }
    }

    let assertions = match &input.data {
        Data::Struct(data) => struct_assert(data, &input, &args),
        Data::Enum(data) => enum_assert(data, &input, &args),
        Data::Union(data) => {
            return syn::Error::new(data.union_token.span, "unions are not supported")
                .to_compile_error()
                .into();
        }
    };

    if args.test_only() {
        output.extend(TokenStream::from(quote! {
            const _: () = {
                #input
                #assertions
            };
        }));
    } else {
        output.extend(TokenStream::from(quote! {
            const _: () = {
                #assertions
            };
        }));
    }

    output
}
