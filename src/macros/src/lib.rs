
#[macro_use]
extern crate quote;
extern crate proc_macro;
use syn::Lit::Str;
use proc_macro::TokenStream;
use syn::{Attribute, parse_quote};


fn validate_attribute(attr: &Attribute) -> (bool, Option<String>) {
    if let Ok(meta) = Attribute::parse_meta(attr) {
        match meta {
            syn::Meta::NameValue(name_value) => {
                let key_is_timed_fn = name_value.path.is_ident("timed_fn");
                if key_is_timed_fn {
                    if let Str(s) = name_value.lit {
                        println!("{:#?}", s.value());
                        return (true, Some(s.value()))
                    };
                }
            },
            syn::Meta::Path(path) => {
                let key_is_timed_fn = path.is_ident("timed_fn");
                if key_is_timed_fn {
                    return (true, None)
                }
            },
            _ => {}
        }
    }
    (false, None)
}

/// Used to log the time to stdout
/// ### Usage
/// ```ignore
/// {
///     timed_fn!("some_fn")
///     fn some_fn() {
///         //your code..
///     }
/// }
/// ```
/// 
/// This will print `[timed]:[function:some_fn]:[0 ms, 15 us]` to std out
#[proc_macro_attribute]
pub fn timed_fn(_: TokenStream, item: TokenStream) -> TokenStream {
    let mut function = syn::parse_macro_input!(item as syn::ItemFn);
    let mut metric_name: Option<String> = None;
    function.attrs.retain(|x| {
        let (valid_attribute, m) = validate_attribute(x);
        metric_name = m;
        if !valid_attribute {
            panic!(format!("Invalid attribute! Please use #[timed_fn(name=\"some name\")] or #[timed_fn]"));
        }
        valid_attribute
    });
    let stmts = function.block.stmts;
    let name = metric_name.unwrap_or(function.sig.ident.to_string());
    function.block = Box::new(parse_quote!({
        let _log_time = timed::timed_execution::log_time(&#name);
        #(#stmts)*
    }));
   
    TokenStream::from(quote!(#function))
}
