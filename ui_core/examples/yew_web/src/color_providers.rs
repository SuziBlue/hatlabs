
use ui_core::colors::{ColorProvider,ColorToken};

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct RgbColor(pub u8, pub u8, pub u8);

pub struct YewColorProvider;

impl ColorProvider for YewColorProvider {
    type ColorType = RgbColor;

    fn provide_color(&self, token: ColorToken) -> Self::ColorType {
        match token {
            ColorToken::Primary => RgbColor(0, 120, 255),
            ColorToken::Secondary => RgbColor(100, 100, 100),
            ColorToken::Background => RgbColor(255, 255, 255),
            ColorToken::Foreground => RgbColor(0, 0, 0),
            ColorToken::Text => RgbColor(50, 50, 50),
            ColorToken::Highlight => RgbColor(255, 200, 0),
            ColorToken::Error => RgbColor(255, 0, 0),
        }
    }
}


