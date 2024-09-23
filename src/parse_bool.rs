use syn::parse::{Parse, ParseStream};
use syn::spanned::Spanned;
use syn::{LitBool, Token};

/// Extension trait for smartly parsing a boolean value from a [`ParseStream`].
pub(crate) trait ParseBool {
    /// Parse the next token(s) in the stream as a boolean value.
    ///
    /// Accepts either:
    /// - An instance of `T`
    /// - An instance of `T` followed by an equals sign and a boolean literal
    ///
    /// For example, both `foo` and `foo = true` will evaluate to `true`.
    /// Whereas, `foo = false` will evaluate to `false`.
    fn parse_bool<T: Parse + Spanned>(&self) -> syn::Result<bool>;
    /// Parse the next token(s) in the stream as a boolean value.
    ///
    /// This is similar to [`ParseBool::parse_bool`], but it returns a [`LitBool`] instead.
    fn parse_lit_bool<T: Parse + Spanned>(&self) -> syn::Result<LitBool>;
}

impl<'a> ParseBool for ParseStream<'a> {
    fn parse_bool<T: Parse + Spanned>(&self) -> syn::Result<bool> {
        self.parse_lit_bool::<T>().map(|lit| lit.value)
    }

    fn parse_lit_bool<T: Parse + Spanned>(&self) -> syn::Result<LitBool> {
        let keyword = self.parse::<T>()?;
        if self.peek(Token![=]) {
            self.parse::<Token![=]>()?;
            Ok(self.parse::<LitBool>()?)
        } else {
            Ok(LitBool::new(true, keyword.span()))
        }
    }
}

pub(crate) fn merge_flags(
    name: &str,
    base: Option<LitBool>,
    other: Option<LitBool>,
) -> syn::Result<Option<LitBool>> {
    match (base, other) {
        (Some(a), Some(b)) if a.value == b.value => Ok(Some(a)),
        (Some(a), None) => Ok(Some(a)),
        (None, Some(b)) => Ok(Some(b)),
        (None, None) => Ok(None),
        (Some(_), Some(b)) => Err(syn::Error::new_spanned(
            b,
            format_args!("conflicting `{}` arguments", name),
        )),
    }
}
