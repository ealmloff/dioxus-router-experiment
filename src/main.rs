use std::str::FromStr;

use dioxus::prelude::*;

struct Router<R: Routable> {
    history: Vec<R>,
    current_route: R,
}

trait Routable: FromStr {
    fn render<'a>(self, cx: &'a ScopeState) -> Element<'a>;
}

// #[derive(Routable)]
// enum Routes {
//     #[route("/(dynamic)")]
//     Route1 { dynamic: String },
//     #[route("/hello_world/(dynamic)")]
//     Route2 { dynamic: u32 },
// }

enum Route {
    Route1 { dynamic: String },
    Route2 { dynamic: u32 },
}

#[allow(non_camel_case_types)]
enum RouteSegmentParseError {
    Route1_dynamic(<String as FromStr>::Err),
    Route2_dynamic(<u32 as FromStr>::Err),
}

struct ParseRouteFailure {
    route_name: String,
    error: RouteParseError,
}

struct RouteParseError {
    attempted_routes: Vec<Route>,
}

impl FromStr for Route {
    type Err = RouteParseError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        todo!()
    }
}

router::gen_mod!(Route1 "pages/(dynamic).rs");
router::gen_mod!(Route2 "pages/hello_world/(dynamic).rs");

impl Routable for Route {
    fn render<'a>(self, cx: &'a ScopeState) -> Element<'a> {
        match self {
            Route::Route1 { dynamic } => {
                let comp = Route1::Route1Props { dynamic };
                let cx = cx.bump().alloc(Scoped {
                    props: cx.bump().alloc(comp),
                    scope: cx,
                });
                Route1::Route1(cx)
            }
            Route::Route2 { dynamic } => {
                let comp = Route2::Route2Props { dynamic };
                let cx = cx.bump().alloc(Scoped {
                    props: cx.bump().alloc(comp),
                    scope: cx,
                });
                Route2::Route2(cx)
            }
        }
    }
}

fn root(cx: Scope) -> Element {
    render! {div {}}
}

fn main() {}
