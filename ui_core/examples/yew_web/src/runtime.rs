use yew::prelude::*;
use gloo::console::log;

use ui_core::components::{Component, Root};
use ui_core::CanRender;
use ui_core::layouts::Layout;

use crate::color_providers::YewColorProvider;
use crate::renderers::{YewRenderer, YewRenderContext};

#[derive(Properties, PartialEq, Clone)]
pub struct RuntimeProps<C: Component> {
    root: C,
}

pub struct RuntimeApp<C: Component> {
    props: RuntimeProps<C>,
}

impl<C> yew::Component for RuntimeApp<C>
where
    C: Component + Clone + 'static,
    YewRenderer: CanRender<C, YewRenderContext, Target = Html>,
{
    type Message = ();
    type Properties = RuntimeProps<C>;

    fn create(ctx: &yew::Context<Self>) -> Self {
        Self {
            props: ctx.props().clone(),
        }
    }

    fn view(&self, _ctx: &yew::Context<Self>) -> Html {
        let renderer = YewRenderer;
        let ctx = YewRenderContext {
            color_provider: YewColorProvider {},
        };

        renderer.render(&self.props.root, &ctx).unwrap()
    }
}

pub fn run_app<C>(
    root: C
)
where 
    C: Component + 'static,
    YewRenderer: CanRender<C, YewRenderContext, Target = Html>,
{
    yew::Renderer::<RuntimeApp<C>>::with_props(RuntimeProps {
        root
    })
    .render();
}
