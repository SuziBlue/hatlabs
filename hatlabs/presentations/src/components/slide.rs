use yew::prelude::*;

/// Props for the Slide component
#[derive(Properties, PartialEq)]
pub struct SlideProps {
    pub children: Children,
}

#[function_component(Slide)]
pub fn slide(props: &SlideProps) -> Html {
    html! {
        <div
            class="slide"
        >
            { for props.children.iter() }
        </div>
    }
}
