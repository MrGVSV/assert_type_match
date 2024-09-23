use crate::args::Args;
use crate::utils::{create_member, extract_cfg_attrs, extract_field_args, try_unzip};
use proc_macro2::TokenStream;
use quote::{format_ident, quote, ToTokens};
use syn::{DataEnum, DeriveInput};

pub(crate) fn enum_assert(
    data: &DataEnum,
    input: &DeriveInput,
    args: &Args,
) -> syn::Result<TokenStream> {
    let ident = &input.ident;
    let foreign_ty = args.foreign_ty();
    let (impl_generics, ty_generics, where_clause) = input.generics.split_for_impl();

    let this = format_ident!("this");

    let (fn_name, this_ty) = if args.skip_types() {
        (
            format_ident!("__assert_untyped_variants_match"),
            foreign_ty.to_token_stream(),
        )
    } else {
        (
            format_ident!("__assert_typed_variants_match"),
            ident.to_token_stream(),
        )
    };

    let this_to_foreign = data.variants.iter().map(|variant| {
        let variant_ident = &variant.ident;

        let iter = variant.fields.iter().enumerate().filter_map(|(index, field)| {
            let member = create_member(field, index);
            let decl_attrs = extract_cfg_attrs(&field.attrs);
            let ctor_attrs = extract_cfg_attrs(&field.attrs);

            let field_args = match extract_field_args(&field.attrs) {
                Ok(args) => args,
                Err(err) => return Some(Err(err)),
            };

            if field_args.skip() {
                return None;
            }

            let alias = format_ident!("__{}", member);

            let value = if field_args.skip_type() {
                quote! { (|| { unreachable!() })() }
            } else {
                quote! { #alias }
            };

            Some(Ok((
                quote!(#( #decl_attrs )* #member: #alias),
                quote! {
                    #( #ctor_attrs )*
                    #member: #value
                }
            )))
        });

        let (field_decls, field_ctors): (Vec<_>, Vec<_>) = try_unzip(iter)?;

        let attrs = extract_cfg_attrs(&variant.attrs);

        Ok(quote! {
            #( #attrs )*
            #this_ty::#variant_ident { #(#field_decls,)* .. } => #foreign_ty::#variant_ident { #(#field_ctors),* },
        })
    }).collect::<syn::Result<TokenStream>>()?;

    let foreign_to_this = data.variants.iter().map(|variant| {
        let variant_ident = &variant.ident;
        let attrs = extract_cfg_attrs(&variant.attrs);

        quote! {
            #( #attrs )*
            #foreign_ty::#variant_ident { .. } => {}
        }
    });

    Ok(quote! {
        fn #fn_name #impl_generics(#this: #this_ty #ty_generics) -> #foreign_ty #ty_generics #where_clause {
             match #this {
                #this_to_foreign
            }
        }

        // This test is needed to ensure that all variants in the foreign enum exist in the input enum
        fn __assert_all_variants_exist #impl_generics(#this: #foreign_ty #ty_generics) #where_clause {
             match #this {
                #(#foreign_to_this),*
            }
        }
    })
}
