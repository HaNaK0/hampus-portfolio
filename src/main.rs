use leptos::{
    prelude::*,
};
use log::info;
use leptos_conf_markdown::Markdown;

fn main() {
    // makes sure panics shows up as readble errors in the web console
    console_error_panic_hook::set_once();
    console_log::init_with_level(log::Level::Trace).expect("Failed to start logger");
    info!("Logger Attached");
    mount_to_body(App);
}

#[allow(non_snake_case)]
#[component]
fn App() -> impl IntoView {
    view! { <Markdown path="./texts/hello_world.md"/>}
}

#[allow(non_snake_case)]
#[component]
fn Nav() -> impl IntoView {
    view! {
        <nav>
            <ul>
                <li>
                    <a href="/">"Bio"</a>
                </li>
                <li>
                    <a href="/sailor">"Sailor"</a>
                </li>
                <li>
                    <a href="/game-dev">"Game Dev"</a>
                </li>
            </ul>
        </nav>
    }
}

