use proc_macro::TokenStream;
use syn::{parse_macro_input, parse_quote, DeriveInput, ItemImpl};

#[proc_macro_derive(ValidSmalltalkObject)]
pub fn derive(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as DeriveInput);
    let name = input.ident;

    let q: ItemImpl = parse_quote!(
        impl ValidObject for #name {
            fn is_valid(obj: &Self) -> bool {
                // Check if the object has the correct size. This is valid
                // for most special objects
                obj.header.is_size(Self::SIZE)
            }

            fn set_invalid(obj: &mut Self) {
                obj.header.set_invalid();
            }
        }
    );

    quote::quote!(#q).into()
}
