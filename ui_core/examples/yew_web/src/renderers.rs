use yew::prelude::*;
use ui_core::{borders::{BorderComponent, BorderType}, colors::{ColorProvider, ColorToken}, components::{Button, Component, Cons, Input, Label, Nil, Root}, layouts::{Layout, LayoutSize, UiLayouts}, text::{TextAlignment, TextStyle}, CanRender, RenderContext, RenderError};

use crate::color_providers::YewColorProvider;

pub type YewRenderContext = RenderContext<YewColorProvider>;

pub struct YewRenderer;

impl YewRenderer {
    pub fn new() -> Self {
        Self{}
    }
}


pub trait ToCss {
    fn to_css(&self, ctx: &YewRenderContext) -> String;
}

impl ToCss for TextStyle {
    fn to_css(&self, ctx: &YewRenderContext) -> String {
        let color = ctx.color_provider.provide_color(self.color);
        format!(
            "font-size: {}px; color: rgb({}, {}, {}); font-weight: {}; font-style: {}; {}",
            self.size,
            color.0,
            color.1,
            color.2,
            if self.bold { "bold" } else { "normal" },
            if self.italic { "italic" } else { "normal" },
            self.alignment.to_css(ctx),
        )
    }
}

impl ToCss for TextAlignment {
    fn to_css(&self, _ctx: &YewRenderContext) -> String {
        let (text_align, vertical_align) = match self {
            TextAlignment::TopLeft => ("left", "top"),
            TextAlignment::Top => ("center", "top"),
            TextAlignment::TopRight => ("right", "top"),
            TextAlignment::Left => ("left", "middle"),
            TextAlignment::Center => ("center", "middle"),
            TextAlignment::Right => ("right", "middle"),
            TextAlignment::BottomLeft => ("left", "bottom"),
            TextAlignment::Bottom => ("center", "bottom"),
            TextAlignment::BottomRight => ("right", "bottom"),
        };

        format!("text-align: {}; vertical-align: {};", text_align, vertical_align)
    }
}

//impl<C> CanRender<Root<C>, YewRenderContext> for YewRenderer
//where 
//    C: Component,
//    YewRenderer: CanRender<C, YewRenderContext>
//{
//    type Target = Html;
//    fn render(
//            &self, 
//            component: &Root<C>,
//            ctx: &YewRenderContext
//        ) -> Result<Self::Target, RenderError> {
//
//        let html = <Self as CanRender<C, YewRenderContext>>::render(self, &component.component, ctx)?;
//
//        Ok(html! {
//            html
//        })
//    }
//}

impl CanRender<Label<String>, YewRenderContext> for YewRenderer {
    type Target = Html;
    fn render(&self, component: &Label<String>, ctx: &YewRenderContext) -> Result<Html, RenderError> {
        let style_string = component.style.to_css(ctx);

        Ok(html! {
            <span style={style_string}>
                { component.text.clone() }
            </span>
        })
    }
}

impl CanRender<Button<String>, YewRenderContext> for YewRenderer {
    type Target = Html;
    fn render(&self, component: &Button<String>, ctx: &YewRenderContext) -> Result<Html, RenderError> {
        let label_html = self.render(&component.label, ctx)?;
        Ok(html! {
            <button>{ label_html }</button>
        })
    }
}


impl CanRender<Nil, YewRenderContext> for YewRenderer {
    type Target = Html;

    fn render(&self, _component: &Nil, _ctx: &YewRenderContext) -> Result<Html, RenderError> {
        Ok(html! {}) // Render nothing
    }
}

impl<H, T> CanRender<Cons<H, T>, YewRenderContext> for YewRenderer
where
    H: Component,
    T: Component,
    YewRenderer: CanRender<H, YewRenderContext, Target = Html>,
    YewRenderer: CanRender<T, YewRenderContext, Target = Html>,
{
    type Target = Html;

    fn render(&self, component: &Cons<H, T>, ctx: &YewRenderContext) -> Result<Html, RenderError> {
        let head_html = <Self as CanRender<H, YewRenderContext>>::render(self, &component.head, ctx)?;
        let tail_html = <Self as CanRender<T, YewRenderContext>>::render(self, &component.tail, ctx)?;

        Ok(html! {
            <>
                { head_html }
                { tail_html }
            </>
        })
    }
}

impl<C, Fixed> CanRender<Layout<C, Fixed>, YewRenderContext> for YewRenderer
where
    C: Component,
    YewRenderer: CanRender<C, YewRenderContext, Target = Html>,
    Fixed: ToString + Ord + Copy + Eq,
{
    type Target = Html;

    fn render(&self, component: &Layout<C, Fixed>, ctx: &YewRenderContext) -> Result<Html, RenderError> {
        let child_html = self.render(&component.component, ctx)?;

        let layout_class = match component.layout_type {
            UiLayouts::Vertical => "layout-vertical",
            UiLayouts::Horizontal => "layout-horizontal",
            UiLayouts::Grid => "layout-grid",
        };

        let size_style = match &component.size {
            LayoutSize::Fill => "flex: 1;".to_string(),
            LayoutSize::Fixed(size) => format!("flex: 0 0 auto; width: {};", size.to_string()),
        };

        Ok(html! {
            <div class={layout_class} style={size_style}>
                { child_html }
            </div>
        })
    }
}

impl<C> CanRender<BorderComponent<C>, YewRenderContext> for YewRenderer
where
    C: Component,
    YewRenderer: CanRender<C, YewRenderContext, Target = Html>,
{
    type Target = Html;

    fn render(&self, border_component: &BorderComponent<C>, ctx: &YewRenderContext) -> Result<Html, RenderError> {
        let inner = self.render(&border_component.component, ctx)?;

        let class = match border_component.border {
            BorderType::Thin => "border-thin",
            BorderType::Thick => "border-thick",
            BorderType::Double => "border-double",
            BorderType::DoubleThick => "border-double-thick",
        };

        Ok(html! {
            <div class={classes!(class)}>
                { inner }
            </div>
        })
    }
}

