use yew::prelude::*;

#[derive(Clone, PartialEq)]
enum Category {
    Blockchain,
    Philosophy,
}

#[derive(Clone, PartialEq)]
struct Article {
    id: usize,
    title: String,
    date: String,
    category: Category,
}

#[derive(Properties, PartialEq)]
struct ArticlesListProps {
    articles: Vec<Article>,
}

#[function_component(ArticlesList)]
fn articles_list(ArticlesListProps { articles }: &ArticlesListProps) -> Html {
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
