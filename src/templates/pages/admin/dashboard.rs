use dioxus::prelude::*;
use crate::templates::pages::admin::layout;

#[derive(Props, PartialEq)]
pub struct DashboardProps {
    pub username: String,
    pub role: String
}
pub fn dashboard_page(cx: Scope<DashboardProps>) -> Element {
    let content = rsx!(
        div { class: "bg-blue-100 h-screen font-sans",            header { class: "bg-header text-white p-4 shadow-md flex items-center h-10vh",
                img { 
                    class: "h-12 max-w-200 object-contain",
                    src: "/static/img/logo.png",
                    alt: "CatalystRSX Logo"
                }
            }
            div { class: "flex h-90vh",
                aside { class: "w-64 bg-blue-300 p-6",
                    div { class: "grid grid-cols-1 gap-4",
                        button { 
                            class: "bg-blue-600 hover:bg-blue-700 text-white font-semibold py-2 px-4 rounded-xl shadow w-full",
                            "Dashboard"
                        }
                        button { 
                            class: "bg-blue-600 hover:bg-blue-700 text-white font-semibold py-2 px-4 rounded-xl shadow w-full",
                            "Users"
                        }
                        button { 
                            class: "bg-blue-600 hover:bg-blue-700 text-white font-semibold py-2 px-4 rounded-xl shadow w-full",
                            "VHosts"
                        }
                        button { 
                            class: "bg-blue-600 hover:bg-blue-700 text-white font-semibold py-2 px-4 rounded-xl shadow w-full",
                            "Logout"
                        }
                    }                }
                main { class: "flex-1 p-6 bg-blue-100" }
            }
        }    );

    layout::admin_layout(cx, content)}