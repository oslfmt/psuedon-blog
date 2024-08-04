use yew_router::Routable;

#[derive(Clone, Routable, PartialEq)]
pub enum Route {
    #[at("/")]
    Home,
    #[at("/posts/:id/:title")]
    Post { id: String, title: String },
    #[at("/thisishowidoit")]
    PostCreateForm,
    #[not_found]
    #[at("/404")]
    NotFound,
}
