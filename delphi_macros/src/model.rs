use darling::{ast::NestedMeta, FromMeta};
use proc_macro2::TokenStream;
use quote::quote;
use syn::{Fields, ItemStruct};

#[derive(FromMeta, Debug, Clone)]
struct ModelArgs {
    pub collection: String,
}

pub fn impl_model(args: TokenStream, item: TokenStream) -> manyhow::Result {
    let args = NestedMeta::parse_meta_list(args)?;
    let ModelArgs { collection } = ModelArgs::from_list(&args).unwrap();

    let ItemStruct { attrs, vis, ident, generics, fields, .. } = syn::parse2::<ItemStruct>(item)?;
    let (impl_generics, type_generics, where_clause) = generics.split_for_impl();
    let fields = match fields {
        Fields::Named(f) => f,
        _ => panic!("Only named fields are supported!")
    }.named;


    Ok(quote! {
        #[derive(Clone, Debug, serde::Serialize, serde::Deserialize, oximod::Model, schemars::JsonSchema, bon::Builder)]
        #(#attrs)*
        #[db("delphi")]
        #[collection(#collection)]
        #[document_id_setter_ident("object_id")]
        #vis struct #ident #generics {
            #[builder(field)]
            #[serde(skip_serializing_if = "Option::is_none")]
            #[schemars(skip)]
            pub _id: Option<bson::oid::ObjectId>,

            #[builder(default = crate::util::default_uid(), name = id)]
            #[serde(default = "crate::util::default_uid")]
            #[default(crate::util::default_uid())]
            #[index(unique)]
            pub _docid: String,

            #fields
        }

        impl #impl_generics #ident #type_generics #where_clause {
            pub fn id(&self) -> String {
                self._docid.clone()
            }

            pub async fn get(id: impl AsRef<str>) -> crate::Result<Option<Self>> {
                use oximod::Model;
                Ok(Self::find_one(bson::doc! {"_docid": id.as_ref().to_string()}).await?)
            }

            pub fn as_document(&self) -> crate::Result<bson::Document> {
                Ok(bson::to_document(&self)?)
            }
            
            pub async fn save(&self) -> crate::Result<()> {
                let collection = Self::get_collection()?;
                Ok(collection.replace_one(bson::doc! {"_docid": self.id()}, self.as_document()?).upsert(true).await.and(Ok(()))?)
            }
        }
    })
}