use dioxus::prelude::*;

pub fn login_page(cx: Scope) -> Element {
    let error = use_state(cx, || None::<String>);
    
    cx.render(rsx! {
        head {
            link { rel: "stylesheet", href: "https://cdn.jsdelivr.net/npm/tailwindcss@2.2.19/dist/tailwind.min.css" }
        }
        div { class: "min-h-screen bg-gradient-to-r from-blue-500 to-indigo-600 flex items-center justify-center",
            div { class: "bg-white bg-opacity-10 p-8 rounded-lg shadow-xl w-96",
                h2 { class: "text-3xl font-bold text-white mb-6 text-center", "Admin Login" }
                {error.get().as_ref().map(|err| rsx!(
                    div { class: "bg-red-500 bg-opacity-20 border border-red-500 text-white p-3 rounded mb-4",
                        "{err}"
                    }
                ))}
                form { 
                    class: "space-y-4",
                    method: "post",
                    action: "/admin/login",
                    div { class: "space-y-2",
                        label { class: "block text-white", "Username" }
                        input { 
                            class: "w-full px-4 py-2 rounded bg-white bg-opacity-20 text-white border border-white border-opacity-30 focus:outline-none focus:border-opacity-60",
                            "type": "text",
                            name: "username",
                            placeholder: "Enter username"
                        }
                    }
                    div { class: "space-y-2",
                        label { class: "block text-white", "Password" }
                        input { 
                            class: "w-full px-4 py-2 rounded bg-white bg-opacity-20 text-white border border-white border-opacity-30 focus:outline-none focus:border-opacity-60",
                            "type": "password",
                            name: "password",
                            placeholder: "Enter password"
                        }
                    }
                    button { 
                        class: "w-full py-2 px-4 bg-white text-indigo-600 rounded hover:bg-opacity-90 transition-colors font-semibold",
                        "type": "submit",
                        "Login"
                    }
                }
            }
        }
    })
}