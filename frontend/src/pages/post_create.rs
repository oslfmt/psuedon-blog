use yew::{function_component, html, use_effect_with, use_state, Html};
use yew_router::prelude::*;
use gloo_net::http::Request;

use crate::routes::Route;

// TODO: USE STATE OR NAVIGATOR MIDDLEWARE TO MAKE SURE THIS PAGE IS NOT ACCESSIBLE
// BY DIRECTLY PUTTING A URL IN

#[function_component(PostCreateForm)]
pub fn post_create_form() -> Html {
    let navigator = use_navigator().unwrap();
    let authenticated = use_state(|| false);

    {
        let authenticated = authenticated.clone();
        use_effect_with((), move |_| {
            wasm_bindgen_futures::spawn_local(async move {
                let res = Request::get("http://localhost:8080/authenticate")
                    .send()
                    .await;

                match res {
                    Ok(response) => {
                        if response.ok() {
                            authenticated.set(true);
                        } else {
                            authenticated.set(false);
                            // navigator.push(&Route::Home);
                        }
                    },
                    Err(_e) => {
                        // TODO: log some other error
                        authenticated.set(false);
                        navigator.push(&Route::Home);
                    }
                }
            })
        })
    }

    if *authenticated {
        html! {
            <div class="container main-container">
                <h1 class="post-title">{"Create new post"}</h1>
                <p class="home-link">
                    <Link<Route> to={Route::Home}>{"Home"}</Link<Route>>
                </p>
                <div class="thin-bar"></div>
                <form target="_blank" action="http://localhost:8080/thisishowidoit" method="post">
                    <div class="form-div">
                        <label for="title" class="form-label">{"Title"}</label>
                        <input name="title" required=true/>
                    </div>
                    <div class="form-div">
                        <label for="date" class="form-label">{"Date"}</label>
                        <input name="date" type="date"  required=true/>
                    </div>
                    <div class="form-div">
                        <label for="content" class="form-label">{"Content"}</label>
                        <textarea name="content" rows="15" cols="80" id="content" required=true/>
                    </div>
                    <div class="form-div">
                        <label for="tag" class="form-label">{"Tag"}</label>
                        // TODO: make this nicer, because it is not nice
                        // TODO: this has a dependency on the enum type for tags on frontend types.rs
                        <select name="tag" id="tag" multiple=false>
                            <option value="Blockchain">{"blockchain"}</option>
                            <option value="Random">{"random"}</option>
                            <option value="Philosophy">{"philosophy"}</option>
                        </select>
                    </div>
                    <div class="form-div">
                        <button type="submit" class="submit-btn">{"Submit"}</button>
                    </div>
                </form>
            </div>
        }
    } else {
        html! {
            <div class="container main-container">
                <h1 class="post-title">{"Secret area"}</h1>
            </div>
        }
    }

}
