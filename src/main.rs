mod eng;

use core::time;
use dioxus::prelude::*;
use eng::UxItem;
use std::thread::{self, sleep};

use crate::eng::{show_leanness, GroceryItem};
// #[derive(PartialEq, Clone)]
// struct DogAppProps {
//     breed: String,
// }
pub const CSS: Asset = asset!("/assets/thing.css");

#[derive(Clone, Copy)]
struct DarkMode(bool);

// pub fn DarkModeToggle() -> Element {
//     let mut dark_mode = use_context::<Signal<DarkMode>>();

//     let style = if dark_mode().0 { "color:white" } else { "" };

//     rsx! {
//         label { style: "{style}",
//             "Dark Mode"
//             input {
//                 r#type: "checkbox",
//                 oninput: move |event| {
//                     let is_enabled = event.value() == "true";
//                     dark_mode.write().0 = is_enabled;
//                 }
//             }
//         }
//     }
// }

#[derive(Clone)]
struct TitleState(String);
#[component]

fn Title() -> Element {
    let title = use_context::<TitleState>();
    rsx! {
        h1 {"{title.0}"}
    }
}

#[derive(Clone)]
struct ListItem(String);

#[derive(Clone)]
struct Item<T, U, V, W>(Vec<UxItem<T, U, V, W>>)
where
    f64: From<T>,
    f64: From<U>,
    f64: From<V>,
    f64: From<W>;

#[component]
fn DoItem() -> Element {
    rsx! {
        li { "item" }
    }
}

#[component]
fn ProteinItems(arr: Vec<GroceryItem>) -> Element {
    rsx! {

        ol { class: "output-items",
                    for item in &arr {
                        li { "{item}" }
                    }
                 }

    }
}

fn calculate(
    arr: &mut Vec<GroceryItem>,
    name: String,
    protein: f64,
    calories: f64,
    cost: f64,
    servings: f64,
) {
    tracing::debug!("Added '{name}' to the list!");
    let _uxi = UxItem {
        name: name,
        protein: protein,
        calories: calories,
        cost: cost,
        servings: servings,
    };

    arr.push(_uxi.to_grocery());
    tracing::debug!("Array new size: {}", &arr.len())
}

