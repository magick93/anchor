use syn::{
    parse::{Parse, ParseStream, Result},
    Token, Lit, Expr, ExprLit, LitStr, 
    punctuated::Punctuated, 
    token::Comma,
    Meta,
};
use proc_macro2::Span;

/// Configuration options for the instrument macro
#[derive(Debug, Default)]
pub(crate) struct InstrumentOpts {
    /// Custom name for the metric (defaults to function name)
    pub(crate) name: Option<String>,
    
    /// Unit of measurement (defaults to "ms")
    pub(crate) unit: Option<String>,
    
    /// Description of the metric
    pub(crate) description: Option<String>,
    
    /// Optional attributes to add to the metric
    pub(crate) attributes: Option<Vec<String>>,
}

impl Parse for InstrumentOpts {
    fn parse(input: ParseStream) -> Result<Self> {
        let mut opts = InstrumentOpts::default();

        let content;
        syn::parenthesized!(content in input);

        let attrs = Punctuated::<Meta, Token![,]>::parse_terminated(&content)?;

        for meta in attrs.iter() {
            match meta {
                Meta::NameValue(name_value) if name_value.path.is_ident("name") => {
                    if let Expr::Lit(ExprLit { lit: Lit::Str(lit_str), .. }) = &name_value.value {
                        opts.name = Some(lit_str.value());
                    }
                },
                Meta::NameValue(name_value) if name_value.path.is_ident("unit") => {
                    if let Expr::Lit(ExprLit { lit: Lit::Str(lit_str), .. }) = &name_value.value {
                        opts.unit = Some(lit_str.value());
                    }
                },
                Meta::NameValue(name_value) if name_value.path.is_ident("description") => {
                    if let Expr::Lit(ExprLit { lit: Lit::Str(lit_str), .. }) = &name_value.value {
                        opts.description = Some(lit_str.value());
                    }
                },
                Meta::List(list) if list.path.is_ident("attributes") => {
                    unimplemented!("Attributes parsing will be added later");
                },
                _ => {}
            }
        }

        Ok(opts)
    }
}
