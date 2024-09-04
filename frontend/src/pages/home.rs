use yew::prelude::*;
use gloo_net::http::Request;
use crate::types::Post;

use crate::components::posts_list::PostsList;

#[function_component(Home)]
pub fn home() -> Html {
    let posts = use_state(|| vec![]);

    {
        let posts = posts.clone();
        use_effect_with((), move |_| {
            let posts = posts.clone();
            wasm_bindgen_futures::spawn_local(async move {
                let fetched_posts: Vec<Post> = Request::get("http://127.0.0.1:8080/api/posts")
                    .send()
                    .await
                    .unwrap()
                    .json()
                    .await
                    .unwrap();

                // console::log_1(&format!("{:?}", fetched_posts).into());
                posts.set(fetched_posts);
            });
            || ()
        });
    }

    html! {
        <div class="container main-container">
            <h1 class="title">{"Victor's Website"}</h1>
            <div class="bar"/>
            <PostsList posts={(*posts).clone()} />
        </div>
    }
}
