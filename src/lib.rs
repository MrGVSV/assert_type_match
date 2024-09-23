use proc_macro::TokenStream;
use quote::{format_ident, quote};
use syn::{parse_macro_input, Attribute, Data, DeriveInput, Field, Member, TypePath};

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
    let mut original_input = input.clone();

    let input = parse_macro_input!(input as DeriveInput);
    let foreign_ty = parse_macro_input!(args as TypePath);

    let ident = &input.ident;
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

            quote! {
                fn assert_type_matches #impl_generics(#this: #ident #ty_generics) -> #foreign_ty #ty_generics #where_clause {
                     #foreign_ty {
                        #(#fields),*
                    }
                }
            }
        }
        Data::Enum(data) => {
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
                    #ident::#variant_ident { #(#field_decls,)* .. } => #foreign_ty::#variant_ident { #(#field_ctors),* }
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
                fn assert_type_matches #impl_generics(#this: #ident #ty_generics) -> #foreign_ty #ty_generics #where_clause {
                     match #this {
                        #(#this_to_foreign),*
                    }
                }

                // This test is needed to ensure that all variants in the foreign enum exist in the input enum
                fn assert_all_variants_exist #impl_generics(#this: #foreign_ty #ty_generics) #where_clause {
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

    original_input.extend(TokenStream::from(assertions));

    original_input
}

fn extract_cfg_attrs(attrs: &[Attribute]) -> impl Iterator<Item=&Attribute> {
    attrs.iter().filter(|attr| attr.path().is_ident("cfg"))
}

fn create_member(field: &Field, index: usize) -> Member {
    field.ident.clone().map(Member::from).unwrap_or_else(|| Member::from(index))
}
