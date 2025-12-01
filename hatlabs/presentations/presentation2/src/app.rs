use std::rc::Rc;
use yew::prelude::*;
use presentations::Presentation;

#[function_component(App)]
pub fn app() -> Html {
    let slides = Rc::new(vec![
        html! {
            <div class="slide-content">
                <h1>{ "PC Building Stream Part 2" }</h1>
                <p>{ "With Suziblue" }</p>
            </div>
        },
        html! {
            <div class="slide-content">
                <h2>{ "This Week's Updates" }</h2>
                <ul>
                    <li>{ "Slideshow presentations!" }</li>
                    <li>{ "New art commission" }</li>
                    <li>{ "Tweaks to Suzi's listening abilities" }</li>
                </ul>
            </div>
        },
        html! {
            <div class="slide-content">
                <h2>{ "Presentations" }</h2>
                <p>{ "I will present my progress on Suziblue and other projects at the start of each stream. Also introduce the topic of the current stream." }</p>
            </div>
        },
        html! {
            <div class="slide-content">
                <h2>{ "New art commission" }</h2>
                <img src="assets/3177566-1.png" style="max-height: 75vh; width: auto;"/>
                <p>{ "Art by: @sYaM_illust" }</p>
            </div>
        },
        html! {
            <div class="slide-content">
                <h2>{ "Improved listening abilities" }</h2>
                <ul>
                    <li>{ "Hopefully no more disconnecting" }</li>
                    <li>{ "Newer transcription model" }</li>
                    <li>{ "Better turn detection" }</li>
                </ul>
            </div>
        },
        html! {
            <div class="slide-content">
                <h2>{ "Today's Agenda" }</h2>
                <ul>
                    <li>{ "Mount new CPU" }</li>
                    <li>{ "Mount the GPU" }</li>
                    <li>{ "Finish cabling" }</li>
                    <li>{ "Install OS" }</li>
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
