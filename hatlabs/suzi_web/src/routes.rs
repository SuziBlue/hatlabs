
use yew_router::prelude::*;
use yew::prelude::*;

use crate::pages::*;

#[derive(Routable, PartialEq, Eq, Clone, Debug)]
pub enum Route {
    #[at("/")]
    Home,
    #[at("/lore")]
    Lore,
    #[at("/schedule")]
    Schedule,
    #[at("/gallery")]
    Gallery,
    #[at("/blog")]
    Blog,
    #[at("/faq")]
    FAQ,
    #[at("/socials")]
    Socials,
    #[at("/post/:id")]
    Post { id: usize }, // match id or slug
    #[not_found]
    #[at("/404")]
    NotFound,
}
// ----- Route Switch -----

pub fn switch(route: Route) -> Html {
    let page = match route {
        Route::Home => html! { <Home /> },
        Route::Lore => html! { <Lore /> },
        Route::Schedule => html! { <Schedule /> },
        Route::Gallery => html! { <Gallery /> },
        Route::Blog => html! { <Blog /> },
        Route::FAQ => html! { <FAQ /> },
        Route::Socials => html! { <Socials /> },        
        Route::Post { id } => html! { <BlogPostDetail id={id} /> },
        Route::NotFound => html! { <NotFound /> },
    };

    html! {
        <Layout>
            {page}
        </Layout>
    }
}
