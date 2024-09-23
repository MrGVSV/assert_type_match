use syn::parse::{Parse, ParseStream};
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
    fn parse_bool<T: Parse>(&self) -> syn::Result<bool>;
}

impl<'a> ParseBool for ParseStream<'a> {
    fn parse_bool<T: Parse>(&self) -> syn::Result<bool> {
        self.parse::<T>()?;
        if self.peek(Token![=]) {
            self.parse::<Token![=]>()?;
            Ok(self.parse::<LitBool>()?.value)
        } else {
            Ok(true)
        }
    }
}