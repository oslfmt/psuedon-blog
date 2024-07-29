use yew::prelude::*;
use yew_router::prelude::Link;
use gloo_net::http::Request;

use crate::routes::Route;

#[derive(Properties, PartialEq)]
pub struct PostProps {
    pub id: usize,
    pub title: String,
}

#[function_component(PostPage)]
pub fn post(props: &PostProps) -> Html {
    let post_content = use_state(|| String::new());
    {
        let post_content = post_content.clone();
        let id = props.id;
        use_effect_with((), move |_| {
            // TODO: figure out why we need to clone twice
            let post_content = post_content.clone();
            wasm_bindgen_futures::spawn_local(async move {
                let fetched_post: String = Request::get(&format!("http://127.0.0.1:8080/posts/{}", id))
                    .send()
                    .await
                    .unwrap()
                    .json()
                    .await
                    .unwrap();

                post_content.set(fetched_post);
            });
            // || () what does this do??
        });
    }
    html! {
        <div class="container main-container post-container">
            <h1 class="post-title">{&props.title}</h1>
            <p class="home-link">
                <Link<Route> to={Route::Home}>{"Home"}</Link<Route>>
            </p>
            <div class="thin-bar"></div>
            <p class="post-content">{format!("{}", *post_content)}</p>
        </div>
    }
}
