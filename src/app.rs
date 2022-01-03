use yew::prelude::*;

#[function_component(App)]
pub fn app() -> Html {
    html! {
        <main>
            <img class="logo" src="https://yew.rs/img/logo.png" alt="Yew logo" />
            <h1>{ "This will be a portfolio!" }</h1>
            <span class="subtitle">{ "built with yew" }<i class="heart" /></span>
        </main>
    }
}
