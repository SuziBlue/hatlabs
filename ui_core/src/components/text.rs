use core::marker::PhantomData;

use crate::{colors::ColorToken, geometry::{Arithmetic, Region}};

use super::Component;



pub trait Text: AsRef<str>
    + Clone
    + Default
{}

impl<T> Text for T
where 
    T: AsRef<str>
    + Clone
    + Default
{}

#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord)]
pub struct TextStyle {
    pub font: Option<Font>,
    pub color: Option<ColorToken>,
    pub size: Option<u32>, // in points or px
    pub bold: Option<bool>,
    pub italic: Option<bool>,
    pub justification: Option<TextJustification>,
    pub vertical_alignment: Option<TextAlignment>,
    // etc.
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord)]
pub enum TextJustification {
    Left,
    Center,
    Right,
    Full,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord)]
pub enum Font {
    SystemDefault,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord)]
pub enum TextAlignment {
    Top,
    Center,
    Bottom,
}

impl TextStyle {
    pub fn merge(self, other: TextStyle) -> TextStyle {
        TextStyle {
            font: other.font.or(self.font),
            color: other.color.or(self.color),
            size: other.size.or(self.size),
            bold: other.bold.or(self.bold),
            italic: other.italic.or(self.italic),
            justification: other.justification.or(self.justification),
            vertical_alignment: other.vertical_alignment.or(self.vertical_alignment),
        }
    }
}

impl Default for TextStyle {
    fn default() -> Self {
        TextStyle { font: None, color: None, size: None, bold: None, italic: None, justification: None, vertical_alignment: None }
    }
}

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub struct TextBox<'a, T: Text, R: Region<A>, A: Arithmetic> {
    pub text: T,
    pub region: &'a R,
    pub style: TextStyle,
    _phantom: PhantomData<A>,
}

impl<'a, T: Text, R: Region<A>, A: Arithmetic> TextBox<'a, T, R, A> {
    pub fn new(text: T, region: &'a R, style: Option<TextStyle>) -> Self {
        Self { 
            text, 
            style: style.unwrap_or(TextStyle::default()),
            region,
            _phantom: PhantomData,
        }
    }
}

impl<T: Text, R: Region<A>, A: Arithmetic> Component for TextBox<'_, T, R, A> {
    type State = T;

    fn state(&self) -> Self::State {
        self.text.clone()
    }
    fn update(&mut self, new_state: Self::State) {
        self.text = new_state;
    }
}
