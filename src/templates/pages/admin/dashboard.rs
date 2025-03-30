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
                    div {
                        class: "tab-menu",
                        div {
                            class: "tab",
                            "Overview"
                        }
                        div {
                            class: "tab",
                            "Users"
                        }
                        div {
                            class: "tab",
                            "Security"
                        }
                        div {
                            class: "tab",
                            "Logs"
                        }
                        div {
                            class: "tab",
                            "Settings"
                        }
                        div {
                            class: "tab",
                            "Domains"
                        }
                        div {
                            class: "tab",
                            "SSL"
                        }
                        div {
                            class: "tab",
                            "Backups"
                        }
                        div {
                            class: "tab",
                            "Updates"
                        }
                        div {
                            class: "tab",
                            "Reports"
                        }
                    }
                    style { {
                        ".tab-menu {{
                            display: flex;
                            background: #2c3e50;
                            border-radius: 5px 5px 0 0;
                            overflow: hidden;
                        }}
                        .tab {{
                            width: 100px;
                            height: 50px;
                            display: flex;
                            align-items: center;
                            justify-content: center;
                            color: white;
                            cursor: pointer;
                            transition: background 0.3s;
                            font-weight: 500;
                        }}
                        .tab:hover {{
                            background: #34495e;
                        }}
                        .tab.active {{
                            background: #3498db;
                        }}"
                    }}                }
            }
        }
    })
}