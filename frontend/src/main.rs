use yew::prelude::*;
use yew_router::prelude::{Switch, BrowserRouter};

use routes::Route;
use pages::{ home::Home, post_page::PostPage, post_create::PostCreateForm, login_page::LoginPage };

mod pages;
mod components;
mod types;
mod routes;

fn switch(routes: Route) -> Html {
    match routes {
        Route::Home => html! {
            <Home />
        },
        Route::Post { id } => html! {
            <PostPage id={id} />
        },
        Route::PostCreateForm => html! {
            <PostCreateForm />
        },
        Route::Login => html! {
            <LoginPage />
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
