/// 3.4 Iteration
/// 3.4.1b For Static Views with Dynamic Content: Vec<_>
use leptos::prelude::*;

#[component]
pub fn App() -> impl IntoView {
    // create a list of 5 signals
    let length = 5;
    let counters = (1..=length).map(|idx| RwSignal::new(idx));
    // each item manages a reactive view
    // but the list itself will never change
    let counter_buttons = counters
        .map(|count| {
            view! {
                <li>
                    <button
                        on:click=move |_| *count.write() += 1
                    >
                        {count}
                    </button>
                </li>
            }
        })
        .collect_view();

    view! {
        <ul>{counter_buttons}</ul>
    }
}

fn main() {
    console_error_panic_hook::set_once();
    mount_to_body(App)
}
