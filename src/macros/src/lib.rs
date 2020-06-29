#[macro_use]
extern crate quote;
extern crate proc_macro;
use proc_macro::TokenStream;
use syn::{parse_quote, Meta, NestedMeta, Lit::Str};

/// Used to log the time to stdout
/// ### Usage
/// ```ignore
/// {
///     #[timed_fn(name = "some_fn")]
///     fn some_fn() {
///         //your code..
///     }
/// }
/// // OR
/// {
///     #[timed_fn]
///     fn some_fn() {
///         //your code..
///     }
/// }
/// ```
///
/// This will print `[timed]:[function:some_fn]:[0 ms, 15 us]` to std out
#[proc_macro_attribute]
pub fn timed_fn(att: TokenStream, item: TokenStream) -> TokenStream {
    let mut function = syn::parse_macro_input!(item as syn::ItemFn);
    let args = syn::parse_macro_input!(att as syn::AttributeArgs);
    let mut metric_name= function.sig.ident.to_string();
    
    for arg in args {
        match arg {
            NestedMeta::Meta(meta) => {
                match meta {
                    Meta::NameValue(name_value) => {
                        if let Str(str) =  name_value.lit {
                            metric_name = str.value();
                        }
                    },
                    _ =>{},
                }
            },
            _ => {}
        }
    }
    let stmts = function.block.stmts;
    function.block = Box::new(parse_quote!({
        let _log_time = timed::timed_execution::log_time(&#metric_name);
        #(#stmts)*
    }));
    TokenStream::from(quote!(#function))
}
