use dioxus::prelude::*;
use crate::templates::pages::admin::layout;

#[derive(Props, PartialEq)]
pub struct DashboardProps {
    pub username: String,
    pub role: String
}
pub fn dashboard_page(cx: Scope<DashboardProps>) -> Element {
    let content = rsx!(
        div { class: "min-h-screen bg-gradient-to-r from-blue-500 to-indigo-600",
            div { class: "container mx-auto px-4 py-8",
                div { class: "dashboard-card bg-white bg-opacity-10 p-8 rounded-lg shadow-xl",
                    h1 { class: "text-3xl font-bold text-white mb-6",
                        "Welcome, {cx.props.username}"
                    }
                    div { class: "text-white mb-4",
                        "Role: {cx.props.role}"
                    }
                }
            }
        }
    );

    layout::admin_layout(cx, content)
}