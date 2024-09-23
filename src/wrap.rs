use crate::args::Args;
use crate::enums::enum_from;
use crate::structs::struct_from;
use crate::ATTRIBUTE;
use proc_macro2::TokenStream;
use quote::{quote, ToTokens};
use syn::{Data, DeriveInput};

/// Wraps the given assertions in a `const` block.
///
/// This also handles the removal of nested `assert_type_match` attributes
/// and the presence of the `test_only` attribute.
pub(crate) fn wrap_assertions<F>(mut input: DeriveInput, args: Args, f: F) -> TokenStream
where
    F: FnOnce(&DeriveInput, &Args) -> syn::Result<TokenStream>,
{
    let assertions = match f(&input, &args) {
        Ok(assertions) => assertions,
        Err(err) => return err.to_compile_error(),
    };

    let from_impl = match generate_from(&input, &args) {
        Ok(from_impl) => from_impl,
        Err(err) => return err.to_compile_error(),
    };

    let mut output = if args.test_only() {
        TokenStream::new()
    } else {
        // Remove all nested `assert_type_match` attributes
        strip_assertion_attributes(&mut input);

        input.to_token_stream()
    };

    if args.test_only() {
        output.extend(quote! {
            const _: () = {
                #input
                #assertions
                #from_impl
            };
        });
    } else {
        output.extend(quote! {
            const _: () = {
                #assertions
                #from_impl
            };
        });
    }

    output
}

/// Removes all `assert_type_match` attributes from the given input.
fn strip_assertion_attributes(input: &mut DeriveInput) {
    match &mut input.data {
        Data::Struct(data) => {
            for field in data.fields.iter_mut() {
                field.attrs.retain(|attr| !attr.path().is_ident(ATTRIBUTE));
            }
        }
        Data::Enum(data) => {
            for variant in data.variants.iter_mut() {
                variant
                    .attrs
                    .retain(|attr| !attr.path().is_ident(ATTRIBUTE));

                for field in variant.fields.iter_mut() {
                    field.attrs.retain(|attr| !attr.path().is_ident(ATTRIBUTE));
                }
            }
        }
        _ => {}
    }
}

fn generate_from(input: &DeriveInput, args: &Args) -> syn::Result<TokenStream> {
    if !args.from() {
        return Ok(TokenStream::new());
    }

    match &input.data {
        Data::Struct(data) => struct_from(data, input, args),
        Data::Enum(data) => enum_from(data, input, args),
        Data::Union(data) => Err(syn::Error::new(
            data.union_token.span,
            "unions are not supported",
        )),
    }
}
