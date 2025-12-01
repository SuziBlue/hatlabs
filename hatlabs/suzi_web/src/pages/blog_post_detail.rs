
use yew::prelude::*;
use gloo_net::http::Request;
use serde::Deserialize;

#[derive(Clone, Debug, PartialEq, Deserialize)]
pub struct BlogPost {
    pub id: usize,
    pub title: String,
    pub date: String,
    pub content: String,
}

#[derive(Properties, PartialEq)]
pub struct Props {
    pub id: usize,
}

#[function_component(BlogPostDetail)]
pub fn blog_post_detail(props: &Props) -> Html {
    let post = use_state(|| None::<BlogPost>);
    let error = use_state(|| None::<String>);
    let id = props.id;

    {
        let post = post.clone();
        let error = error.clone();

        use_effect_with((), move |_| {
            wasm_bindgen_futures::spawn_local(async move {
                match Request::get("/blog.json").send().await {
                    Ok(resp) => match resp.json::<Vec<BlogPost>>().await {
                        Ok(posts) => {
                            if let Some(found) = posts.into_iter().find(|p| p.id == id) {
                                post.set(Some(found));
                            } else {
                                error.set(Some("Post not found.".to_string()));
                            }
                        }
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
            {
                if let Some(err) = &*error {
                    html! { <p style="color: red;">{ err }</p> }
                } else if let Some(post) = &*post {
                    html! {
                        <article class="blog-post">
                            <h1>{ &post.title }</h1>
                            <p class="date">{ &post.date }</p>
                            <p>{ &post.content }</p>
                        </article>
                    }
                } else {
                    html! { <p>{ "Loading post..." }</p> }
                }
            }
        </>
    }
}
