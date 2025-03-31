use dioxus::prelude::*;
use crate::templates::pages::layout::layout;

pub fn index_page(cx: Scope) -> Element {
    let show_menu = use_state(cx, || false);
    let toggle_menu = move |_| {
        show_menu.set(!show_menu.get());
    };
    
    let content = rsx!(
        div { class: "min-h-screen bg-gradient-to-r from-blue-500 to-indigo-600",
            nav { class: "bg-white bg-opacity-10 p-4",
                div { class: "container mx-auto flex justify-between items-center",
                    h1 { class: "text-2xl font-bold text-white", "CatalystRSX" }
                    div { class: "space-x-4",
                        button { 
                            class: "text-white hover:text-yellow-400",
                            onclick: toggle_menu,
                            svg {
                                class: "w-10 h-10",
                                fill: "none",
                                stroke_linecap: "round",
                                stroke_linejoin: "round",
                                stroke_width: "3",
                                view_box: "0 0 30 30",
                                stroke: "currentColor",
                                path {
                                    d: "M4 6h16M4 12h16M4 18h16"
                                }
                            }
                        }
                        div { 
                            class: if *show_menu.get() { 
                                "absolute right-0 mt-2 w-48 py-2 bg-white bg-opacity-10 backdrop-blur-lg rounded-md shadow-xl" 
                            } else { 
                                "absolute right-0 mt-2 w-48 py-2 bg-white bg-opacity-10 backdrop-blur-lg rounded-md shadow-xl hidden" 
                            },
                            a {
                                class: "block px-4 py-2 text-white hover:bg-white hover:bg-opacity-20",
                                href: "/admin/login",
                                "Login"
                            }
                        }
                    }
                }
            }
            main { class: "container mx-auto px-4 py-16",
                div { class: "text-center text-white",
                    h2 { 
                        class: "text-5xl font-bold mb-8", 
                        "Enterprise Web Software as a Service"
                    }
                    p { 
                        class: "text-xl mb-12", 
                        "A Security First Approach"
                    }                    
                    div { class: "grid grid-cols-1 md:grid-cols-2 lg:grid-cols-4 gap-8 mt-16",
                        div { class: "bg-white bg-opacity-10 p-6 rounded-lg transform hover:scale-105 transition-transform",
                            h3 { class: "text-xl font-semibold mb-4", "Rate Limiting" }
                            p { "Configurable request throttling with burst protection" }
                        }
                        div { class: "bg-white bg-opacity-10 p-6 rounded-lg transform hover:scale-105 transition-transform",
                            h3 { class: "text-xl font-semibold mb-4", "Security Headers" }
                            p { "CSP, HSTS, and XSS protection with secure session management" }
                        }
                        div { class: "bg-white bg-opacity-10 p-6 rounded-lg transform hover:scale-105 transition-transform",
                            h3 { class: "text-xl font-semibold mb-4", "TLS Security" }
                            p { "Modern TLS configuration with certificate management" }
                        }
                        div { class: "bg-white bg-opacity-10 p-6 rounded-lg transform hover:scale-105 transition-transform",
                            h3 { class: "text-xl font-semibold mb-4", "Input Validation" }
                            p { "Comprehensive request validation and sanitization" }
                        }
                    }
                }
            }
            footer { class: "fixed bottom-0 w-full bg-white bg-opacity-10 py-4 z-10",
                div { class: "container mx-auto text-center text-white",
                    p { "Powered by Rust & Dioxus | Enterprise-grade security for modern web applications" }
                }
            }
        }
    );

    layout(cx, content)
}