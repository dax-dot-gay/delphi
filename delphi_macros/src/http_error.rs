use std::collections::HashMap;

use convert_case::{ Case, Casing };
use darling::FromMeta;
use manyhow::error_message;
use proc_macro2::TokenStream;
use quote::{ format_ident, quote };
use syn::{ punctuated::Punctuated, token::Comma, Field, Fields, FieldsNamed, Ident, ItemEnum };

/*
Ok(
    quote! {
    mod #mod_id {
        use serde::{Serialize, Deserialize};
        use schemars::JsonSchema;
        use rocket::Responder;
        use okapi::openapi3::{MediaType, Responses};
        use rocket_okapi::response::OpenApiResponderInner;

        #[derive(Clone, Debug, Serialize, Deserialize, JsonSchema, Responder, thiserror::Error)]
        #[response(status = 401, content_type = "json")]
        #[serde(tag = "code", rename_all = "snake_case")]
        enum Unauthorized {
            #[error("This path requires login: {path}")]
            RequiresLogin {
                path: String
            }
        }

        #[derive(Clone, Debug, Serialize, Deserialize, JsonSchema, Responder, thiserror::Error)]
        #[serde(tag = "status", rename_all = "snake_case")]
        pub enum #wrapper_id {
            #[error(transparent)]
            #[response(status = 401, content_type = "json")]
            Unauthorized {
                error: Unauthorized
            }
        }

        impl #wrapper_id {
            pub fn not_authorized_requires_login(path: impl Into<String>) -> Self {
                Self::Unauthorized {error: Unauthorized::RequiresLogin {path: path.into()}}
            }
        }

        impl OpenApiResponderInner for #wrapper_id {
            fn responses(generator: &mut rocket_okapi::r#gen::OpenApiGenerator) -> rocket_okapi::Result<okapi::openapi3::Responses> {
                use okapi::openapi3::RefOr;
                Ok(Responses {
                    responses: okapi::map! {
                        "401".to_owned() => RefOr::Object({
                            let schema = generator.json_schema::<Unauthorized>();
                            okapi::openapi3::Response {
                                description: "# 401 Unauthorized".to_owned(),
                                content: okapi::map! {
                                    "application/json".to_owned() => MediaType {
                                        schema: Some(schema),
                                        ..Default::default()
                                    }
                                },
                                ..Default::default()
                            }
                        })
                    },
                    ..Default::default()
                })
            }
        }
    }

    #vis use #mod_id::#wrapper_id;
}
)
*/

#[derive(FromMeta, Debug, Clone)]
struct ErrorArgs {
    pub code: u16,
    pub message: String,
}

#[derive(Clone, Debug)]
struct ErrorVariant {
    pub fields: FieldsNamed,
    pub ident: Ident,
    pub message: String,
}

fn codegen_status(
    code: u16,
    variants: Vec<ErrorVariant>,
    status_enums: &mut Vec<TokenStream>,
    status_variants: &mut Punctuated<TokenStream, Comma>,
    error_functions: &mut Vec<TokenStream>,
    status_responses: &mut Punctuated<TokenStream, Comma>,
    code_matches: &mut Punctuated<TokenStream, Comma>
) -> manyhow::Result<()> {
    let status = httpstatus::StatusCode::from(code);
    let status_name = format_ident!("{}", status.reason_phrase().to_case(Case::Pascal));
    let status_prefix = status.reason_phrase().to_case(Case::Snake);

    status_variants.push(
        quote! {
        #[error(transparent)]
        #[response(status = #code, content_type = "application/json")]
        #status_name {
            error: #status_name
        }
    }
    );

    let code_str = code.to_string();
    let code_reason = status.reason_phrase();
    let code_desc = format!("# {code} {code_reason}");
    status_responses.push(
        quote! {
        #code_str.to_owned() => RefOr::Object({
            let schema = generator.json_schema::<#status_name>();
            okapi::openapi3::Response {
                description: #code_desc.to_owned(),
                content: okapi::map! {
                    "application/json".to_owned() => MediaType {
                        schema: Some(schema),
                        ..Default::default()
                    }
                },
                ..Default::default()
            }
        })
    }
    );

    let mut internal_variants: Punctuated<TokenStream, Comma> = Punctuated::new();
    for ErrorVariant { fields, ident, message } in variants.clone() {
        internal_variants.push(
            quote! {
            #[error(#message)]
            #ident #fields
        }
        );

        let fn_name = format_ident!("{status_prefix}_{}", ident.to_string().to_case(Case::Snake));

        let fn_args: Punctuated<TokenStream, Comma> = fields.named
            .iter()
            .cloned()
            .map(|Field { ident, ty, .. }| {
                quote! { #ident: impl Into<#ty> }
            })
            .collect();

        let fn_args_resolved: Punctuated<TokenStream, Comma> = fields.named
            .iter()
            .cloned()
            .map(|Field { ident, .. }| {
                quote! { #ident: #ident.into() }
            })
            .collect();

        error_functions.push(
            quote! {
            pub fn #fn_name(#fn_args) -> Self {
                Self::#status_name {error: #status_name::#ident {#fn_args_resolved}}
            }
        }
        );
    }

    status_enums.push(
        quote! {
        #[derive(Clone, Debug, Serialize, Deserialize, JsonSchema, thiserror::Error)]
        #[serde(tag = "code", rename_all = "snake_case")]
        enum #status_name {
            #internal_variants
        }

        impl<'r> rocket::response::Responder<'r, 'static> for #status_name {
            fn respond_to(self, r: &'r rocket::Request<'_>) -> rocket::response::Result<'static> {
                Json(self).respond_to(r)
            }
        }
    }
    );

    code_matches.push(
        quote! {
        Self::#status_name {..} => rocket::http::Status::new(#code)
    }
    );

    Ok(())
}

