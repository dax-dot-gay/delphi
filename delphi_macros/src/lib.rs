use proc_macro::TokenStream;
mod model;
mod http_error;

#[manyhow::manyhow]
#[proc_macro_attribute]
pub fn model(args: TokenStream, item: TokenStream) -> manyhow::Result {
    model::impl_model(args.into(), item.into())
}

#[manyhow::manyhow]
#[proc_macro_attribute]
pub fn http_error(args: TokenStream, item: TokenStream) -> manyhow::Result {
    http_error::impl_http_error(args.into(), item.into())
}

#[manyhow::manyhow]
#[proc_macro_attribute]
pub fn err(_: TokenStream, item: TokenStream) -> TokenStream {
    item
}