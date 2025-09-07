mod asset_mgt;
use dioxus::prelude::*;

use asset_mgt::{
    BOSTON, CINCINNATI, EGGS, LINDNER, LOGO, MIRADOR_CONDOR, PHILLY, PRINCETON, PROFILE, SBU,
    TOTODILE, UC,
};

// #[derive(Routable, Clone)]
// enum Route {
//     #[layout(Header)]
//     #[route("/")]
//     Home {},

//     #[route("/about")]
//     About {},

//     #[nest("/blog")]
//     #[layout(Blog)]
//     #[route("/")]
//     BlogList {},
//     #[route("/post/:name")]
//     BlogPost { name: String },
//     #[end_layout]
//     #[end_nest]
//     #[end_layout]
//     #[nest("/myblog")]
//     #[redirect("/", || Route::BlogList {})]
//     #[redirect("/:name", |name: String| Route::BlogPost { name })]
//     #[end_nest]
//     #[route("/:..route")]
//     PageNotFound { route: Vec<String> },

//     #[layout(Wrapper)]
//     #[route("/index")]
//     Index {},
// }

// #[derive(Routable, Clone)]
// #[rustfmt::skip]
// enum Route {
//     #[layout(Footer)] // wrap the entire app in a footer
//         #[route("/")]
//         Home {},

//         #[route("/about")]
//         About {},

//         #[route("/index")]
//         Index {},

//         #[route("/blog")]
//         Blog {},
// }

// #[component]
// fn Footer() -> Element {
//     rsx! {
//         nav {
//             Link { to: Route::Home {}, class: "nav-btn", "Home" }
//             Link { to: Route::About {}, class: "nav-btn", "About" }
//             Link { to: Route::Index {}, class: "nav-btn", "Index" }
//             Link { to: Route::Blog {}, class: "nav-btn", "Blog" }
//         }
//         div { id: "content",
//             Outlet::<Route> {}
//         }
//     }
// }

#[derive(Routable, Clone)]
#[rustfmt::skip]
enum Route {
    #[layout(Wrapper)]
        #[route("/")]
        Home {}
}

#[component]
fn Wrapper() -> Element {
    rsx! {
        header { "header" }
        // The index route will be rendered here
        Outlet::<Route> {}
        footer { "footer" }
    }
}

#[component]
fn Home() -> Element {
    // let nav = navigator();

    // // push
    // nav.push(Route::PageNotFound { route: vec![] });

    // // replace
    // nav.replace(Route::Home {});

    // // go back
    // nav.go_back();

    // // go forward
    // nav.go_forward();

    rsx! {
        h1 { "Dan" }
        img { src: "{PROFILE}" }
        img { src: "{TOTODILE}"}
    }
}

#[component]
fn About() -> Element {
    rsx! { h1 { "About Me" }
        AboutMe {}


        img { src: "{CINCINNATI}" }
        img { src: "{BOSTON}" }
        img { src: "{EGGS}" }
        img { src: "{MIRADOR_CONDOR}" }
        img { src: "{LINDNER}" }
        img { src: "{LOGO}" }
        img { src: "{PHILLY}" }
        img { src: "{PRINCETON}" }
        img { src: "{SBU}" }
        img { src: "{UC}" }
    }
}

#[component]
fn Header() -> Element {
    rsx! {


            // div { class: "navbar",

            //     div {
            //         class: "logo",
            //         ul {
            //             li { img { src: "{LOGO}"} }
            //             li {"Dan"}

            //         }

            //     }
            //     div {
            //         class: "navigation",
            //         nav {
            //             class: "nav",
            //             ul {
            //                 li {
            //                     Link { to: Route::Home {}, "Home" }
            //                 }
            //                 li {
            //                     Link { to: Route::About {}, "About Me" }
            //                 }
            //                 li {
            //                     Link { to: Route::BlogList {  }, "Blog" }
            //                 }
            //             }
            //         }
            //     }

            // }

        Outlet::<Route> {}
    }
}

#[component]
fn Blog() -> Element {
    rsx! {
        h1 { "Blog" }
        Outlet::<Route> {}
    }
}

#[component]
fn BlogPost(name: String) -> Element {
    rsx! { h2 { "Blog Post: {name}" } }
}

#[component]
fn PageNotFound(route: Vec<String>) -> Element {
    rsx! {
        h1 { "Page not found" }
        p { "We are terribly sorry, but the page you requested doesn't exist." }
        pre { color: "red", "log:\nattemped to navigate to: {route:?}" }
    }
}

#[component]
fn NavBar() -> Element {
    rsx! {
        nav {
            ul { li { "links" } }
        }
        // The Outlet component will render child routes (In this case just the Home component) inside the Outlet component
        Outlet::<Route> {}
    }
}

#[component]
fn App() -> Element {
    rsx! {
        document::Stylesheet {
            // Urls are relative to your Cargo.toml file
            href: asset!("/assets/main.css")
        }

        header {
            class: "my-header",
            div { class: "me",
                ul {
                    li { img { src: "{LOGO}"} }
                    li { "Dan" }
                }
            }
            NavBar {}
            div { class: "nav",
                ul {
                    // li { Link { to: Route::Home {}, "Home"}}
                    li { Link { to: Route::Home {}, "Alex Horne" }}
                    li { "Greg Davies" }
                    li { "Josh Widdicombe" }
                    li { "Kathryn Ryan" }
                }
        }
        Router::<Route> {}
        }
        body {

            div {
                class: "container",
                "Words"
            }
        }
    }
}

fn main() {
    dioxus::launch(App);
}