use leptos::{
    html::{h1, p},
    prelude::*,
};
use log::info;

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
    (
        h1().child("Hampus portfolio").class("bg-sky-500"),
        p().child("This is just a beginning written in rust using leptos"),
    )
}
