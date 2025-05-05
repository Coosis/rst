use proc_macro::TokenStream;

#[proc_macro_derive(TryIntoClientMessage)]
pub fn try_into_client_message(input: TokenStream) -> TokenStream {
    let input = syn::parse_macro_input!(input as syn::DeriveInput);
    let name = &input.ident;
    let implmented = quote::quote! {
        impl TryInto<super::super::ClientMessage> for #name {
            type Error = serde_json::Error;

            fn try_into(self) -> std::result::Result<super::super::ClientMessage, Self::Error> {
                let content = serde_json::to_vec(&self)?;
                Ok(super::super::ClientMessage {
                    instruct: super::super::ClientInstruct::#name,
                    content,
                })
            }
        }
    };
    implmented.into()
}

#[proc_macro_derive(TryIntoServerMessage)]
pub fn try_into_server_message(input: TokenStream) -> TokenStream {
    let input = syn::parse_macro_input!(input as syn::DeriveInput);
    let name = &input.ident;
    let implmented = quote::quote! {
        impl TryInto<super::super::ServerMessage> for #name {
            type Error = serde_json::Error;

            fn try_into(self) -> std::result::Result<super::super::ServerMessage, Self::Error> {
                let content = serde_json::to_vec(&self)?;
                Ok(super::super::ServerMessage {
                    instruct: super::super::ServerInstruct::#name,
                    content,
                })
            }
        }
    };
    implmented.into()
}
