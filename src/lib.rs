mod args;
mod parse_bool;

use proc_macro::TokenStream;
use quote::{format_ident, quote, ToTokens};
use syn::{parse_macro_input, Attribute, Data, DeriveInput, Field, Member};
use syn::spanned::Spanned;
use crate::args::Args;

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
/// ## `check_name`
///
/// Type: `bool`
///
/// Controls whether the name of the annotated struct or enum should be compared to
/// the name of the foreign type.
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
/// struct OtherType {
///     x: i32,
///     y: i32,
/// }
///
/// #[assert_type_match(OtherType)]
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
/// struct OtherType {
///     x: i32,
///     y: i32,
/// }
///
/// #[assert_type_match(OtherType)]
/// struct Test {
///     x: i32,
///     z: i32, // Error: struct `OtherType` has no field named `z`
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

    let foreign_ty = args.foreign_ty();

    let ident = &input.ident;

    if args.check_name() {
        let Some(segment) = foreign_ty.path.segments.last() else {
            return syn::Error::new(foreign_ty.span(), "expected a type path").to_compile_error().into();
        };

        if &segment.ident != ident {
            return syn::Error::new(ident.span(), format_args!("type name does not match: expected `{}`", segment.ident.to_string())).to_compile_error().into();
        }
    }


    let (impl_generics, ty_generics, where_clause) = input.generics.split_for_impl();

    let this = format_ident!("this");

    let assertions = match &input.data {
        Data::Struct(data) => {
            let fields = data.fields.iter().enumerate().map(|(index, field)| {
                let member = create_member(field, index);
                let attrs = extract_cfg_attrs(&field.attrs);

                quote! {
                    #( #attrs )*
                    #member: #this.#member
                }
            });

            let (fn_name, this_ty) = if args.skip_types() {
                (format_ident!("__assert_untyped_fields_match"), foreign_ty.to_token_stream())
            } else {
                (format_ident!("__assert_typed_fields_match"), ident.to_token_stream())
            };

            quote! {
                fn #fn_name #impl_generics(#this: #this_ty #ty_generics) -> #foreign_ty #ty_generics #where_clause {
                     #foreign_ty {
                        #(#fields),*
                    }
                }
            }
        }
        Data::Enum(data) => {
            let (fn_name, this_ty) = if args.skip_types() {
                (format_ident!("__assert_untyped_variants_match"), foreign_ty.to_token_stream())
            } else {
                (format_ident!("__assert_typed_variants_match"), ident.to_token_stream())
            };

            let this_to_foreign = data.variants.iter().map(|variant| {
                let variant_ident = &variant.ident;

                let (field_decls, field_ctors): (Vec<_>, Vec<_>) = variant.fields.iter().enumerate().map(|(index, field)| {
                    let member = create_member(field, index);
                    let decl_attrs = extract_cfg_attrs(&field.attrs);
                    let ctor_attrs = extract_cfg_attrs(&field.attrs);

                    let alias = format_ident!("__{}", member);

                    (
                        quote!(#( #decl_attrs )* #member: #alias),
                        quote! {
                            #( #ctor_attrs )*
                            #member: #alias
                        }
                    )
                }).unzip();

                let attrs = extract_cfg_attrs(&variant.attrs);

                quote! {
                    #( #attrs )*
                    #this_ty::#variant_ident { #(#field_decls,)* .. } => #foreign_ty::#variant_ident { #(#field_ctors),* }
                }
            });

            let foreign_to_this = data.variants.iter().map(|variant| {
                let variant_ident = &variant.ident;
                let attrs = extract_cfg_attrs(&variant.attrs);

                quote! {
                    #( #attrs )*
                    #foreign_ty::#variant_ident { .. } => {}
                }
            });

            quote! {
                fn #fn_name #impl_generics(#this: #this_ty #ty_generics) -> #foreign_ty #ty_generics #where_clause {
                     match #this {
                        #(#this_to_foreign),*
                    }
                }

                // This test is needed to ensure that all variants in the foreign enum exist in the input enum
                fn __assert_all_variants_exist #impl_generics(#this: #foreign_ty #ty_generics) #where_clause {
                     match #this {
                        #(#foreign_to_this),*
                    }
                }
            }
        }
        Data::Union(data) => {
            syn::Error::new(data.union_token.span, "unions are not supported").to_compile_error()
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

fn extract_cfg_attrs(attrs: &[Attribute]) -> impl Iterator<Item=&Attribute> {
    attrs.iter().filter(|attr| attr.path().is_ident("cfg"))
}

fn create_member(field: &Field, index: usize) -> Member {
    field.ident.clone().map(Member::from).unwrap_or_else(|| Member::from(index))
}
