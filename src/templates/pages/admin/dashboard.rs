use dioxus::prelude::*;
#[derive(Props, PartialEq)]
pub struct DashboardProps {
    pub username: String,
    pub role: String
}
pub fn dashboard_page(cx: Scope<DashboardProps>) -> Element {
    cx.render(rsx! {
        head {
            link {
                rel: "stylesheet",
                href: "https://cdn.jsdelivr.net/npm/tailwindcss@2.2.19/dist/tailwind.min.css"
            }
        }
        div {
            class: "min-h-screen bg-gradient-to-r from-blue-500 to-indigo-600",
            div {
                class: "container mx-auto px-4 py-8",
                div {
                    class: "bg-white bg-opacity-10 p-8 rounded-lg shadow-xl",
                    h1 {
                        class: "text-3xl font-bold text-white mb-6",
                        "Admin Dashboard"
                    }
                    div {
                        class: "text-white mb-4",
                        "Welcome, {cx.props.username} ({cx.props.role})"
                    }
                }
            }
        }
    })
}