/// 3.4 Iteration
/// 3.4.1 For Static Views: Vec<_>
use leptos::prelude::*;

#[component]
pub fn App() -> impl IntoView {
    let values = vec![0, 1, 2];
    view! {
        // this will just render "012"
        <p>{values.clone()}</p>
        // or we can wrap them in <li>
        <ul>
            {
                values.into_iter()
                    .map(|n| view! { <li>{n}</li>})
                    // .collect::<Vec<_>>()   // also works
                    .collect_view()
            }
        </ul>
    }
}

fn main() {
    console_error_panic_hook::set_once();
    mount_to_body(App)
}
