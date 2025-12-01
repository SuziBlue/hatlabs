use ui_core::borders::BorderComponent;
use ui_core::borders::BorderType;
use ui_core::colors::ColorToken;
use ui_core::components::Component;
use ui_core::components::Cons;
use ui_core::components::Nil;
use ui_core::components::Input;
use ui_core::layouts::Layout;
use ui_core::layouts::LayoutSize;
use ui_core::layouts::UiLayouts;
use ui_core::text::Font;
use ui_core::text::TextAlignment;
use ui_core::text::TextStyle;
use yew_web::color_providers::YewColorProvider;
use yew_web::renderers::YewRenderer;
use yew_web::renderers::YewRenderContext;
use ui_core::components::{Button, Label};
use ui_core::CanRender;

use yew_web::runtime::run_app; // Or wherever your `run_app` is defined

fn main() {
    let text_style = TextStyle {
        font: Font::SystemDefault,
        size: 32,
        color: ColorToken::Text,
        bold: false,
        italic: true,
        alignment: TextAlignment::Top,
    };

    let label = Label {
        text: "Hello from core!".into(),
        style: text_style.clone(),
    };

    let button = Button {
        label: Label {
            text: "Click me".into(),
            style: text_style.clone(),
        },
    };

    let components = Cons {
        head: label,
        tail: Cons {
            head: button,
            tail: Nil,
        }
    };

    let layout = Layout {
        component: components,
        layout_type: UiLayouts::Vertical,
        size: LayoutSize::Fill,
    };

    run_app(layout);
}
//use yew::prelude::*;
//use gloo::console::log;
//
//
//#[function_component(App)]
//fn app() -> Html {
//    let text_style = TextStyle {
//        font: Font::SystemDefault,
//        size: 32,
//        color: ColorToken::Text,
//        bold: false,
//        italic: true,
//        alignment: TextAlignment::Top,
//    };
//
//    // Input component state (managed in App)
//    let input_state = use_state(|| Input {
//        value: Some("initial".to_string()),
//        style: text_style.clone(),
//    });
//
//    let on_change = {
//        let input_state = input_state.clone();
//        Callback::from(move |new_val: Option<String>| {
//            let mut updated = (*input_state).clone();
//            updated.update(new_val);
//            log!("Input updated: {}", updated.state());
//            input_state.set(updated);
//        })
//    };
//
//    // Your static components
//    let label = Label {
//        text: "Hello from core!".into(),
//        style: text_style.clone(),
//    };
//
//    let button = Button {
//        label: Label {
//            text: "Click me".into(),
//            style: text_style.clone(),
//        },
//    };
//
//    let components = Cons {
//        head: label,
//        tail: button,
//    };
//
//    let layout: Layout<_, &str> = Layout {
//        component: components,
//        layout_type: UiLayouts::Vertical,
//        size: LayoutSize::Fill,
//    };
//
//    // Renderer only handles pure display
//    let renderer = YewRenderer;
//    let ctx = YewRenderContext {
//        color_provider: YewColorProvider {},
//    };
//
//    let rendered_ui = renderer.render(
//        &layout,
//        &ctx,
//    ).unwrap();
//
//    html! {
//        <>
//            { rendered_ui }
//
//        </>
//    }
//}
//
//fn main() {
//    yew::Renderer::<App>::new().render();
//}
