use leptos::{
    html::{h1, p},
    prelude::*,
};

fn main() {
    mount_to_body(App);
}

#[allow(non_snake_case)]
#[component]
fn App() -> impl IntoView {
    // makes sure panics shows up as readble errors in the web console
    console_error_panic_hook::set_once();

    (
        h1().child("Hampus portfolio"),
        p().child("This is just a beginning written in rust using leptos"),
    )
}
