/// 3.5 Iterating over More Complex Data
use leptos::prelude::*;

#[component]
fn App() -> impl IntoView {
    view! {
        <h5>"3.5 Iterating over More Complex Data"</h5>
    }
}

fn main() {
    console_error_panic_hook::set_once();
    mount_to_body(App)
}
