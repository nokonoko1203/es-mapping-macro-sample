mod types;

use proc_macro::TokenStream;
use proc_macro2::TokenStream as TokenStream2;
use quote::quote;
use syn::{parse_macro_input, DeriveInput, LitStr};
use types::EsType;

#[proc_macro_derive(EsMapping, attributes(es))]
pub fn es_mapping_derive(token: TokenStream) -> TokenStream {
    let derive_input = &parse_macro_input!(token as DeriveInput);

    let fields = match &derive_input.data {
        syn::Data::Struct(ref data) => &data.fields,
        _ => {
            todo!("Only struct is supported");
        }
    };

    let mut es_fields: Vec<TokenStream2> = Vec::new();

    for field in fields {
        // field name
        let ident = field.ident.as_ref().unwrap();
        let ident_str = ident.to_string();

        // field type
        // todo: What to do when renaming with serde
        let ty = &field.ty;
        let es_type_text = rust_type_to_es_type(ty).as_str();
        let mut es_type = quote! { #es_type_text };

        // if the field has es attribute, override the type
        for attr in &field.attrs {
            let _ = attr.parse_nested_meta(|meta| {
                if meta.path.is_ident("type") {
                    let name: LitStr = meta.value()?.parse()?;
                    es_type = es_type_text_to_quote(name.value().as_str());
                }

                Ok(())
            });
        }
        es_fields.push(quote! {
            #ident_str: {
                "type": #es_type
            }
        });
    }

    let name = &derive_input.ident;
    let expanded: TokenStream2 = quote! {
        impl EsMap for #name {
            fn generate_mapping() -> serde_json::Value {
                serde_json::json!({
                    "mappings":{
                        "properties": {
                            #( #es_fields ),*
                        }
                    }
                })
            }
        }
    };

    TokenStream::from(expanded)
}

fn es_type_text_to_quote(es_type: &str) -> TokenStream2 {
    match es_type {
        "text" => quote! { "text" },
        "keyword" => quote! { "keyword" },
        "long" => quote! { "long" },
        "boolean" => quote! { "boolean" },
        "unsigned_long" => quote! { "unsigned_long" },
        "double" => quote! { "double" },
        "object" => quote! { "object" },
        "nested" => quote! { "nested" },
        "geo_point" => quote! { "geo_point" },
        "geo_shape" => quote! { "geo_shape" },
        _ => quote! { "text" },
    }
}

fn rust_type_to_es_type(ty: &syn::Type) -> EsType {
    match ty {
        syn::Type::Path(type_path) if type_path.path.is_ident("String") => EsType::Text,
        syn::Type::Path(type_path) if type_path.path.is_ident("bool") => EsType::Boolean,
        syn::Type::Path(type_path)
            if type_path.path.is_ident("i8")
                || type_path.path.is_ident("i16")
                || type_path.path.is_ident("i32")
                || type_path.path.is_ident("i64")
                || type_path.path.is_ident("i128")
                || type_path.path.is_ident("isize") =>
        {
            EsType::Number
        }
        syn::Type::Path(type_path)
            if type_path.path.is_ident("u8")
                || type_path.path.is_ident("u16")
                || type_path.path.is_ident("u32")
                || type_path.path.is_ident("u64")
                || type_path.path.is_ident("u128")
                || type_path.path.is_ident("usize") =>
        {
            EsType::UnsignedNumber
        }
        syn::Type::Path(type_path)
            if type_path.path.is_ident("f32") || type_path.path.is_ident("f64") =>
        {
            EsType::Double
        }
        syn::Type::Path(type_path) if type_path.path.is_ident("char") => EsType::Keyword,
        // todo: handle object
        syn::Type::Path(type_path)
            if type_path.path.is_ident("IndexMap") || type_path.path.is_ident("HashMap") =>
        {
            EsType::Object
        }
        // todo: handle nested
        syn::Type::Path(type_path) if type_path.path.is_ident("Vec") => EsType::Nested,
        syn::Type::Array(_) => EsType::Nested,
        // todo: handle Option<T>
        syn::Type::Path(type_path) if type_path.path.is_ident("Option") => EsType::Text,
        // todo: handle geo_point
        syn::Type::Path(type_path) if type_path.path.is_ident("GeoPoint") => EsType::GeoPoint,
        // todo: handle geo_shape
        syn::Type::Path(type_path) if type_path.path.is_ident("GeoShape") => EsType::GeoShape,
        _ => EsType::Text,
    }
}
