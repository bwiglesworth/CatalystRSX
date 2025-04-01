use dioxus::prelude::*;

pub fn admin_layout<'a, T>(cx: Scope<'a, T>, children: LazyNodes<'a, 'a>) -> Element<'a> {    cx.render(rsx! {
        div {
            meta { charset: "utf-8" }
            meta { name: "viewport", content: "width=device-width, initial-scale=1.0" }
            meta { name: "robots", content: "noindex, nofollow" }
            link { rel: "stylesheet", href: "/static/css/tailwind.css" }
            link { rel: "stylesheet", href: "/static/css/main.css" }
            script { src: "/static/js/app.js", defer: true }
            children
        }
    })
}