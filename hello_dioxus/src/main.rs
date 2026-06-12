use dioxus::prelude::*;
use dioxus_ssr::render;

fn main() {
    let mut dom = VirtualDom::new(App);
    dom.rebuild_in_place();

    let html = render(&dom);
    println!("{html}");
}

#[component]
fn App() -> Element {
    rsx! {
        h1 { "Hello, World!" }
        h2 { "Welcome to Dioxus!" }
        p { "This is a simple Dioxus application." }
    }
}
