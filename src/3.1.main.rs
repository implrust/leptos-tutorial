//3.1 A Basic Component
use leptos::prelude::*;

/// optional props
#[component]
pub fn ProgressBar(
    // #[prop(optional)]
    #[prop(default = 50)] max: u16,
    #[prop(into)] progress: Signal<i32>,
) -> impl IntoView {
    view! {
        <progress max={max} value=progress />
        <br/>
    }
}

#[component]
pub fn App() -> impl IntoView {
    let (count, set_count) = signal(0);
    let double_count = move || count.get() * 2;

    view! {
        <button
            on:click=move |_| {
                *set_count.write() += 1;
            }
            class:red=move || count.get() % 4 == 1
            class=("green", move || count.get() % 4 == 2)
            class=(["blue","bgwhite"], move || count.get() % 4 == 3)
        >
            "Click me"
        </button>

        <br/>
        <ProgressBar progress=count />
        <br/>
        <p>"Count: " {count}</p>

        <br/>
        <ProgressBar max=50 progress=Signal::derive(double_count) />
        <p>"Double Count: " {double_count}</p>
        <br/>
    }
}

fn main() {
    console_error_panic_hook::set_once();
    mount_to_body(App)
}
