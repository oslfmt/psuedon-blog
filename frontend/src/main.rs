use gloo_net::http::Request;
use serde::Deserialize;
use yew::prelude::*;
use chrono;

#[derive(Clone, PartialEq, Deserialize)]
enum Tag {
    Blockchain,
    Philosophy,
}

#[derive(Clone, PartialEq, Deserialize)]
struct Post {
    id: usize,
    title: String,
    date: chrono::NaiveDate,
    tag: Tag,
}

#[derive(Properties, PartialEq)]
struct PostsListProps {
    posts: Vec<Post>,
}

#[function_component(PostsList)]
fn posts_list(PostsListProps { posts}: &PostsListProps) -> Html {
    posts.iter().map(|post| html! {
        <>
            <p>{format!("{}", post.date)}</p>
            <h3 key={post.id}>{format!("{}", post.title)}</h3>
        </>
    }).collect()
}

#[function_component(App)]
fn app() -> Html {
    let posts = use_state(|| vec![]);

    {
        let posts = posts.clone();
        use_effect_with((), move |_| {
            let posts = posts.clone();
            wasm_bindgen_futures::spawn_local(async move {
                let fetched_posts: Vec<Post> = Request::get("/")
                    .send()
                    .await
                    .unwrap()
                    .json()
                    .await
                    .unwrap();
                posts.set(fetched_posts);
            });
            || ()
        });
    }

    html! {
        <>
            <h1>{"Psuedon's Website"}</h1>
            <PostsList posts={(*posts).clone()} />
        </>
    }
}

fn main() {
    yew::Renderer::<App>::new().render();
}
