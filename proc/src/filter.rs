use crate::helper::{CaseStyle, CaseStyleHelpers};
use proc_macro::TokenStream;
use quote::{format_ident, quote};
use syn::{Attribute, Data, DeriveInput, Fields, LitStr, parse_macro_input};

/// Namespace and attribute keys for `flo_orm`.
const FLO_ORM_NAMESPACE: &str = "sea_orm";
/// attribute keys for skipping the field
const SKIP_FILTER_KEY: &str = "filter_skip";
/// attribute keys for overriding the type
const OVERRIDE_KEY: &str = "filter_override";
/// attribute keys for renaming the end name
const RENAME_KEY: &str = "filter_rename";
/// attribute keys for renaming the model name
const MODEL_NAME_KEY: &str = "table_name";
/// attribute keys for only equal/not_equal
const PLUS_ORDER_KEY: &str = "filter_plus_order";
/// attribute keys for single
const SINGLE_KEY: &str = "filter_single";

/// Derives a filter query struct for the given struct, generating fields for various filter operations.
///
/// This procedural macro generates a filter query struct for SeaORM entities, adding fields to filter based on
/// equality, inequality, and range comparisons (greater than, less than, greater than or equal, less than or equal).
pub fn derive_to_filter_query(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as DeriveInput);
    let struct_name = input.ident;

    // Retrieve the model name from the attribute `#[flo_orm(model_name = "name")]`
    let model_name = input
        .attrs
        .iter()
        .find_map(get_model_name)
        .unwrap_or_else(|| struct_name.to_string());

    // Generate the name of the filter struct, appending "FilterQuery"
    let filter_struct_name = format_ident!("{}FilterQuery", model_name);

    let fields = if let Data::Struct(data_struct) = input.data {
        if let Fields::Named(fields_named) = data_struct.fields {
            fields_named.named
        } else {
            panic!("DeriveToFilterQuery only supports structs with named fields.");
        }
    } else {
        panic!("DeriveToFilterQuery only supports structs.");
    };

    // Process each field, applying filters, overrides, and renames
    let map = fields.iter().filter_map(|field| {
        let field_name = &field.ident;
        let column_name = format_ident!(
            "{}",
            field_name
                .clone()
                .unwrap()
                .convert_case(Some(CaseStyle::PascalCase))
        );

        // Check for `#[flo_orm(skip_filter)]`
        if field.attrs.iter().any(is_skip_filter) {
            return None;
        }

        // Check for `#[flo_orm(override = "type")]` and replace the type if present
        let original_field_type = field.ty.clone();
        let field_type = field
            .attrs
            .iter()
            .find_map(get_override_type)
            .unwrap_or_else(|| original_field_type.clone());

        // Check for `#[flo_orm(rename = "name")]` and rename the field if present
        let output_name = field
            .attrs
            .iter()
            .find_map(get_rename_name)
            .unwrap_or_else(|| field_name.clone().unwrap());

        let eq_name = format_ident!("{}_eq", output_name);
        let neq_name = format_ident!("{}_neq", output_name);
        let gt_name = format_ident!("{}_gt", output_name);
        let lt_name = format_ident!("{}_lt", output_name);
        let gte_name = format_ident!("{}_gte", output_name);
        let lte_name = format_ident!("{}_lte", output_name);

        let eq_name_string = eq_name.to_string();
        let neq_name_string = neq_name.to_string();
        let gt_name_string = gt_name.to_string();
        let lt_name_string = lt_name.to_string();
        let gte_name_string = gte_name.to_string();
        let lte_name_string = lte_name.to_string();

        let mut struct_part = quote! {
                /// Field to filter for equality, allowing multiple values.
                /// This creates a condition where the column matches any of the provided values.
                #[serde(default)]
                pub #eq_name: Vec<#field_type>,

                /// Field to filter for inequality, allowing multiple values.
                /// This excludes any results where the column matches one of the provided values.
                #[serde(default)]
                pub #neq_name: Vec<#field_type>,
            };

        let mut debug_part = quote! {
                if  !self.#eq_name.is_empty() { res.push(format!("{}={:?}", #eq_name_string, self.#eq_name)) }
                if  !self.#neq_name.is_empty() { res.push(format!("{}={:?}", #neq_name_string, self.#neq_name)) }
            };

        let mut filter_part = quote! {
                .add_option({ if self.#eq_name.is_empty() { None } else { Some(Column::#column_name.is_in(self.#eq_name.into_iter().map(Into::<#original_field_type>::into))) }})
                .add_option({ if self.#neq_name.is_empty() { None } else { Some(Column::#column_name.is_not_in(self.#neq_name.into_iter().map(Into::<#original_field_type>::into))) }})
            };

        if field.attrs.iter().any(is_single_filter) {
            struct_part = quote! {
                /// Field to filter for equality, allowing multiple values.
                /// This creates a condition where the column matches the provided values.
                pub #eq_name: Option<#field_type>,

                /// Field to filter for inequality, allowing multiple values.
                /// This excludes any results where the column matches the provided values.
                pub #neq_name: Option<#field_type>,
            };

            debug_part = quote! {
                if let Some(x) = self.#eq_name.clone() { res.push(format!("{}={x:?}", #eq_name_string)) }
                if let Some(x) = self.#neq_name.clone() { res.push(format!("{}={x:?}", #neq_name_string)) }
            };

            filter_part = quote! {
                .add_option(self.#eq_name.map(|x| Column::#column_name.eq(Into::<#original_field_type>::into(x))))
                .add_option(self.#neq_name.map(|x| Column::#column_name.ne(Into::<#original_field_type>::into(x))))
            };
        }


        // Check for `#[flo_orm(filter_only_eq)]`
        if field.attrs.iter().any(is_plus_order) {
            struct_part = quote! {
                #struct_part

                /// Field to filter for #field_type greater than the specified value.
                /// Useful for range-based queries (e.g., prices above a threshold).
                pub #gt_name: Option<#field_type>,

                /// Field to filter for #field_type less than the specified value.
                /// Useful for range-based queries (e.g., dates before a threshold).
                pub #lt_name: Option<#field_type>,

                /// Field to filter for #field_type greater than or equal to the specified value.
                /// Similar to `gt`, but includes the boundary value in the results.
                pub #gte_name: Option<#field_type>,

                /// Field to filter for #field_type less than or equal to the specified value.
                /// Similar to `lt`, but includes the boundary value in the results.
                pub #lte_name: Option<#field_type>,
            };

            debug_part = quote! {
                #debug_part
                if let Some(x) = self.#gt_name.clone() { res.push(format!("{}={x:?}", #gt_name_string)) }
                if let Some(x) = self.#lt_name.clone() { res.push(format!("{}={x:?}", #lt_name_string)) }
                if let Some(x) = self.#gte_name.clone() { res.push(format!("{}={x:?}", #gte_name_string)) }
                if let Some(x) = self.#lte_name.clone() { res.push(format!("{}={x:?}", #lte_name_string)) }
            };

            filter_part = quote! {
                #filter_part
                .add_option(self.#gt_name.map(|x| Column::#column_name.gt(Into::<#original_field_type>::into(x))))
                .add_option(self.#lt_name.map(|x| Column::#column_name.lt(Into::<#original_field_type>::into(x))))
                .add_option(self.#gte_name.map(|x| Column::#column_name.gte(Into::<#original_field_type>::into(x))))
                .add_option(self.#lte_name.map(|x| Column::#column_name.lte(Into::<#original_field_type>::into(x))))
            };
        }

        Some((struct_part, debug_part, filter_part))
    });

    let iter_map = map.into_iter();
    let struct_impl: Vec<_> = iter_map
        .clone()
        .map(|(struct_part, _, _)| struct_part.clone())
        .collect();
    let debug_impl: Vec<_> = iter_map
        .clone()
        .map(|(_, debug_part, _)| debug_part.clone())
        .collect();
    let filter_impl: Vec<_> = iter_map
        .map(|(_, _, filter_part)| filter_part.clone())
        .collect();

    // Generate the output token stream
    let expanded = quote! {
        /// Generated by DeriveToFilterQuery macro for flo_orm
        #[derive(Clone, serde::Deserialize, utoipa::IntoParams)]
        pub struct #filter_struct_name {
            #(#struct_impl)*
        }

        impl std::fmt::Debug for #filter_struct_name {
            fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                let mut res: Vec<String> = vec![];

                #(#debug_impl)*

                if res.is_empty() {
                    write!(f, "*")
                } else {
                    write!(f, "{}", res.join("&"))
                }
            }
        }


        impl sea_orm::sea_query::IntoCondition for #filter_struct_name {
            fn into_condition(self) -> sea_orm::Condition {
                use sea_orm::{sea_query::SimpleExpr::Custom, ColumnTrait, Condition};
                sea_orm::Condition::all()
                    #(#filter_impl)*
            }
        }
    };

    TokenStream::from(expanded)
}

/// Helper function to check if an attribute is `#[flo_orm(skip_filter)]`
fn is_skip_filter(attr: &Attribute) -> bool {
    let mut result = false;
    if attr.path().is_ident(FLO_ORM_NAMESPACE) {
        attr.parse_nested_meta(|meta| {
            if meta.path.is_ident(SKIP_FILTER_KEY) {
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

fn is_plus_order(attr: &Attribute) -> bool {
    let mut result = false;
    if attr.path().is_ident(FLO_ORM_NAMESPACE) {
        attr.parse_nested_meta(|meta| {
            if meta.path.is_ident(PLUS_ORDER_KEY) {
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

fn is_single_filter(attr: &Attribute) -> bool {
    let mut result = false;
    if attr.path().is_ident(FLO_ORM_NAMESPACE) {
        attr.parse_nested_meta(|meta| {
            if meta.path.is_ident(SINGLE_KEY) {
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

/// Helper function to extract the type from `#[flo_orm(override = "type")]`
fn get_override_type(attr: &Attribute) -> Option<syn::Type> {
    let mut result = None;
    if attr.path().is_ident(FLO_ORM_NAMESPACE) {
        attr.parse_nested_meta(|meta| {
            if meta.path.is_ident(OVERRIDE_KEY) {
                let value = meta.value()?;
                let s: LitStr = value.parse()?;

                result = s.parse().ok();
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
