use dioxus::prelude::*;

pub fn layout(cx: Scope, children: Element) -> Element {
    cx.render(rsx! {
        html {
            head {
                meta { charset: "utf-8" }
                meta { name: "viewport", content: "width=device-width, initial-scale=1.0" }
                link { rel: "stylesheet", href: "/static/css/tailwind.css" }
                link { rel: "stylesheet", href: "/static/css/main.css" }
                link { rel: "stylesheet", href: "https://cdn.jsdelivr.net/npm/tailwindcss@2.2.19/dist/tailwind.min.css" }
                script { src: "/static/js/app.js", defer: true }
            }
            body {
                children
            }
        }
    })
}