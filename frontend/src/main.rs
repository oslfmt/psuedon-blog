use yew::prelude::*;
use yew_router::prelude::*;

use routes::Route;
use pages::home::Home;
use pages::post_page::PostPage;

// TODO: do we need to declare this pub? What difference does it make?
mod pages;
mod types;
mod routes;

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
