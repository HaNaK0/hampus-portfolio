use leptos::prelude::*;
use leptos_conf_markdown::Markdown;
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
    let file_content = LocalResource::new(move || read_file("./resources/hello_world.md"));
    view! { 
        <Markdown path="./texts/hello_world.md" /> 
        <p>{file_content.get()}</p>
    }
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

async fn read_file(_path: &str) -> String {
    let this_page = web_sys::window()
        .expect("failed to get web sys wondow")
        .location()
        .href()
        .expect("failed to get url");

    info!("this page is \"{}\"", this_page);
    this_page.to_string()
}
