use dioxus::prelude::*;

pub fn login_page(cx: Scope) -> Element {
    let on_submit = move |evt: Event<FormData>| {
        // Handle form submission
        log::info!("Form submitted");
    };

    cx.render(rsx! {
        div { class: "admin-login",
            h1 { "Admin Login" }
            form { 
                class: "login-form",
                onsubmit: on_submit,
                div { class: "form-group",
                    label { "Username:" }
                    input { 
                        r#type: "text",
                        name: "username",
                        required: true 
                    }
                }
                div { class: "form-group",
                    label { "Password:" }
                    input { 
                        r#type: "password",
                        name: "password",
                        required: true 
                    }
                }
                button { 
                    r#type: "submit",
                    "Login" 
                }
            }
        }
    })
}