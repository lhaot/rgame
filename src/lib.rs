use proc_macro::TokenStream;

use syn::__private::quote::quote;
use syn::{parse_macro_input, Data, DataEnum, DeriveInput, Variant};

#[proc_macro_derive(Values)]
pub fn derive_values(token_stream: TokenStream) -> TokenStream {
    let ast: DeriveInput = parse_macro_input!(token_stream);
    let variants = match &ast.data {
        Data::Enum(DataEnum { variants, .. }) => variants,
        _ => unreachable!("derive marco: [Values] must use on enum!"),
    };
    let ident = ast.ident;
    let names = variants.iter().map(|Variant { ident, .. }| ident);
    let size = names.len();
    TokenStream::from(quote!(
        impl #ident {
            const values: [Self; #size] = [#(Self::#names),*];
        }
    ))
}
