extern crate proc_macro;

use proc_macro::TokenStream;
use quote::quote;
use syn::{parse_macro_input, DeriveInput};

#[proc_macro_derive(ErrorCode, attributes(error_code))]
pub fn error_code_derive(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as DeriveInput);
    let name = &input.ident;

    let expanded = quote! {

        impl ErrorCode for #name {
            fn error(&self) -> (StatusCode, String, String) {
                (self.status, self.code.to_string(), self.message.to_string())
            }
        }
    };

    TokenStream::from(expanded)
}



#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let result = add(2, 2);
        assert_eq!(result, 4);
    }
}
