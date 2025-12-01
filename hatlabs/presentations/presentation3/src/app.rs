use std::rc::Rc;
use yew::prelude::*;
use presentations::Presentation;

#[function_component(App)]
pub fn app() -> Html {
    let slides = Rc::new(vec![
        html! {
            <div class="slide-content">
                <h1>{ "Let's Build a Discord Bot from Scratch!" }</h1>
                <p>{ "With Suziblue" }</p>
            </div>
        },
        html! {
            <div class="slide-content">
                <h2>{ "This Week's Updates" }</h2>
                <ul>
                    <li>{ "Slimezilla server is operational" }</li>
                </ul>
            </div>
        },
        html! {
            <div class="slide-content">
                <h2>{ "Today's Agenda" }</h2>
                <ul>
                    <li>{ "Learn how the Discord API works" }</li>
                    <li>{ "Implement protocol in Rust" }</li>
                    <li>{ "Build an API client" }</li>
                    <li>{ "Stream audio from Discord voice server to Suzi" }</li>
                </ul>
            </div>
        },
        html! {
            <div class="slide-content">
                <h1>{ "Let's get started!" }</h1>
            </div>
        },
    ]);

    html! {
        <Presentation slides={slides} />
    }
}
