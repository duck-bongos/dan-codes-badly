mod asset_mgt;
use dioxus::prelude::*;

use asset_mgt::{
    BOSTON, CINCINNATI, EGGS, LINDNER, LOGO, MIRADOR_CONDOR, PHILLY, PRINCETON, PROFILE, SBU,
    TOTODILE, UC,
};
fn main() {
    dioxus::launch(|| {
        rsx! {
            document::Link { rel: "stylesheet", href: asset!("/assets/main.css") }
            Router::<Route> {}
        }
    })
}

#[derive(Routable, Clone)]
#[rustfmt::skip]
enum Route {
    #[layout(Footer)] // wrap the entire app in a footer
        #[route("/")]
        Home {},

        #[route("/career")]
        Career {},

        #[route("/personal")]
        Personal {},

        #[route("/achievements")]
        Achievements {},

}

#[component]
fn Footer() -> Element {
    rsx! {
        // img { id: "img-logo", class: "logo", src: "{LOGO}" }
        div { class: "navigation",
            // Link { to: Route::Home {}, img { id: "img-logo", class: "logo", src: "{LOGO}" }}
            nav { class: "navbar",
                Link { to: Route::Home {}, class: "nav-btn", "Home" }
                Link { to: Route::Career {}, class: "nav-btn", "Career" }
                Link { to: Route::Personal {}, class: "nav-btn", "My Story" }
                Link { to: Route::Achievements {}, class: "nav-btn", "Achievements" }
            }
        }
        div { id: "content", Outlet::<Route> {} }
    }
}

#[component]
fn Home() -> Element {
    rsx!(
        div { id: "home-content",

            h1 { class: "page-title", "Home" }
            // img { class: "profile-img", src: "{PROFILE}"}
            div { class: "page-content",
                p {

                    "Hi! My name is Dan a.k.a duck-bongos. I like striving for magis, or “more” excellence in my life,
            particularly around cognitive curiosities, athletic goals, and relationships with people. Feel free 
            to poke around, read more, or send me an email."
                }

                p {
                    "If you're a recruiter interested in hiring me, please first check my resume for a potential fit.
            Then, if you think navigate to the Professional section, find the Easter Egg Instructions to show 
            you're interested (my mom found it in 13 seconds, can you?)."
                }

                p { "Otherwise, I hope you enjoy this! I certainly had fun making it." }
            }
        }
    )
}

#[component]
fn Career() -> Element {
    rsx!(
        div { class: "page-content",
            h1 { class: "page-title", "Career" }
            // Dummy text that talks about video games
            p { "Lorem career are sit amet  Sed do eiusmod tempor et dolore magna aliqua." }
        }
    )
}

#[component]
fn Personal() -> Element {
    rsx!(
        div { class: "page-content",
            h1 { class: "page-title", "My Story" }
        }
    )
}

#[component]
fn Achievements() -> Element {
    rsx!(
        div { class: "page-content",
            h1 { class: "page-title", "Achievements" }
            p {
                "Achievements are consectetur adipiscing elit. Sed do eiusmod tempor incididunt ut labore et dolore magna aliqua."
            }
        }
    )
}
