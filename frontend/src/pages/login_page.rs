use yew::{function_component, html, Html};
use yew_router::prelude::*;

use crate::routes::Route;

#[function_component(LoginPage)]
pub fn login() -> Html {
    html! {
        <div class="container main-container">
            <h1 class="post-title">{"Login"}</h1>
            <form>
                <div class="form-div">
                    <label for="username" class="form-label">{"Username"}</label>
                    <input name="username" required=true/>
                </div>
                <div class="form-div">
                    <label for="password" class="form-label">{"Password"}</label>
                    <input name="password" required=true/>
                </div>
                <div class="form-div">
                    <button type="submit" class="login-submit">{"Submit"}</button>
                </div>
            </form>
        </div>
    }
}
