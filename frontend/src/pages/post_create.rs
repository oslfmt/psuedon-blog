use yew::{function_component, html, Html};
use yew_router::prelude::*;

use crate::routes::Route;

// TODO: USE STATE OR NAVIGATOR MIDDLEWARE TO MAKE SURE THIS PAGE IS NOT ACCESSIBLE
// BY DIRECTLY PUTTING A URL IN

#[function_component(PostCreateForm)]
pub fn post_create_form() -> Html {
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
}
