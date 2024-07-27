use yew::prelude::*;
use yew_router::prelude::Link;
use gloo_net::http::Request;

use crate::types::Post;
use crate::routes::Route;

#[derive(Properties, PartialEq)]
struct PostsListProps {
    posts: Vec<Post>,
}

#[function_component(PostsList)]
fn posts_list(PostsListProps { posts}: &PostsListProps) -> Html {
    html! {
        <ul class="posts-list">
            {
                for posts.iter().map(|post| html! {
                    <li class="post-list-item">
                        <p class="post-date">{format!("{}", post.date)}</p>
                        <p class="post-link">
                            <Link<Route> to={Route::Post { id: post.id, title: post.title.clone() }}>{format!("{}", post.title)}</Link<Route>>
                        </p>
                    </li>
                })
            }
        </ul>
    }
}

#[function_component(Home)]
pub fn home() -> Html {
    let posts = use_state(|| vec![]);

    {
        let posts = posts.clone();
        use_effect_with((), move |_| {
            let posts = posts.clone();
            wasm_bindgen_futures::spawn_local(async move {
                let fetched_posts: Vec<Post> = Request::get("http://127.0.0.1:8080/")
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
