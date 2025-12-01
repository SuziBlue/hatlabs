
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub enum ColorToken {
    Primary,
    Secondary,
    Foreground,
    Background,
    Text,
    Highlight,
    Error,
}

pub trait ColorProvider {
    type ColorType: Clone;

    fn provide_color(&self, token: ColorToken) -> Self::ColorType;
}
