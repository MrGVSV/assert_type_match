use crate::parse_bool::ParseBool;
use syn::parse::{Parse, ParseStream};
use syn::{Token, TypePath};

/// The arguments for the `assert_type_match` attribute macro.
pub(crate) struct Args {
    foreign_ty: TypePath,
    test_only: Option<bool>,
}

impl Args {
    /// Get the foreign type that the annotated struct or enum should match.
    pub fn foreign_ty(&self) -> &TypePath {
        &self.foreign_ty
    }

    /// Controls whether to output the annotated struct or enum in the generated code.
    pub fn test_only(&self) -> bool {
        self.test_only.unwrap_or_default()
    }
}

impl Parse for Args {
    fn parse(input: ParseStream) -> syn::Result<Self> {
        let foreign_ty = input.parse().map_err(|err| {
            syn::Error::new(err.span(), "expected the type path to a foreign type (e.g. `#[assert_type_match(path::to::ForeignType)]`)")
        })?;

        let mut this = Self {
            foreign_ty,
            test_only: None,
        };

        while input.peek(Token![,]) {
            input.parse::<Token![,]>()?;

            if input.is_empty() {
                break;
            }

            let lookahead = input.lookahead1();
            if lookahead.peek(kw::test_only) {
                this.test_only = Some(input.parse_bool::<kw::test_only>()?);
            } else {
                return Err(lookahead.error());
            }
        }

        Ok(this)
    }
}

mod kw {
    syn::custom_keyword!(test_only);
}