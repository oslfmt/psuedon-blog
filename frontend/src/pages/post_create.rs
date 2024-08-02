use yew::prelude::*;
use yew_router::prelude::*;

use crate::routes::Route;

#[function_component(PostCreateForm)]
pub fn post_create_form() -> Html {
    html! {
        <div class="container main-container">
            <h1 class="post-title">{"Create new post"}</h1>
            <p class="home-link">
                <Link<Route> to={Route::Home}>{"Home"}</Link<Route>>
            </p>
            <div class="thin-bar"></div>
            <form>
                <div class="form-div">
                    <label for="title" class="form-label">{"Title"}</label>
                    <input/>
                    // <input type="email" class="form-control" id="exampleInputEmail1" aria-describedby="emailHelp"</input>
                </div>
                <div class="form-div">
                    <label for="content" class="form-label">{"Content"}</label>
                    <textarea rows="15" cols="80" id="content" />
                </div>
                <div class="form-div">
                    <button type="submit" class="submit-btn">{"Submit"}</button>
                </div>
            </form>
        </div>
    }
}
