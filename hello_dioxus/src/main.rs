use dioxus::prelude::*;
use dioxus_ssr::render;

mod html;

fn main() {
    let mut dom = VirtualDom::new(App);
    dom.rebuild_in_place();

    let html = render(&dom);
    println!("{}", html::pretty_print_html(&html));
}

#[component]
fn App() -> Element {
    rsx! {
        h1 { "Hello, World!" }
        h2 { "Welcome to Dioxus!" }
        hr {}
        p { "This is a simple Dioxus application." }
    }
}
