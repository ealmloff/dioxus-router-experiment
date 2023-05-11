use dioxus::prelude::*;
use dioxus_router_core::history::HistoryProvider;
use router::Routable;
use std::str::FromStr;

#[derive(Debug, PartialEq)]
struct RouteParseError<E: std::fmt::Display> {
    attempted_routes: Vec<E>,
}

impl<E: std::fmt::Display> std::fmt::Display for RouteParseError<E> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Route did not match: ")?;
        for route in &self.attempted_routes {
            writeln!(f, "{}", route)?;
        }
        Ok(())
    }
}

struct Router<R: Routable, H: HistoryProvider> {
    history: H,
    route: R,
}

impl<R: Routable, H: HistoryProvider> Router<R, H> {
    fn new(history: H) -> Result<Self, R::Err> {
        let path = history.current_path();
        Ok(Self {
            history,
            route: R::from_str(path.as_str())?,
        })
    }
}

trait Routable: FromStr + std::fmt::Display {
    fn render(self, cx: &ScopeState) -> Element;
}

#[derive(Routable, Debug, PartialEq)]
enum Route {
    #[route("/(dynamic)")]
    Route1 { dynamic: String },
    #[route("/hello_world/(dynamic)")]
    Route2 { dynamic: u32 },
}

#[test]
fn display_works() {
    let route = Route::Route1 {
        dynamic: "hello".to_string(),
    };

    assert_eq!(route.to_string(), "/hello");

    let route = Route::Route2 { dynamic: 1234 };

    assert_eq!(route.to_string(), "/hello_world/1234");

    let route = Route::Route1 {
        dynamic: "hello_world".to_string(),
    };

    assert_eq!(route.to_string(), "/hello_world");
}

#[test]
fn from_string_works() {
    let w = "/hello";
    assert_eq!(
        Route::from_str(w),
        Ok(Route::Route1 {
            dynamic: "hello".to_string()
        })
    );
    let w = "/hello/";
    assert_eq!(
        Route::from_str(w),
        Ok(Route::Route1 {
            dynamic: "hello".to_string()
        })
    );

    let w = "/hello_world/1234";
    assert_eq!(Route::from_str(w), Ok(Route::Route2 { dynamic: 1234 }));
    let w = "/hello_world/1234/";
    assert_eq!(Route::from_str(w), Ok(Route::Route2 { dynamic: 1234 }));

    let w = "/hello_world";
    assert_eq!(
        Route::from_str(w),
        Ok(Route::Route1 {
            dynamic: "hello_world".to_string()
        })
    );

    let w = "/hello_world/-1";
    match Route::from_str(w) {
        Ok(r) => panic!("should not parse {r:?}"),
        Err(err) => println!("{err}"),
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
