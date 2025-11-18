use std::path::{Path};

use leptos::prelude::*;

#[derive(Default, Debug)]
pub struct MarkdownClasses {
    pub p: String,
}

#[allow(non_snake_case)]
#[component]
pub fn Markdown(path: impl AsRef<Path>) -> impl IntoView {
    let path_string = path
        .as_ref()
        .to_str()
        .expect("path needs to be utf8")
        .to_string();
    view! { <p>{path_string}</p> }
}

#[cfg(test)]
mod tests {}
