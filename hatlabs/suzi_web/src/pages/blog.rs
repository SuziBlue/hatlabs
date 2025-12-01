
use yew::prelude::*;
use yew_router::prelude::*;
use gloo_net::http::Request;

use crate::routes::Route;


use serde::Deserialize;

#[derive(Clone, Debug, PartialEq, Deserialize)]
pub struct BlogPost {
    pub id: usize,
    pub title: String,
    pub date: String,
    pub content: String,
}

#[function_component(Blog)]
pub fn blog() -> Html {
    let posts = use_state(|| Option::<Vec<BlogPost>>::None);
    let error = use_state(|| None::<String>);

    {
        let posts = posts.clone();
        let error = error.clone();

        use_effect_with((), move |_| {
            wasm_bindgen_futures::spawn_local(async move {
                match Request::get("/blog.json").send().await {
                    Ok(resp) => match resp.json::<Vec<BlogPost>>().await {
                        Ok(data) => posts.set(Some(data)),
                        Err(err) => error.set(Some(format!("Failed to parse JSON: {err}"))),
                    },
                    Err(err) => error.set(Some(format!("Failed to fetch: {err}"))),
                }
            });
            || ()
        });
    }

    html! {
        <>
            <h1>{ "Suziblue's Blog" }</h1>
            {
                if let Some(err) = &*error {
                    html! { <p style="color: red;">{ err }</p> }
                } else if let Some(posts) = &*posts {
                    html! {
                        <>
                            { for posts.iter().map(|post| html! {
                                <article class="blog-post">
                                    <h2>{ &post.title }</h2>
                                    <p class="date">{ &post.date }</p>
                                    <p>
                                        { &post.content.chars().take(100).collect::<String>() }
                                        { "..." }
                                    </p>
                                    <Link<Route> to={Route::Post { id: post.id }} classes="read-more-btn">
                                        { "Read More" }
                                    </Link<Route>>
                                </article>
                            })}
                        </>
                    }
                } else {
                    html! { <p>{ "Loading posts..." }</p> }
                }
            }
        </>
    }
}
