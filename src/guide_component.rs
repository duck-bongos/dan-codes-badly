

#[derive(Props, PartialEq, Clone)]
struct DogAppProps {
    breed: String,
}

#[component]
fn DogApp(breed: String) -> Element {
    tracing::info!("Rendered with breed: {breed}");

    todo!()
}
