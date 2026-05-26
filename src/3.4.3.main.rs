/// 3.4 Iteration
/// 3.4.3 Accessing an index while iterating with <ForEnumerate/>
use leptos::prelude::*;

#[derive(Copy, Clone, Debug, PartialEq, Eq)]
struct Counter {
    id: usize,
    count: RwSignal<i32>,
}

#[component]
fn App() -> impl IntoView {
    let initial_counters = (0..5)
        .map(|id| Counter {
            id,
            count: RwSignal::new((id + 1) as i32),
        })
        .collect::<Vec<_>>();
    let (counters, _set_counters) = signal(initial_counters);
    view! {
        <ForEnumerate
            each=move || counters.get() // Same as <For/>
            key=|counter| counter.id    // Same as <For/>
            // Provides the index as a signal and the child T
            children={move |index: ReadSignal<usize>, counter: Counter| {
                view! {
                    <button>
                        {move || index.get()} ". Value: " {move || counter.count.get()}
                    </button>
                }
            }}
        />
        <hr/>
        <ForEnumerate
            each=move || counters.get() // Same as <For/>
            key=|counter| counter.id    // Same as <For/>
            let(idx, counter)           // let syntax
        >
            <button>
            {move || idx.get()} ". Value: " {move || counter.count.get()}
            </button>
        </ ForEnumerate>
    }
}

fn main() {
    console_error_panic_hook::set_once();
    mount_to_body(App)
}
