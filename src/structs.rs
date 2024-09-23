use crate::args::Args;
use crate::utils::{create_member, extract_cfg_attrs};
use proc_macro2::TokenStream;
use quote::{format_ident, quote, ToTokens};
use syn::{DataStruct, DeriveInput};

pub(crate) fn struct_assert(data: &DataStruct, input: &DeriveInput, args: &Args) -> TokenStream {
    let ident = &input.ident;
    let foreign_ty = args.foreign_ty();
    let (impl_generics, ty_generics, where_clause) = input.generics.split_for_impl();

    let this = format_ident!("this");

    let fields = data.fields.iter().enumerate().map(|(index, field)| {
        let member = create_member(field, index);
        let attrs = extract_cfg_attrs(&field.attrs);

        quote! {
            #( #attrs )*
            #member: #this.#member
        }
    });

    let (fn_name, this_ty) = if args.skip_types() {
        (
            format_ident!("__assert_untyped_fields_match"),
            foreign_ty.to_token_stream(),
        )
    } else {
        (
            format_ident!("__assert_typed_fields_match"),
            ident.to_token_stream(),
        )
    };

    quote! {
        fn #fn_name #impl_generics(#this: #this_ty #ty_generics) -> #foreign_ty #ty_generics #where_clause {
             #foreign_ty {
                #(#fields),*
            }
        }
    }
}
