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
    #[at("/post/:id")]
    Post { id: usize },
    #[not_found]
    #[at("/404")]
    NotFound,
}

#[derive(Clone, PartialEq, Deserialize, Debug, Default)]
enum Tag {
    blockchain,
    philosophy,
    #[default]
    general,
}

#[derive(Clone, PartialEq, Deserialize, Debug, Default)]
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
            // NOTE: this link here is what sets the URL path to /post/:id when it is clicked
            <Link<Route> to={Route::Post { id: post.id }}>{format!("{}", post.title)}</Link<Route>>
            // <h3 key={post.id}>{format!("{}", post.title)}</h3>
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

#[derive(Default, Deserialize)]
struct PostContent {
    content: String,
}

#[derive(Properties, PartialEq)]
struct PostProps {
    id: usize
}

#[function_component(PostPage)]
fn post(props: &PostProps) -> Html {
    let post_content = use_state(|| String::new());
    {
        let post_content = post_content.clone();
        let id = props.id;
        use_effect_with((), move |_| {
            // TODO: figure out why we need to clone twice
            let post_content = post_content.clone();
            wasm_bindgen_futures::spawn_local(async move {
                let fetched_post: String = Request::get(&format!("http://127.0.0.1:8080/post/{}", id))
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
        <>
            <h1>{"POST TITLE GOES HERE"}</h1>
            <p>{format!("{}", *post_content)}</p>
        </>
    }
}

fn switch(routes: Route) -> Html {
    match routes {
        Route::Home => html! {
            <Home />
        },
        Route::Post { id } => html! {
            <PostPage id={id} />
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
