use proc_macro::TokenStream;
use quote::{quote, quote_spanned};
use syn::{parse_macro_input, ItemFn, parse_quote, Visibility, spanned::Spanned, Type};

/// Valid forms:
/// ```rust,ignore
/// #[pdmain]
/// fn main(pd: Playdate) {
///     
/// }
/// ```
#[proc_macro_attribute]
pub fn pdmain(attr: TokenStream, input: TokenStream) -> TokenStream {
    let mut input = parse_macro_input!(input as ItemFn);
    input.attrs.push(parse_quote! {#[no_mangle]});
    input.vis = Visibility::Public(parse_quote! {pub});
    match &input.sig.abi {
        Some(a) => {
            return quote_spanned! {a.span() => {
                ::core::compile_error!{"Expected a function with signature fn(Playdate)"}
                #input
            }}.into()
        }
        None => {
            if &input.sig.inputs.len() != &1 {
                return quote_spanned! {input.sig.inputs.span() => {
                    compile_error!("Expected a function with signature fn(Playdate)")
                    #input
                }}.into()
            }
            match &input.sig.inputs[0] {
                syn::FnArg::Receiver(v) => {
                    return quote_spanned! {v.span() => {
                        compile_error!("Expected a function with signature fn(Playdate)")
                        #input
                    }}.into()
                },
                syn::FnArg::Typed(v) => {
                    match &*v.ty {
                        Type::Path(pat) => {
                            match &pat.path.segments.last() {
                                Some(x) if &x.ident.to_string() == "Playdate" => {
                                    quote! {
                                        #input
                                    }.into()
                                }
                                Some(x) => {
                                    return quote_spanned! {x.span() => {
                                        compile_error!("Expected a function with signature fn(Playdate)")
                                        #input
                                    }}.into()
                                }
                                None => {
                                    return quote_spanned! {pat.path.segments.span() => {
                                        #input
                                        compile_error!("Expected a function with signature fn(Playdate)")
                                    }}.into()
                                }
                            }
                        }
                        o => {
                            return quote_spanned! {o.span() => {
                                compile_error!("Expected a function with signature fn(Playdate)")
                            }}.into()
                        }
                    }
                }
            }
        }
    }
}

/// Valid forms:
/// ```rust,ignore
/// #[pdupdate]
/// fn update(pd: Playdate) {
///     
/// }
/// ```
#[proc_macro_attribute]
pub fn pdupdate(attr: TokenStream, input: TokenStream) -> TokenStream {
    let mut input = parse_macro_input!(input as ItemFn);
    input.attrs.push(parse_quote! {#[no_mangle]});
    input.vis = Visibility::Public(parse_quote! {pub});
    match &input.sig.abi {
        Some(a) => {
            return quote_spanned! {a.span() => {
                ::core::compile_error!{"Expected a function with signature fn(Playdate)"}
                #input
            }}.into()
        }
        None => {
            if &input.sig.inputs.len() != &1 {
                return quote_spanned! {input.sig.inputs.span() => {
                    compile_error!("Expected a function with signature fn(Playdate)")
                    #input
                }}.into()
            }
            match &input.sig.inputs[0] {
                syn::FnArg::Receiver(v) => {
                    return quote_spanned! {v.span() => {
                        compile_error!("Expected a function with signature fn(Playdate)")
                        #input
                    }}.into()
                },
                syn::FnArg::Typed(v) => {
                    match &*v.ty {
                        Type::Path(pat) => {
                            match &pat.path.segments.last() {
                                Some(x) if &x.ident.to_string() == "Playdate" => {
                                    quote! {
                                        #input
                                    }.into()
                                }
                                Some(x) => {
                                    return quote_spanned! {x.span() => {
                                        compile_error!("Expected a function with signature fn(Playdate)")
                                        #input
                                    }}.into()
                                }
                                None => {
                                    return quote_spanned! {pat.path.segments.span() => {
                                        #input
                                        compile_error!("Expected a function with signature fn(Playdate)")
                                    }}.into()
                                }
                            }
                        }
                        o => {
                            return quote_spanned! {o.span() => {
                                compile_error!("Expected a function with signature fn(Playdate)")
                            }}.into()
                        }
                    }
                }
            }
        }
    }
}