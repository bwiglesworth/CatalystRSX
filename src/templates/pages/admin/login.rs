use dioxus::prelude::*;

pub fn login_page(cx: Scope) -> Element {
    let on_submit = move |evt: FormEvent| {
        evt.stop_propagation();
        let _form = evt.data; // Now properly marked as intentionally unused
        // Handle form submission
    };

    cx.render(rsx! {
        div { class: "admin-login",
            h1 { "Admin Login" }
            form { 
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