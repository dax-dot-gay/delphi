use darling::{ast::NestedMeta, FromMeta};
use proc_macro2::{TokenStream, TokenTree};
use quote::quote;
use syn::{parse::Parser, punctuated::Punctuated, token::Comma, Field, Fields, ItemStruct, Meta, MetaList};

#[derive(FromMeta, Debug, Clone)]
struct ModelArgs {
    pub collection: String,
}

#[derive(Clone, Debug)]
enum FieldKind {
    Start,
    Field,
    Finish,
    Other
}

fn parse_attrs(field: Field) -> FieldKind {
    for attr in field.attrs {
        if attr.path().is_ident("builder") {
            if let Meta::List(MetaList {tokens, ..}) = attr.meta {
                for item in tokens {
                    if let TokenTree::Ident(id) = item {
                        match id.to_string().as_str() {
                            "start_fn" => {
                                return FieldKind::Start;
                            },
                            "field" => {
                                return FieldKind::Field;
                            },
                            "finish_fn" => {
                                return FieldKind::Finish;
                            },
                            _ => ()
                        }
                    }
                }
            }
        }
    }

    FieldKind::Other
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
    
    let mut start_fields = Punctuated::<Field, Comma>::new();
    let mut field_fields = Punctuated::<Field, Comma>::new();
    let mut finish_fields = Punctuated::<Field, Comma>::new();
    let mut other_fields = Punctuated::<Field, Comma>::new();
    for field in fields.clone() {
        match parse_attrs(field.clone()) {
            FieldKind::Start => start_fields.push(field),
            FieldKind::Field => field_fields.push(field),
            FieldKind::Finish => finish_fields.push(field),
            FieldKind::Other => other_fields.push(field),
        }
    }

    let mut combined_fields = Punctuated::<Field, Comma>::new();
    combined_fields.extend(start_fields);
    combined_fields.push(Field::parse_named.parse2(quote! {
        #[builder(field)]
        #[serde(skip_serializing_if = "Option::is_none")]
        #[schemars(skip)]
        pub _id: Option<bson::oid::ObjectId>
    })?);
    combined_fields.extend(field_fields);
    combined_fields.extend(finish_fields);
    combined_fields.push(Field::parse_named.parse2(quote! {
        #[builder(default = crate::util::default_uid(), name = id)]
        #[serde(default = "crate::util::default_uid")]
        #[default(crate::util::default_uid())]
        #[index(unique)]
        pub _docid: String
    })?);
    combined_fields.extend(other_fields);


    Ok(quote! {
        #[derive(Clone, Debug, serde::Serialize, serde::Deserialize, oximod::Model, schemars::JsonSchema, bon::Builder)]
        #(#attrs)*
        #[db("delphi")]
        #[collection(#collection)]
        #[document_id_setter_ident("object_id")]
        #vis struct #ident #generics {
            #combined_fields
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