pub fn App() -> Element {
    let mut name: Signal<String> = use_signal(|| String::from(""));
    let mem_name = use_memo(move || name());
    let mut protein = use_signal(|| 0.0);
    let mem_protein = use_memo(move || protein());
    let mut calories = use_signal(|| 0.0);
    let mem_calories = use_memo(move || calories());
    let mut cost = use_signal(|| 0.0);
    let mem_cost = use_memo(move || cost());
    let mut servings = use_signal(|| 1.0);
    let mem_servings = use_memo(move || servings());
    let mut grocery_items: Signal<Vec<GroceryItem>> = use_signal(|| vec![]);
    let mut read_groceries = use_memo(move || grocery_items().clone());

    use_context_provider(|| TitleState("Protein Comparison Calculator".to_string()));
    // use_effect(move || {
    //     let _protein = protein();
    //     let _name = name();
    //     let _calories = calories();
    //     let _cost = cost();
    //     let _servings = servings();

    //     tracing::debug!(
    //         "(${cost}) '{_name}' | p: {_protein} kcal: {_calories} servings: {_servings}"
    //     )
    // });

    rsx! {
    document::Stylesheet { href: CSS }
    Title {}

    // div {
    //     h1 { "Input name: {name}"}

    // }
    div { class: "flex-container",
        div { class: "div-form",
            h3 { "Input Factors" }
            form { class: "responsive-form",
                div { class: "form-group",
                    label { for: "name", "Protein Source Name:"}
                    input { type: "text", id: "name", name: "name", value:"{name}", oninput: move |e| name.set(e.value())}
                }
                div { class: "form-group",
                        label { for: "protein", "Protein (g):"}
                        input { type: "number", id: "protein", name: "protein", value: protein, oninput: move |e| {
                                let _is = e.value();
                                match _is.parse::<f64>() {
                                    Ok(parsed_val) => protein.set(parsed_val),
                                    Err(v) => tracing::info!("{v} Naaaahhhh")
                                }
                            }
                        }
                    }
                }
                div { class: "form-group",
                    label { for: "calories", "Calories per Serving:"}
                    input { type: "number", id: "calories", name: "calories", value: calories, oninput: move |e| {
                                let _is = e.value();
                                match _is.parse::<f64>() {
                                    Ok(parsed_val) => calories.set(parsed_val),
                                    Err(_) => tracing::info!("Naaaahhhh")
                                }
                            }
                    }
                }
                div { class: "form-group",
                    label { for: "cost", "Total Cost:"}
                    input { type: "number", id: "cost", name: "cost", min: 0.01, value: cost, oninput: move |e| {
                                let _is = e.value();
                                match _is.parse::<f64>() {
                                    Ok(parsed_val) => cost.set(parsed_val),
                                    Err(v) => tracing::info!("{v} Naaaahhhh")
                                }
                            }}
                }
                div { class: "form-group",

                    label { for: "servings", "Total Servings:"}
                    input { type: "number", id: "servings", name: "servings",value: servings, oninput: move |e| {
                                let _is = e.value();
                                match _is.parse::<f64>() {
                                    Ok(parsed_val) => servings.set(parsed_val),
                                    Err(v) => tracing::info!("{v} Naaaahhhh")
                                }
                            }}
                }
                div { class: "input-form-buttons",
                input { class: "form-button", type: "submit", value: "Add", onclick: move |_| {
                    let _uxi = UxItem {
                        name: name(),
                        protein: protein(),
                        calories: calories(),
                        cost: cost(),
                        servings: servings(),
                    }.to_grocery();

                    grocery_items.push(_uxi);

                    // calculate(&mut grocery_items(), mem_name(), mem_protein(), mem_calories(), mem_cost(), mem_servings());
                    name.set(String::new());
                    protein.set(0.0);
                    calories.set(0.0);
                    cost.set(0.0);
                    servings.set(1.0);
                }}
                input { class: "form-button", type: "reset", value: "Clear", onclick: move |_| {
                    name.set(String::new());
                    protein.set(0.0);
                    calories.set(0.0);
                    cost.set(0.0);
                    servings.set(1.0);
                }}
            }
        }



        div { id: "protein-items", class: "div-form",
            h3 { "Protein Items" }
            h4 { "Decide how to Sort" }
            input { class: "form-button", type: "reset", value: "Clear Items", onclick: move |_| {
                grocery_items.set(vec![]);
                // name.set(String::new());
                //     protein.set(0.0);
                //     calories.set(0.0);
                //     cost.set(0.0);
                //     servings.set(1.0);
            }}
            div { class: "input-form-buttons",
                input { class: "form-button", type: "submit", value: "Leanness", onclick: move |_| {
                    tracing::info!("Leanness: {:?}", grocery_items());
                    grocery_items.write().sort_by(|a, b| a.leanness.cmp(&b.leanness))
                }}
                input { class: "form-button", type: "submit", value: "Protein per Dollar", onclick: move |_| {
                    tracing::info!("Protein per Dollar: {:?}", grocery_items());
                    grocery_items.write().sort_by(|a, b| b.ppd.cmp(&a.ppd));
                }}
                input { class: "form-button", type: "submit", value: "Leanness per Dollar", onclick: move |_| {
                    tracing::info!("Protein per Dollar: {:?}", grocery_items());
                    grocery_items.write().sort_by(|a, b| a.lpd.cmp(&b.lpd));
                }}

            }

            ol {
                for item in grocery_items.read().iter() {
                    li { "{item}" }
                }
            }
        }
        }
    }
}

fn main() {
    dioxus::launch(App);
}
