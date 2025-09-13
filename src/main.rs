mod eng;

use dioxus::prelude::*;
use eng::UxItem;

use crate::eng::GroceryItem;
// #[derive(PartialEq, Clone)]
// struct DogAppProps {
//     breed: String,
// }
pub const CSS: Asset = asset!("/assets/thing.css");

#[derive(Clone)]
struct TitleState(String);
#[component]

fn Title() -> Element {
    let title = use_context::<TitleState>();
    rsx! {
        h1 { "{title.0}" }
    }
}

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

pub fn app() -> Element {
    let mut name: Signal<String> = use_signal(|| String::from(""));
    let mut protein = use_signal(|| 0.0);
    let mut calories = use_signal(|| 0.0);
    let mut cost = use_signal(|| 0.0);
    let mut servings = use_signal(|| 1.0);
    let mut grocery_items: Signal<Vec<GroceryItem>> = use_signal(|| vec![]);
    let mut sort_label: Signal<String> = use_signal(|| {
        String::from(
            "Leanness = Calories / Protein. Lower = Leaner. Chicken Breast is approximately 5.51",
        )
    });

    use_context_provider(|| TitleState("Protein Comparison Calculator".to_string()));

    rsx! {
        document::Stylesheet { href: CSS }
        Title {}

        div { class: "flex-container",
            div { class: "div-form",
                h3 { "Input Factors" }
                form { class: "responsive-form",
                    div { class: "form-group",
                        label { r#for: "name", "Protein Source Name:" }
                        input {
                            r#type: "text",
                            id: "name",
                            name: "name",
                            value: "{name}",
                            oninput: move |e| name.set(e.value()),
                        }
                    }
                    div { class: "form-group",
                        label { r#for: "protein", "Protein (g):" }
                        input {
                            r#type: "number",
                            id: "protein",
                            name: "protein",
                            value: protein,
                            oninput: move |e| {
                                let _is = e.value();
                                match _is.parse::<f64>() {
                                    Ok(parsed_val) => protein.set(parsed_val),
                                    Err(v) => tracing::info!("{v} Naaaahhhh"),
                                }
                            },
                        }
                    }
                }
                div { class: "form-group",
                    label { r#for: "calories", "Calories per Serving:" }
                    input {
                        r#type: "number",
                        id: "calories",
                        name: "calories",
                        value: calories,
                        oninput: move |e| {
                            let _is = e.value();
                            match _is.parse::<f64>() {
                                Ok(parsed_val) => calories.set(parsed_val),
                                Err(_) => tracing::info!("Naaaahhhh"),
                            }
                        },
                    }
                }
                div { class: "form-group",
                    label { r#for: "cost", "Total Cost:" }
                    input {
                        r#type: "number",
                        id: "cost",
                        name: "cost",
                        min: 0.01,
                        value: cost,
                        oninput: move |e| {
                            let _is = e.value();
                            match _is.parse::<f64>() {
                                Ok(parsed_val) => cost.set(parsed_val),
                                Err(v) => tracing::info!("{v} Naaaahhhh"),
                            }
                        },
                    }
                }
                div { class: "form-group",

                    label { r#for: "servings", "Total Servings:" }
                    input {
                        r#type: "number",
                        id: "servings",
                        name: "servings",
                        value: servings,
                        oninput: move |e| {
                            let _is = e.value();
                            match _is.parse::<f64>() {
                                Ok(parsed_val) => servings.set(parsed_val),
                                Err(v) => tracing::info!("{v} Naaaahhhh"),
                            }
                        },
                    }
                }
                div { class: "input-form-buttons",
                    input {
                        class: "form-button",
                        r#type: "submit",
                        value: "Add",
                        onclick: move |_| {
                            let _uxi = UxItem {
                                name: name(),
                                protein: protein(),
                                calories: calories(),
                                cost: cost(),
                                servings: servings(),
                            }
                                .to_grocery();
                            grocery_items.push(_uxi);
                            name.set(String::new());
                            protein.set(0.0);
                            calories.set(0.0);
                            cost.set(0.0);
                            servings.set(1.0);
                        },
                    }
                    input {
                        class: "form-button",
                        r#type: "reset",
                        value: "Clear",
                        onclick: move |_| {
                            name.set(String::new());
                            protein.set(0.0);
                            calories.set(0.0);
                            cost.set(0.0);
                            servings.set(1.0);
                        },
                    }
                }
            }

            div { id: "protein-items", class: "div-form",
                h3 { "Protein Items" }
                h4 { "Decide how to Sort" }
                div { class: "sort-label",
                    h5 { class: "", "{sort_label}" }
                }
                input {
                    class: "form-button",
                    r#type: "reset",
                    value: "Clear Items",
                    onclick: move |_| {
                        grocery_items.set(vec![]);
                    },
                }
                div { class: "input-form-buttons",
                    input {
                        class: "form-button",
                        r#type: "submit",
                        value: "Leanness",
                        onclick: move |_| {
                            tracing::info!("Leanness: {:?}", grocery_items());
                            grocery_items.write().sort_by(|a, b| a.leanness.cmp(&b.leanness));
                            sort_label
                                .set(
                                    String::from(
                                        "Leanness = Calories / Protein. Lower = Leaner. Chicken Breast is approximately 5.51, ",
                                    ),
                                )
                        },
                    }
                    input {
                        class: "form-button",
                        r#type: "submit",
                        value: "Protein per Dollar",
                        onclick: move |_| {
                            tracing::info!("Protein per Dollar: {:?}", grocery_items());
                            grocery_items.write().sort_by(|a, b| b.ppd.cmp(&a.ppd));
                            sort_label
                                .set(
                                    String::from(
                                        "Protein (g) multiplied by Servings all divided by total Cost.",
                                    ),
                                )
                        },
                    }
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
    dioxus::launch(app);
}
