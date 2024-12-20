use syn::{Meta, MetaList, Token, Expr, Lit, ExprLit, MetaNameValue, ExprArray};
use proc_macro2::Span;
use darling::ast::NestedMeta;

use crate::opts::InstrumentOpts;

/// Parse attribute arguments from token stream
pub(crate) fn parse_instrument_args(attr_args: proc_macro::TokenStream) -> Vec<NestedMeta> {
    match syn::parse::<MetaList>(attr_args.clone()) {
        Ok(meta_list) => {
            // Extract string attributes manually
            let attrs: Vec<String> = meta_list.tokens
                .into_iter()
                .filter_map(|token| {
                    if let proc_macro2::TokenTree::Literal(lit) = token {
                        let s = lit.to_string();
                        // Remove quotes and parse as string
                        if s.starts_with('"') && s.ends_with('"') {
                            Some(s[1..s.len()-1].to_string())
                        } else {
                            None
                        }
                    } else {
                        None
                    }
                })
                .collect();

            // Create a mock NestedMeta list for Darling
            vec![
                NestedMeta::Meta(Meta::NameValue(MetaNameValue {
                    path: syn::Path::from(syn::Ident::new("attributes", Span::call_site())),
                    eq_token: Token![=](Span::call_site()),
                    value: Expr::Lit(ExprLit {
                        attrs: vec![],
                        lit: Lit::Str(syn::LitStr::new(&format!("{:?}", attrs), Span::call_site())),
                    }),
                }))
            ]
        },
        Err(_) => vec![]
    }
}

/// Extract instrumentation options from parsed metadata
pub(crate) fn extract_instrument_opts(input_fn: &syn::ItemFn, opts: &InstrumentOpts) -> (String, String, String) {
    // Determine metric name (use function name if not specified)
    let metric_name = opts.name.clone().unwrap_or_else(|| input_fn.sig.ident.to_string());
    let unit = opts.unit.clone().unwrap_or_else(|| "ms".to_string());
    let description = opts.description.clone().unwrap_or_else(|| 
        format!("Metrics for function {}", input_fn.sig.ident)
    );

    (metric_name, unit, description)
}
