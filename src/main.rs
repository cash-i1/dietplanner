use leptos::*;

#[component]
fn App() -> impl IntoView {
    view! {
        <Home/>
    }
}

#[component]
fn Home() -> impl IntoView {
    view! {
        <div class="root">
            <div class="center-box">
                <h1>diet tracker</h1>
            </div>
        </div>
    }
}

fn main() {
    mount_to_body(move || view !{ <App/> })
}
