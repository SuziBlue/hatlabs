#[derive(Clone, PartialEq)]
pub enum Palette {
    Default,
    Warm,
    Calm,
}

#[derive(Clone, PartialEq)]
pub enum ThemeMode {
    Light,
    Dark,
}

impl Palette {
    pub fn as_str(&self) -> &'static str {
        match self {
            Palette::Default => "default",
            Palette::Warm => "warm",
            Palette::Calm => "calm",
        }
    }
}

impl ThemeMode {
    pub fn as_str(&self) -> &'static str {
        match self {
            ThemeMode::Light => "light",
            ThemeMode::Dark => "dark",
        }
    }
}

pub fn theme_class(palette: &Palette, mode: &ThemeMode) -> String {
    format!("theme-{}-{}", palette.as_str(), mode.as_str())
}
