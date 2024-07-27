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
    #[at("/posts/:id/:title")]
    Post { id: usize, title: String },
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
        <div class="container main-container">
            <h1 class="title">{"Victor's Website"}</h1>
            <div class="bar"/>
            <PostsList posts={(*posts).clone()} />
        </div>
    }
}

#[derive(Default, Deserialize)]
struct PostContent {
    content: String,
}

#[derive(Properties, PartialEq)]
struct PostProps {
    id: usize,
    title: String,
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
        <>
            <h1>{&props.title}</h1>
            <p>{format!("{}", *post_content)}</p>
        </>
    }
}

fn switch(routes: Route) -> Html {
    match routes {
        Route::Home => html! {
            <Home />
        },
        Route::Post { id , title} => html! {
            <PostPage id={id} title={title} />
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
