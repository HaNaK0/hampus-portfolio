use leptos::{
    ev,
    html::{br, button, div, p},
    prelude::*,
};

fn main() {
    mount_to_body(App);
}

#[allow(non_snake_case)]
#[component]
fn App() -> impl IntoView {
    console_error_panic_hook::set_once();
    let (count, set_count) = signal(0);
    let double_count = move || count.get() * 2;

    (
        button()
            .on(ev::click, move |_| *set_count.write() += 1)
            .style("position:absolute")
            .style(("left", move || format!("{}px", count.get() * 10)))
            .style(("--columns", move || count.get().to_string()))
            .child("Increase"),
        br(),
        ProgressBar(ProgressBarProps { max: 100, progress: count.into() }),
        p()
            .child(("Double count ", double_count))
        ,
    )
    // view! {
    //     <button
    //         on:click=move |_| *set_count.write() += 10
    //         style="position:absolute"
    //         style:left=move || format!("{}px", count.get() + 100)
    //         style:backgound-color=move || format!("rgb({}, 100, 100)", count.get())
    //         style:max-width="400px"
    //         style=("--columns", move || count.get().to_string())
    //     >
    //         "click me: " {count}
    //     </button>
    //
    //     <br/>
    //
    //     <ProgressBar progress=count/>
    //     <ProgressBar progress=Signal::derive(double_count)/>
    //
    //     <p>
    //         "Double count: "
    //         {double_count}
    //     </p>
    // }
}

#[allow(non_snake_case)]
#[component]
fn ProgressBar(
    #[prop(default = 100)] max: u16,
    #[prop(into)] progress: Signal<i32>,
) -> impl IntoView {
    view! {
        <progress
            max=max
            value=progress
        />
        <br/>
    }
}
