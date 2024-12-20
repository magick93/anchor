extern crate proc_macro;

mod opts;

use proc_macro::TokenStream;
use quote::quote;
use syn::{parse_macro_input, ItemFn};

use crate::opts::InstrumentOpts;

/// Instrumentation macro for adding OpenTelemetry metrics to functions
#[proc_macro_attribute]
pub fn instrument(attr_args: TokenStream, input: TokenStream) -> TokenStream {
    // Parse the input function
    let input_fn = parse_macro_input!(input as ItemFn);

    // Parse the attribute arguments
    let opts = match syn::parse::<InstrumentOpts>(attr_args) {
        Ok(opts) => opts,
        Err(e) => return TokenStream::from(e.to_compile_error()),
    };

    // Determine metric name (use function name if not specified)
    let metric_name = opts.name.clone().unwrap_or_else(|| input_fn.sig.ident.to_string());
    let unit = opts.unit.clone().unwrap_or_else(|| "ms".to_string());
    let description = opts.description.clone().unwrap_or_else(|| 
        format!("Metrics for function {}", input_fn.sig.ident)
    );

    let fn_name = &input_fn.sig.ident;
    let fn_body = &input_fn.block;
    let fn_vis = &input_fn.vis;
    let fn_sig = &input_fn.sig;

    // Generate instrumented function
    let output = quote! {
        #fn_vis #fn_sig {
            // Initialize OpenTelemetry meter
            let meter = opentelemetry::global::meter("anchor_instrumentation");
            
            // Create histogram for timing
            let histogram = meter
                .f64_histogram(#metric_name)
                .with_unit(#unit)
                .with_description(#description)
                .build();

            // Prepare attributes
            let mut attributes = vec![
                opentelemetry::KeyValue::new("function", stringify!(#fn_name).to_string())
            ];

            // Add any custom attributes
            if let Some(custom_attributes) = &opts.attributes {
                for attr in custom_attributes {
                    attributes.push(opentelemetry::KeyValue::new(
                        attr.clone(),
                        attr.clone()
                    ));
                }
            }

            // Start timing
            let start_time = std::time::Instant::now();

            // Execute original function body
            let result = {
                #fn_body
            };

            // Record execution time
            let duration = start_time.elapsed().as_secs_f64() * 1000.0; // Convert to milliseconds
            histogram.record(duration, &[]);

            result
        }
    };

    TokenStream::from(output)
}
