use dioxus::prelude::*;

pub fn layout<'a>(cx: Scope<'a>, children: LazyNodes<'a, 'a>) -> Element<'a> {    cx.render(rsx! {
        div {
            meta { charset: "utf-8" }
            meta { name: "viewport", content: "width=device-width, initial-scale=1.0" }
            link { rel: "stylesheet", href: "/static/css/tailwind.css" }
            link { rel: "stylesheet", href: "/static/css/main.css" }
            script { src: "/static/js/app.js", defer: true }
            children
        }
    })
}