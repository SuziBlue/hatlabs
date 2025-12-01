pub mod components;
pub mod colors;

use std::rc::Rc;

use web_sys::{window, Document, Element, HtmlElement};
use web_sys::wasm_bindgen::JsCast;
use yew::prelude::*;
use components::*;
use gloo_timers::callback::Timeout;
use crate::colors::*;

#[derive(PartialEq, Clone)]
enum Direction {
    Forward,
    Backward,
}

#[derive(Clone, PartialEq, Properties)]
pub struct PresentationProps {
    pub slides: Rc<Vec<Html>>,
}

const STYLE: &str = include_str!("../styles/style.css");

fn inject_css(css: &str) {
    let document = window().unwrap().document().unwrap();
    let head = document.head().unwrap();

    let style = document
        .create_element("style")
        .unwrap()
        .dyn_into::<HtmlElement>()
        .unwrap();

    style.set_inner_html(css);
    head.append_child(&style).unwrap();
}

#[function_component(Presentation)]
pub fn presentation(props: &PresentationProps) -> Html {

    let slides = props.slides.clone();

    use_effect(|| {
        inject_css(STYLE);
        || ()
    });

    let current_slide = use_state(|| 0);
    let previous_slide = use_state(|| None::<usize>);
    let direction = use_state(|| Direction::Forward); // slide direction


    // NEXT SLIDE
    let next = {
        let current_slide = current_slide.clone();
        let previous_slide = previous_slide.clone();
        let direction = direction.clone();
        let slides_len = slides.len();

        Callback::from(move |_| {
            let current = *current_slide;
            let next = (current + 1) % slides_len;
            previous_slide.set(Some(current));
            direction.set(Direction::Forward);
            current_slide.set(next);

            // Clear previous after animation
            let previous_slide = previous_slide.clone();
            Timeout::new(600, move || previous_slide.set(None)).forget();
        })
    };

    // PREVIOUS SLIDE
    let prev = {
        let current_slide = current_slide.clone();
        let previous_slide = previous_slide.clone();
        let direction = direction.clone();
        let slides_len = slides.len();

        Callback::from(move |_| {
            let current = *current_slide;
            let prev = (current + slides_len - 1) % slides_len;
            previous_slide.set(Some(current));
            direction.set(Direction::Backward);
            current_slide.set(prev);

            // Clear previous after animation
            let previous_slide = previous_slide.clone();
            Timeout::new(600, move || previous_slide.set(None)).forget();
        })
    };

    let previous_slide_html = if let Some(prev_index) = *previous_slide {
        let class = match *direction {
            Direction::Forward => "slide exit-left",
            Direction::Backward => "slide exit-right",
        };
        Some(html! {
            <div key={prev_index} class={class}>
                { slides[prev_index].clone() }
            </div>
        })
    } else {
        None
    };

    let current_class = match *direction {
        Direction::Forward => "slide enter-right",
        Direction::Backward => "slide enter-left",
    };

    html! {    // Invisible hover zone at bottom of viewport
        <>
            <div class="hover-zone"></div>

            <div class="slide-wrapper">
                { for previous_slide_html }

                <div key={*current_slide} class={current_class}>
                    { slides[*current_slide].clone() }
                </div>

                // LEFT SIDE NAV
                <div class="nav-container left">
                    <div class="edge-hover" onclick={prev}>
                        <button id="nav-left" class="nav-button">{ "⟵" }</button>
                    </div>
                </div>

                // RIGHT SIDE NAV
                <div class="nav-container right">
                    <div class="edge-hover" onclick={next}>
                        <button id="nav-right" class="nav-button">{ "⟶" }</button>
                    </div>
                </div>
            </div>
        </>
    }
}

