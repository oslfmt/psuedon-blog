use yew_router::Routable;

#[derive(Clone, Routable, PartialEq)]
pub enum Route {
    #[at("/")]
    Home,
    #[at("/posts/:id/:title")]
    Post { id: usize, title: String },
    #[not_found]
    #[at("/404")]
    NotFound,
}
