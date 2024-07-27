use gloo_net::http::Request;
use yew::prelude::*;
use yew_router::prelude::*;

use routes::Route;
use pages::home::Home;

mod pages;
mod types; // TODO: do we need to declare this pub? What difference does it make?
mod routes;

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
