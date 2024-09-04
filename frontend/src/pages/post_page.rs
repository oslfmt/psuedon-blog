use yew::prelude::*;
use yew_router::prelude::Link;
use gloo_net::http::Request;

use crate::routes::Route;

#[derive(Properties, PartialEq)]
pub struct PostProps {
    pub id: String,
}

#[function_component(PostPage)]
pub fn post(props: &PostProps) -> Html {
    let post_content = use_state(|| String::new());
    let post_title = use_state(|| String::new());
    let post_date = use_state(|| String::new());
    
    {
        let post_content = post_content.clone();
        let post_title = post_title.clone();
        let post_date = post_date.clone();
        let id = props.id.clone();

        use_effect_with((), move |_| {
            let post_content = post_content.clone();
            let post_title = post_title.clone();
            let post_date = post_date.clone();
            wasm_bindgen_futures::spawn_local(async move {
                let (content, title, date): (String, String, chrono::NaiveDate) = Request::get(&format!("http://127.0.0.1:8080/api/posts/{}", id))
                    .send()
                    .await
                    .unwrap()
                    .json()
                    .await
                    .unwrap();

                post_content.set(content);
                post_title.set(title);
                post_date.set(date.to_string());
            });
            // || () what does this do??
        });
    }
    
    html! {
        <div class="container main-container post-container">
            <h1 class="post-title">{format!("{}", *post_title)}</h1>
            <p>{format!("{}", *post_date)}</p>
            <p class="home-link">
                <Link<Route> to={Route::Home}>{"Home"}</Link<Route>>
            </p>
            <div class="thin-bar"></div>
            <p class="post-content">{format!("{}", *post_content)}</p>
        </div>
    }
}
