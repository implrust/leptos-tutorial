use leptos::prelude::*;
mod app2;
use app2::App;

fn main() {
    console_error_panic_hook::set_once();
    mount_to_body(App)
}
