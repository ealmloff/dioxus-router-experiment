use dioxus::prelude::*;
use dioxus_router_core::history::HistoryProvider;
use router::Routable;
use std::str::FromStr;

struct Router<R: Routable, H: HistoryProvider> {
    history: H,
    current_route: R,
}

impl<R: Routable, H: HistoryProvider> Router<R, H> {
    fn new(history: H) -> Result<Self, R::Err> {
        let path = history.current_path();
        Ok(Self {
            history,
            current_route: R::from_str(path.as_str())?,
        })
    }
}

trait Routable: FromStr {
    fn render<'a>(self, cx: &'a ScopeState) -> Element<'a>;
}

#[derive(Routable)]
enum Route {
    #[route("/hello_world/(dynamic)")]
    Route2 { dynamic: u32 },
    #[route("/(dynamic)")]
    Route1 { dynamic: String },
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

impl Routable for Route {
    fn render<'a>(self, cx: &'a ScopeState) -> Element<'a> {
        match self {
            Route::Route1 { dynamic } => {
                let comp = Route1Props { dynamic };
                let cx = cx.bump().alloc(Scoped {
                    props: cx.bump().alloc(comp),
                    scope: cx,
                });
                Route1(cx)
            }
            Route::Route2 { dynamic } => {
                let comp = Route2Props { dynamic };
                let cx = cx.bump().alloc(Scoped {
                    props: cx.bump().alloc(comp),
                    scope: cx,
                });
                Route2(cx)
            }
        }
    }
}

fn root(cx: Scope) -> Element {
    render! {div {}}
}

fn main() {}
