use crate::helper::{CaseStyle, CaseStyleHelpers};
use proc_macro::TokenStream;
use quote::{format_ident, quote};
use syn::{Attribute, Data, DeriveInput, Fields, LitStr, parse_macro_input};

/// Namespace and attribute keys for `flo_orm`.
const FLO_ORM_NAMESPACE: &str = "sea_orm";
/// attribute keys for skipping the field
const SKIP_SORT_KEY: &str = "sort_skip";
/// attribute keys for renaming the end name
const RENAME_KEY: &str = "sort_rename";
/// attribute keys for renaming the model name
const MODEL_NAME_KEY: &str = "table_name";

/// Derives a sort query struct for the given struct, generating fields for various sort operations.
///
/// This procedural macro generates a sort query struct for SeaORM entities, adding fields to sort based on
/// asc or desc.
pub fn derive_to_sort_query(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as DeriveInput);
    let struct_name = input.ident;

    // Retrieve the model name from the attribute `#[flo_orm(model_name = "name")]`
    let model_name = input
        .attrs
        .iter()
        .find_map(get_model_name)
        .unwrap_or_else(|| struct_name.to_string());

    // Generate the name of the sort struct, appending "SortQuery"
    let sort_struct_name = format_ident!("{}SortQuery", model_name);

    // Generate the name of the sort enum, appending "SortEnum"
    let sort_enum_name = format_ident!("{}SortEnum", model_name);

    let fields = if let Data::Struct(data_struct) = input.data {
        if let Fields::Named(fields_named) = data_struct.fields {
            fields_named.named
        } else {
            panic!("DeriveToSortQuery only supports structs with named fields.");
        }
    } else {
        panic!("DeriveToSortQuery only supports structs.");
    };

    // Process each field, applying sorts, overrides, and renames
    let map = fields.iter().filter_map(|field| {
        let field_name = &field.ident;
        let column_name = format_ident!(
            "{}",
            field_name
                .clone()
                .unwrap()
                .convert_case(Some(CaseStyle::PascalCase))
        );

        // Check for `#[flo_orm(skip_sort)]`
        if field.attrs.iter().any(is_skip_sort) {
            return None;
        }

        // Check for `#[flo_orm(rename = "name")]` and rename the field if present
        let output_field_name = field
            .attrs
            .iter()
            .find_map(get_rename_name)
            .unwrap_or_else(|| field_name.clone().unwrap());

        let comment_field = field
            .attrs
            .iter()
            .find_map(get_comment_field)
            .unwrap_or_else(|| "Default comment for the field".to_string());

        let output_name_asc = format_ident!(
            "{}Asc",
            output_field_name.convert_case(Some(CaseStyle::PascalCase))
        );

        let output_name_desc = format_ident!(
            "{}Desc",
            output_field_name.convert_case(Some(CaseStyle::PascalCase))
        );

        let output_debug_name = output_field_name.convert_case(Some(CaseStyle::SnakeCase));


        let enum_part = quote! {
            #[doc = #comment_field]
            #output_name_asc,

            #[doc = #comment_field]
            #output_name_desc,
        };

        let into_part = quote! {
            #sort_enum_name::#output_name_asc => (Expr::col(Column::#column_name).into(), sea_orm::Order::Asc),
            #sort_enum_name::#output_name_desc => (Expr::col(Column::#column_name).into(), sea_orm::Order::Desc),
        };

        let debug_part = quote! {
            #sort_enum_name::#output_name_asc => format!("{}+", #output_debug_name),
            #sort_enum_name::#output_name_desc => format!("{}-", #output_debug_name),
        };

        Some((enum_part, debug_part, into_part))
    });

    let iter_map = map.into_iter();
    let enum_impl: Vec<_> = iter_map
        .clone()
        .map(|(enum_part, _, _)| enum_part.clone())
        .collect();
    let debug_impl: Vec<_> = iter_map
        .clone()
        .map(|(_, debug_part, _)| debug_part.clone())
        .collect();
    let sort_impl: Vec<_> = iter_map
        .map(|(_, _, sort_part)| sort_part.clone())
        .collect();

    // Generate the output token stream
    let expanded = quote! {
        #[derive(Debug, Clone, Copy, serde::Deserialize, utoipa::ToSchema)]
        #[serde(rename_all = "snake_case")]
        /// Generated by DeriveToSortQuery macro for flo_orm
        pub enum #sort_enum_name {
            #(#enum_impl)*
        }

        /// Generated by DeriveToSortQuery macro for flo_orm
        #[derive(Clone, serde::Deserialize, utoipa::IntoParams)]
        pub struct #sort_struct_name {
            /// Used for sorting the output
            #[serde(default)]
            pub sort: Vec<#sort_enum_name>
        }

        impl std::fmt::Debug for #sort_struct_name {
            fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                let res = self.sort.iter().map(|name| {
                    match name {
                        #(#debug_impl)*
                    }
                }).collect::<Vec<_>>();

                if res.is_empty() {
                    write!(f, "*")
                } else {
                    write!(f, "{}", res.join(","))
                }
            }
        }

        impl IntoIterator for #sort_struct_name {
            type Item = (sea_orm::sea_query::SimpleExpr, sea_orm::Order);
            type IntoIter = std::vec::IntoIter<Self::Item>;

            fn into_iter(self) -> Self::IntoIter {
                self.sort
                .into_iter()
                .map(|name| {
                    match name {
                        #(#sort_impl)*
                    }
                })
                .collect::<Vec<_>>()
                .into_iter()
            }
        }
    };

    TokenStream::from(expanded)
}

