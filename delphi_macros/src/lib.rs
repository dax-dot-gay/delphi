use proc_macro::TokenStream;
mod model;

#[manyhow::manyhow]
#[proc_macro_attribute]
pub fn model(args: TokenStream, item: TokenStream) -> manyhow::Result {
    model::impl_model(args.into(), item.into())
}