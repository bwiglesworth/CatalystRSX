use dioxus::prelude::*;

pub fn dashboard_page(cx: Scope) -> Element {
    cx.render(rsx! {
        div { class: "admin-dashboard",
            h1 { "Admin Dashboard" }
        }
    })
}
