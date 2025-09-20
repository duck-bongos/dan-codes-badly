use dan_codes_badly::protein_calculator::ProteinCalculator;
use dioxus::prelude::*;
use serde::{Deserialize, Serialize};

#[derive(Clone, Routable, Debug, PartialEq, Serialize, Deserialize)]
enum Route {
    #[route("/")]
    Home,
    #[route("/protein_calculator")]
    ProteinCalc,
}

static CSS: Asset = asset!("/assets/style.css");
static LOGO: Asset = asset!(
    "/assets/logo-removebg-preview.png",
    ImageAssetOptions::new()
        // You can set the image size in pixels at compile time to send the smallest possible image to the client
        .with_size(ImageSize::Manual {
            width: 71,
            height: 48,
        })
        // You can also convert the image to a web friendly format at compile time. This can make your images significantly smaller
        .with_format(ImageFormat::Png)
);
static RUST: Asset = asset!(
    "/assets/rust-logo-png-transparent.png",
    ImageAssetOptions::new()
        // You can set the image size in pixels at compile time to send the smallest possible image to the client
        .with_size(ImageSize::Manual {
            width: 50,
            height: 50
        })
        // You can also convert the image to a web friendly format at compile time. This can make your images significantly smaller
        .with_format(ImageFormat::Png)
);
static DIOXUS: Asset = asset!(
    "/assets/favicon.ico",
    ImageAssetOptions::new()
        .with_size(ImageSize::Manual {
            width: 50,
            height: 50
        })
        .with_format(ImageFormat::Png)
);

#[component]
pub fn TopNav() -> Element {
    rsx! {
        div {
            class: "mobile-container",
            div {
                class: "topnav",
                Link {
                    id: "logo-img",
                    to: Route::Home,
                    img {
                        class: "logo-img",
                        src: LOGO
                    }
                }
                div { id: "myLinks",

                    Link { class: "active",
                        to: Route::ProteinCalc,
                        "Protein Calculator"
                    }
                }

            }
    }
        Outlet::<Route> {}
    }
}

#[component]
fn Footer() -> Element {
    rsx! {
            div { class: "footer",
                div { class: "footer-ack",
                    "Powered by"
                }
                a {
                    href: "https://www.rust-lang.org/",
                    target: "_blank",
                    img { src: RUST}
                }
                a {
                    href: "https://dioxuslabs.com/",
                    target: "_blank",
                    img { src: DIOXUS }
                }

                div { class: "footer-site-promotion",
            }
            }
    }
}

#[component]
fn Home() -> Element {
    rsx! {
        document::Stylesheet { href: CSS }
        TopNav {  }
        h1 { "Dan Codes Badly"}
        p {
            "Hi there. My name is Dan. I write code, often not well. I am cetainly not a web developer. I wrote this website to learn three things:"
        }
        ol {
            li { "How to write a website - gotta start with a bad one."}
            li { "How to deploy a website - it worked!"}
            li { "How to write in Rust - opted to use the ", a { href: "https://dioxuslabs.com/", "Dioxus framework"}  "." }
        }
        Footer {  }
    }
}

#[component]
fn ProteinCalc() -> Element {
    rsx! {
        document::Stylesheet { href: CSS }
        TopNav { }
        ProteinCalculator { }
        Footer {}
    }
}

fn main() {
    launch(|| rsx! { Router::<Route> {} });
}
