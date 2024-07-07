use gloo_net::http::Request;
use serde::Deserialize;
use yew::prelude::*;

#[derive(Clone, PartialEq, Deserialize)]
enum Category {
    Blockchain,
    Philosophy,
}

#[derive(Clone, PartialEq, Deserialize)]
struct Article {
    id: usize,
    title: String,
    date: String,
    category: Category,
}

#[derive(Properties, PartialEq)]
struct ArticlesListProps {
    articles: Vec<Article>,
    on_click: Callback<Article>,
}

#[function_component(ArticlesList)]
fn articles_list(ArticlesListProps { articles, on_click}: &ArticlesListProps) -> Html {
    articles.iter().map(|article| html! {
        <>
            <p>{article.date.clone()}</p>
            <h3 key={article.id}>{format!("{}", article.title)}</h3>
        </>
    }).collect()
}

#[function_component(App)]
fn app() -> Html {
    let articles = vec![
        Article {
            id: 1,
            title: "Floating".to_string(),
            date: "2024 4 July".to_string(),
            category: Category::Philosophy,
        },
        Article {
            id: 2,
            title: "Some thoughts on blockchain".to_string(),
            date: "2024 10 July".to_string(),
            category: Category::Blockchain,
        },
    ];

    let articles = use_state(|| vec![]);
    {
        let articles = articles.clone();
        use_effect_with((), move |_| {
            let articles = articles.clone();
            wasm_bindgen_futures::spawn_local(async move {
                let fetched_articles: Vec<Article> = Request::get("http://127.0.0.1:8080/posts")
                    .send()
                    .await
                    .unwrap()
                    .json()
                    .await
                    .unwrap();
                articles.set(fetched_articles);
            });
            || ()
        });
    }

    html! {
        <>
            <h1>{"Psuedon's Website"}</h1>
            <ArticlesList articles={articles} />
        </>
    }
}

fn main() {
    yew::Renderer::<App>::new().render();
}
