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
        write!(f, "Route did not match:\nAttempted Matches:\n")?;
        for (i, route) in self.attempted_routes.iter().enumerate() {
            writeln!(f, "{i}) {route}")?;
        }
        Ok(())
    }
}

struct Router<R: Routable, H: HistoryProvider> where <R as FromStr>::Err: std::fmt::Display {
    history: H,
    route: R,
}

impl<R: Routable, H: HistoryProvider> Router<R, H>where <R as FromStr>::Err: std::fmt::Display {
    fn new(history: H) -> Result<Self, R::Err> {
        let path = history.current_path();
        Ok(Self {
            history,
            route: R::from_str(path.as_str())?,
        })
    }
}

#[derive(Props, PartialEq)]
struct RouterProps {
    current_route: String,
}

trait Routable: FromStr + std::fmt::Display + Clone where <Self as FromStr>::Err: std::fmt::Display {
    fn render(self, cx: &ScopeState) -> Element;

    fn comp(cx: Scope<RouterProps>)-> Element where Self: 'static {
        let router = Self::from_str(&cx.props.current_route);
        match router{
            Ok(router) => router.render(cx),
            Err(err) => {
                render! {pre {
                    "{err}"
                }}
            }
        }
    }
}

#[derive(Routable, Clone, Debug, PartialEq)]
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

fn root(cx: Scope) -> Element {
    let current_route = use_ref(cx, String::new);

    render!{
        input {
            oninput: |evt| {
                *current_route.write() = evt.value.clone()
            },
            value: "{current_route.read()}"
        }

        Route::comp {
            current_route: current_route.read().clone(),
        }
    }
}

fn main() {
    dioxus_desktop::launch(root);
}
