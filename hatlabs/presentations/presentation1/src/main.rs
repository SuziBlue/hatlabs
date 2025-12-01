use std::rc::Rc;
use yew::prelude::*;
use presentations::Presentation;

#[function_component(App)]
fn app() -> Html {
    let slides = Rc::new(vec![
        html! {
            <div class="slide-content">
                <h1>{ "Welcome to My Presentation" }</h1>
                <p>{ "This is the first slide." }</p>
            </div>
        },
        html! {
            <div class="slide-content">
                <h2>{ "About Me" }</h2>
                <ul>
                    <li>{ "Rustacean 🦀" }</li>
                    <li>{ "Yew Enthusiast" }</li>
                    <li>{ "Open Source Contributor" }</li>
                </ul>
            </div>
        },
        html! {
            <div class="slide-content">
                <h2>{ "Thank You!" }</h2>
                <p>{ "Questions?" }</p>
            </div>
        },
    ]);

    html! {
        <Presentation slides={slides} />
    }
}

fn main() {
    yew::Renderer::<App>::new().render();
}
