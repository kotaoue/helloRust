use dioxus::prelude::*;

fn main() {
    launch(App);
}

#[component]
fn App() -> Element {
    let mut count = use_signal(|| 0);

    rsx! {
        h1 { "Hello, World!" }
        h2 { "Welcome to Dioxus!" }
        hr {}
        p { "This is a simple Dioxus application." }
        p { "Count: {count()}" }
        button {
            onclick: move |_| count.set(count() + 1),
            "+1"
        }
    }
}
