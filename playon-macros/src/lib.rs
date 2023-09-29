use proc_macro::TokenStream;
use quote::{quote, quote_spanned};
use syn::{parse_macro_input, parse_quote, spanned::Spanned, ItemFn, Type, Visibility, ItemImpl};

/// Valid forms:
/// ```rust,ignore
/// #[pd]
/// fn main(pd: Playdate) {
///     
/// }
/// ```
#[proc_macro_attribute]
pub fn pd(attr: TokenStream, input: TokenStream) -> TokenStream {
    let mut input = parse_macro_input!(input as ItemImpl);
    match &mut input.trait_ {
        Some(t) => {
            t.0 = None;
            t.1 = parse_quote! { ::playon::Game };
        }
        None => {
            return quote_spanned! {input.self_ty.span() => {
                ::core::compile_error!{"Expected `impl Game for "}
                #input
            }}
            .into()
        }
    }
    let s_ty = &input.self_ty;
    quote! {
        #[doc(hidden)]
        static __P: ::playon::__private::Mutex<#s_ty> = ::playon::__private::Mutex::new(unsafe { ::playon::__private::uninit::<#s_ty>() });
        #[no_mangle]
        #[doc(hidden)]
        pub fn __playon_start() {
            *__P.get() = <#s_ty as ::playon::Game>::new();
            <#s_ty as ::playon::Game>::start(&mut *__P.get(), ::playon::Playdate::current());
            let _ = ::playon::__private::dt();
        }
        #[no_mangle]
        #[doc(hidden)]
        pub fn __playon_update() -> i32 {
            <#s_ty as ::playon::Game>::update(&mut *__P.get(), ::playon::Playdate::current(), ::playon::__private::dt());
            0
        }
        #input
    }.into()
}