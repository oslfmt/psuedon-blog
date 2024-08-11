use yew_router::Routable;

#[derive(Clone, Routable, PartialEq)]
pub enum Route {
    #[at("/")]
    Home,
    #[at("/posts/:id")]
    Post { id: String },
    #[at("/thisishowidoit")]
    PostCreateForm,
    #[at("/login")]
    Login,
    #[not_found]
    #[at("/404")]
    NotFound,
}
