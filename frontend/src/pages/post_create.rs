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
                </div>
                <div class="form-div">
                    <label for="tags" class="form-label">{"Date"}</label>
                    <input type="date"/>
                </div>
                <div class="form-div">
                    <label for="content" class="form-label">{"Content"}</label>
                    <textarea rows="15" cols="80" id="content" />
                </div>
                <div class="form-div">
                    <label for="tags" class="form-label">{"Tags"}</label>
                    // TODO: make this nicer, because it is not nice
                    <select name="cars" id="cars" multiple=true>
                        <option value="volvo">{"blockchain"}</option>
                        <option value="saab">{"random"}</option>
                        <option value="opel">{"philosophy"}</option>
                    </select>
                </div>
                <div class="form-div">
                    <button type="submit" class="submit-btn">{"Submit"}</button>
                </div>
            </form>
        </div>
    }
}
