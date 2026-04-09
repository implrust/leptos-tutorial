// 3.2 Dynamic Attributes
use leptos::prelude::*;

/// optional props: function
#[component]
pub fn ProgressBar(
    #[prop(optional)] progress: Option<Box<dyn Fn() -> i32 + Send + Sync>>,
) -> impl IntoView {
    if let Some(p) = progress {
        view! {
            <progress max=100 value=p />
            <br/>
        }
        .into_any()
    } else {
        // ().into_any()
        view! {
            <progress max=100 value=0 />
            <br/>
        }
        .into_any()
    }
}

#[component]
pub fn App() -> impl IntoView {
    let (count, set_count) = signal(0);
    let double_count = move || count.get() * 2;
    let html = "<p>This HTML will be injected.</p>";
    view! {
        <button
            on:click=move |_| {
                *set_count.write() += 10;
            }
            // set the `style` attribute
            style="position: absolute"
            // and toggle individual CSS properties with `style:`
            style:left=move || format!("{}px", count.get() + 100)
            style:background-color=move || format!("rgb({}, {}, 100)", count.get(), 100)
            style:max-width="400px"
            // Set a CSS variable for stylesheet use
            style=("--columns", move || count.get().to_string())
        >
            "Click to Move"
        </button>
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
        <ProgressBar progress=Box::new(move || count.get()) />
        <br/>
        <p>"Count: " {count}</p>

        <br/>
        <ProgressBar progress=Box::new(double_count) />
        <p>"Double Count: " {double_count}</p>
        <br/>
        <ProgressBar />
        <br/>
        <div inner_html=html/>
    }
}

fn main() {
    console_error_panic_hook::set_once();
    mount_to_body(App)
}