pub fn impl_http_error(_: TokenStream, item: TokenStream) -> manyhow::Result {
    let item = syn::parse2::<ItemEnum>(item)?;
    let mod_id = format_ident!("http_error_{}", item.ident.to_string().to_case(Case::Snake));
    let wrapper_id = item.ident.clone();
    let vis = item.vis.clone();

    let mut mapped_variants: HashMap<u16, Vec<ErrorVariant>> = HashMap::new();
    for variant in item.variants.clone() {
        if let Fields::Named(fields) = variant.fields.clone() {
            if variant.attrs[0].path().is_ident("err") {
                let args = ErrorArgs::from_meta(&variant.attrs[0].meta).unwrap();

                if !mapped_variants.contains_key(&args.code) {
                    let _ = mapped_variants.insert(args.code, Vec::new());
                }

                mapped_variants.get_mut(&args.code).unwrap().push(ErrorVariant {
                    fields,
                    ident: variant.ident.clone(),
                    message: args.message,
                });
            } else {
                return Err(
                    error_message!(
                        "Exactly one #[err(...)] attribute and no other attributes are allowed."
                    ).into()
                );
            }
        } else {
            return Err(error_message!("Only variants with named fields are allowed!").into());
        }
    }

    let mut status_enums: Vec<TokenStream> = Vec::new();
    let mut status_variants: Punctuated<TokenStream, Comma> = Punctuated::new();
    let mut error_functions: Vec<TokenStream> = Vec::new();
    let mut status_responses: Punctuated<TokenStream, Comma> = Punctuated::new();
    let mut code_matches: Punctuated<TokenStream, Comma> = Punctuated::new();

    for (code, variants) in mapped_variants {
        codegen_status(
            code,
            variants,
            &mut status_enums,
            &mut status_variants,
            &mut error_functions,
            &mut status_responses,
            &mut code_matches
        )?;
    }

    Ok(
        quote! {
        mod #mod_id {
            use serde::{Serialize, Deserialize};
            use schemars::JsonSchema;
            use rocket::{Responder, serde::json::Json};
            use okapi::openapi3::{MediaType, Responses};
            use rocket_okapi::response::OpenApiResponderInner;

            #(#status_enums)*

            #[derive(Clone, Debug, Serialize, Deserialize, JsonSchema, Responder, thiserror::Error)]
            #[serde(tag = "status", rename_all = "snake_case")]
            pub enum #wrapper_id {
                #status_variants
            }

            impl #wrapper_id {
                pub fn code(&self) -> rocket::http::Status {
                    match self {
                        #code_matches
                    }
                }

                #(#error_functions)*
            }

            impl OpenApiResponderInner for #wrapper_id {
                fn responses(generator: &mut rocket_okapi::r#gen::OpenApiGenerator) -> rocket_okapi::Result<okapi::openapi3::Responses> {
                    use okapi::openapi3::RefOr;
                    Ok(Responses {
                        responses: okapi::map! {
                            #status_responses
                        },
                        ..Default::default()
                    })
                }
            }
        }

        #vis use #mod_id::#wrapper_id;
    }
    )
}
