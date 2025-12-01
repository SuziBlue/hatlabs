use yew::prelude::*;
use yew_router::prelude::*;

pub mod routes;
pub mod pages;

use crate::routes::{switch, Route};

pub fn frontend_domain() -> &'static str {
    option_env!("FRONTEND_DOMAIN").unwrap_or("http://localhost:3030")
}

// ----- App Root -----

#[function_component(App)]
fn app() -> Html {
    html! {
        <BrowserRouter>
            <nav class="navbar">
                <ul>
                    <li><Link<Route> to={Route::Home}>{ "Home" }</Link<Route>></li>
                    <li><Link<Route> to={Route::Lore}>{ "Lore" }</Link<Route>></li>
                    <li><Link<Route> to={Route::Schedule}>{ "Schedule" }</Link<Route>></li>
                    <li><Link<Route> to={Route::Gallery}>{ "Gallery" }</Link<Route>></li>
                    <li><Link<Route> to={Route::Blog}>{ "Blog" }</Link<Route>></li>
                    <li><Link<Route> to={Route::FAQ}>{ "FAQ" }</Link<Route>></li>
                    <li><Link<Route> to={Route::Socials}>{ "Socials" }</Link<Route>></li>
                </ul>
            </nav>

            <main>
                <Switch<Route> render={switch} />
            </main>
        </BrowserRouter>
    }
}


fn main() {
    yew::Renderer::<App>::new().render();
}
