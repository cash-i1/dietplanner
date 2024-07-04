use leptos::*;

#[component]
fn App() -> impl IntoView {
    view! {
        <Home/>
    }
}

#[component]
fn Home() -> impl IntoView {}

fn main() {
    mount_to_body(move || view !{ <App/> })
}
