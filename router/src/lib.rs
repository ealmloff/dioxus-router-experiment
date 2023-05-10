extern crate proc_macro;

use proc_macro::TokenStream;
use quote::{__private::Span, quote, ToTokens};
use syn::{parse_macro_input, Ident, LitStr};

#[proc_macro_derive(Routable, attributes(route))]
pub fn derive_routable(input: TokenStream) -> TokenStream {
    let routes_enum = parse_macro_input!(input as syn::DeriveInput);

    let route_enum = RouteEnum::parse(routes_enum).unwrap();

    quote! {
        #route_enum
    }
    .into()
}

struct RouteEnum {
    route_name: Ident,
    routes: Vec<Route>,
}

impl RouteEnum {
    fn parse(input: syn::DeriveInput) -> syn::Result<Self> {
        let name = &input.ident;

        if let syn::Data::Enum(data) = input.data {
            let mut routes = Vec::new();

            for variant in data.variants {
                let route = Route::parse(variant)?;
                routes.push(route);
            }

            Ok(Self {
                route_name: name.clone(),
                routes,
            })
        } else {
            Err(syn::Error::new_spanned(
                input.clone(),
                "Routable can only be derived for enums",
            ))
        }
    }
}

impl ToTokens for RouteEnum {
    fn to_tokens(&self, tokens: &mut quote::__private::TokenStream) {
        let routes = &self.routes;

        tokens.extend(quote!(
            #[path = "pages"]
            mod pages {
                #(#routes)*
            }
            pub use pages::*;
        ));
    }
}

#[derive(Debug)]
struct Route {
    route_name: Ident,
    route: LitStr,
    route_segments: Vec<RouteSegment>,
}

impl Route {
    fn parse(input: syn::Variant) -> syn::Result<Self> {
        let route_attr = input
            .attrs
            .iter()
            .find(|attr| attr.path.is_ident("route"))
            .ok_or_else(|| {
                syn::Error::new_spanned(
                    input.clone(),
                    "Routable variants must have a #[route(...)] attribute",
                )
            })?;
        let route = route_attr.parse_args::<LitStr>()?;

        let route_name = input.ident;

        Ok(Self {
            route_name,
            route_segments: parse_route_segments(route.value()),
            route,
        })
    }
}

impl ToTokens for Route {
    fn to_tokens(&self, tokens: &mut quote::__private::TokenStream) {
        let route = self.route.value()[1..].to_string() + ".rs";
        let route_name: Ident = self.route_name.clone();
        let prop_name = Ident::new(&(self.route_name.to_string() + "Props"), Span::call_site());

        tokens.extend(quote!(
            #[path = #route]
            mod #route_name;
            pub use #route_name::{#prop_name, #route_name};
        ));
    }
}

fn parse_route_segments(route: String) -> Vec<RouteSegment> {
    let mut route_segments = Vec::new();

    for segment in route.split('/') {
        if segment.starts_with('(') && segment.ends_with(')') {
            let ident = segment[1..segment.len() - 1].to_string();
            route_segments.push(RouteSegment::Dynamic(Ident::new(&ident, Span::call_site())));
        } else {
            route_segments.push(RouteSegment::Static(segment.to_string()));
        }
    }

    route_segments
}

#[derive(Debug)]
enum RouteSegment {
    Static(String),
    Dynamic(Ident),
}
