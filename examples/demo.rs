use dioxus::prelude::*;
use dioxus_library_template::prelude::*;

fn main() {
    dioxus_desktop::launch(app);
}

fn app(cx: Scope) -> Element {
    let msg = use_dioxus(cx);

    render! {
        div {
            display: "flex",
            justify_content: "center",
            h3 {
                "{msg}"
            }
        }
    }
}
