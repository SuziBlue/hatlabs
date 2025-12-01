
use ratatui::layout::Rect as RRect;
use ui_core::inputs::Vec2;
use ui_core::layouts::{Region, Rectangle};

#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub struct RatatuiRect(pub RRect);


impl Region<u16> for RatatuiRect {
    fn is_inside(&self, position: Vec2<u16>) -> bool {
        let rect = self.0;
        position.x >= rect.x &&
        position.x < rect.x + rect.width &&
        position.y >= rect.y &&
        position.y < rect.y + rect.height
    }
}

impl Rectangle<u16> for RatatuiRect {
    fn new(top_left: Vec2<u16>, bottom_right: Vec2<u16>) -> Self {
        let x = top_left.x;
        let y = top_left.y;
        let width = bottom_right.x.saturating_sub(x);
        let height = bottom_right.y.saturating_sub(y);
        RatatuiRect(RRect { x, y, width, height })
    }

    fn top_left(&self) -> Vec2<u16> {
        Vec2::new(self.0.x, self.0.y)
    }

    fn bottom_right(&self) -> Vec2<u16> {
        Vec2::new(self.0.x + self.0.width, self.0.y + self.0.height)
    }

    fn width(&self) -> u16 {
        self.0.width
    }

    fn height(&self) -> u16 {
        self.0.height
    }
}