/// Helper function to check if an attribute is `#[flo_orm(skip_sort)]`
fn is_skip_sort(attr: &Attribute) -> bool {
    let mut result = false;
    if attr.path().is_ident(FLO_ORM_NAMESPACE) {
        attr.parse_nested_meta(|meta| {
            if meta.path.is_ident(SKIP_SORT_KEY) {
                result = true;
            } else {
                // Reads the value expression to advance the parse stream.
                // Some parameters, such as `primary_key`, do not have any value,
                // so ignoring an error occurred here.
                let _: Option<syn::Expr> = meta.value().and_then(|v| v.parse()).ok();
            }
            Ok(())
        })
        .unwrap_or(())
    }
    result
}
/// Helper function to extract the renamed field name from `#[flo_orm(rename = "name")]`
fn get_rename_name(attr: &Attribute) -> Option<syn::Ident> {
    let mut result = None;
    if attr.path().is_ident(FLO_ORM_NAMESPACE) {
        attr.parse_nested_meta(|meta| {
            if meta.path.is_ident(RENAME_KEY) {
                let value = meta.value()?;
                let s: LitStr = value.parse()?;
                result = syn::Ident::new(&s.value(), s.span()).into();
            } else {
                // Reads the value expression to advance the parse stream.
                // Some parameters, such as `primary_key`, do not have any value,
                // so ignoring an error occurred here.
                let _: Option<syn::Expr> = meta.value().and_then(|v| v.parse()).ok();
            }
            Ok(())
        })
        .unwrap_or(())
    }
    result
}

/// Helper function to extract the model name from `#[flo_orm(model_name = "name")]`
fn get_model_name(attr: &Attribute) -> Option<String> {
    let mut result = None;
    if attr.path().is_ident(FLO_ORM_NAMESPACE) {
        attr.parse_nested_meta(|meta| {
            if meta.path.is_ident(MODEL_NAME_KEY) {
                let value = meta.value()?;
                let s: LitStr = value.parse()?;
                let r = s.value();
                let capitalized = r
                    .chars()
                    .next()
                    .map(|c| c.to_uppercase().collect::<String>() + &r[c.len_utf8()..])
                    .unwrap_or_default();
                result = Some(capitalized);
            } else {
                // Reads the value expression to advance the parse stream.
                // Some parameters, such as `primary_key`, do not have any value,
                // so ignoring an error occurred here.
                let _: Option<syn::Expr> = meta.value().and_then(|v| v.parse()).ok();
            }
            Ok(())
        })
        .unwrap_or(())
    }
    result
}

/// Helper function to extract the comments field
fn get_comment_field(attr: &Attribute) -> Option<String> {
    if !attr.path().is_ident("path") {
        return None;
    }

    if let syn::Meta::NameValue(meta) = &attr.meta {
        if let syn::Expr::Lit(expr) = &meta.value {
            if let syn::Lit::Str(lit_str) = &expr.lit {
                return Some(lit_str.value());
            }
        }
    }

    None
}
