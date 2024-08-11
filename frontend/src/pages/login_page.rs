use web_sys::HtmlInputElement;
use yew::{function_component, html, use_state, Callback, Html, InputEvent, SubmitEvent, TargetCast};
use gloo_net::http::Request;
use yew_router::hooks::use_navigator;

use crate::routes::Route;

#[function_component(LoginPage)]
pub fn login() -> Html {
    let navigator = use_navigator().unwrap();
    let username = use_state(|| String::new());
    let password = use_state(|| String::new());
    let login_invalid = use_state(|| false);

    let onsubmit = {
        let username = username.clone();
        let password = password.clone();
        let login_invalid = login_invalid.clone();
        
        Callback::from(move |event: SubmitEvent| {
            event.prevent_default();

            let navigator = navigator.clone();
            let username = (*username).clone();
            let password = (*password).clone();
            let login_invalid = login_invalid.clone();

            // console::log_1(&"hi".into());
            let form_data = (username, password);

            wasm_bindgen_futures::spawn_local(async move {
                let res = Request::post("http://127.0.0.1:8080/login")
                    .json(&form_data)
                    .expect("Failed to serialize the form data")
                    .send()
                    .await;

                match res {
                    Ok(r) => {
                        if r.ok() {
                            // login successful, redirect to create_post page
                            navigator.push(&Route::PostCreateForm);
                            login_invalid.set(false);
                        } else {
                            login_invalid.set(true);
                        }
                    },
                    Err(_e) => {
                        // redirect to 404 page
                        navigator.push(&Route::NotFound)
                    }
                }
            });
        })
    };

    let oninput_username = {
        let username = username.clone();
        Callback::from(move |e: InputEvent| {
            let input: HtmlInputElement = e.target_unchecked_into();
            username.set(input.value());
        })
    };

    let oninput_password = {
        let password = password.clone();
        Callback::from(move |e: InputEvent| {
            let input: HtmlInputElement = e.target_unchecked_into();
            password.set(input.value());
        })
    };

    html! {
        <div class="container main-container">
            <h1 class="post-title">{"Login"}</h1>
            <form {onsubmit}>
                <div class="form-div">
                    <label for="username" class="form-label">{"Username"}</label>
                    <input name="username" value={(*username).clone()} oninput={oninput_username} required=true/>
                </div>
                <div class="form-div">
                    <label for="password" class="form-label">{"Password"}</label>
                    <input type="password" name="password" value={(*password).clone()} oninput={oninput_password} required=true/>
                </div>
                <div class="form-div">
                    <button type="submit" class="login-submit">{"Submit"}</button>
                </div>
                <div class="form-div">
                    <p>{ if (*login_invalid).clone() { "Invalid credentials!"} else {""}}</p>
                </div>
            </form>
        </div>
    }
}
