use gloo_net::http::{Request, Response};
use serde::Deserialize;
use yew::prelude::*;
use web_sys::{console, js_sys::JsString};
use chrono;
use yew_router::prelude::*;

#[derive(Clone, Routable, PartialEq)]
enum Route {
    #[at("/")]
    Home,
    #[not_found]
    #[at("/404")]
    NotFound,
}

#[derive(Clone, PartialEq, Deserialize, Debug)]
enum Tag {
    blockchain,
    philosophy,
}

#[derive(Clone, PartialEq, Deserialize, Debug)]
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

#[function_component(Home)]
fn home() -> Html {
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
        <>
            <h1>{"Victor's Website"}</h1>
            <PostsList posts={(*posts).clone()} />
        </>
    }
}

fn switch(routes: Route) -> Html {
    match routes {
        Route::Home => html! {
            <Home />
        },
        Route::NotFound => html! { <h1>{ "404 Not Found" }</h1> }
    }
}

#[function_component(App)]
fn app() -> Html {
    html! {
        <BrowserRouter>
            <Switch<Route> render={switch} />
        </BrowserRouter>
    }
}

fn main() {
    yew::Renderer::<App>::new().render();
}
