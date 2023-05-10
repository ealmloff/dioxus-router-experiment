extern crate proc_macro;

use std::collections::HashSet;

use proc_macro::TokenStream;
use quote::quote;
use syn::{parse::Parse, parse_macro_input, Ident, LitStr};

#[proc_macro]
pub fn gen_mod(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as ProcInput);

    let route = input.route.value();
    let route_name = input.route_name;

    quote! {
        #[path = #route]
        mod #route_name;
    }
    .into()
}

struct ProcInput {
    route_name: Ident,
    route: LitStr,
    route_segments: Vec<RouteSegment>,
}

impl Parse for ProcInput {
    fn parse(input: syn::parse::ParseStream) -> syn::Result<Self> {
        let route_name = input.parse::<Ident>()?;

        let route = input.parse::<syn::LitStr>()?;

        Ok(Self {
            route_name,
            route,
            route_segments: Vec::new(),
        })
    }
}

enum RouteSegment {
    Static(String),
    Dynamic(Ident),
}
