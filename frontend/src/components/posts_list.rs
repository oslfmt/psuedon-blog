use yew_router::prelude::Link;
use yew::prelude::*;

use crate::types::Post;
use crate::routes::Route;

#[derive(Properties, PartialEq)]
pub struct PostsListProps {
    pub posts: Vec<Post>,
}

#[function_component(PostsList)]
pub fn posts_list(PostsListProps { posts}: &PostsListProps) -> Html {
    html! {
        <ul class="posts-list">
            {
                for posts.iter().map(|post| html! {
                    <li class="post-list-item">
                        <p class="post-date">{format!("{}", post.date)}</p>
                        <p class="post-link">
                            <Link<Route> to={Route::Post { id: post.id.clone() }}>{format!("{}", post.title)}</Link<Route>>
                        </p>
                    </li>
                })
            }
        </ul>
    }
}
