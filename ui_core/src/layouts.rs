use crate::geometry::{Arithmetic, Rectangle, Region, Vec2};



pub trait LayoutGenerator<T: Arithmetic, R: Region<T>, const N: usize> {
    fn generate(&self, region: impl Into<R>) -> [R; N];
}

pub struct HSplit<const L: u16, const R: u16>;

impl<T, R, const LEFT: u16, const RIGH: u16> LayoutGenerator<T, R, 2> for HSplit<LEFT, RIGH> 
where 
    T: Arithmetic + From<u16>,
    R: Rectangle<T>,
{
    fn generate(&self, rect: impl Into<R>) -> [R; 2] {
        let rect = rect.into();

        let left_width = (rect.width() * T::from(LEFT)) / T::from(LEFT + RIGH);
        let top_left = rect.top_left();
        let bottom_right = rect.bottom_right();

        let left = R::new(
            top_left,
            Vec2::new(top_left.x + left_width, bottom_right.y),
        );

        let right = R::new(
            Vec2::new(top_left.x + left_width, top_left.y),
            bottom_right,
        );

        [left, right]
    }
}

pub struct VSplit<const T: u16, const B: u16>;

impl<T, R, const TOP: u16, const BOT: u16> LayoutGenerator<T, R, 2> for VSplit<TOP, BOT>
where
    T: Arithmetic + From<u16>,
    R: Rectangle<T>,
{
    fn generate(&self, rect: impl Into<R>) -> [R; 2] {
        let rect = rect.into();
        let total = T::from(TOP + BOT);
        let top_ratio = T::from(TOP) / total;

        let top_height = rect.height() * top_ratio;
        let top_left = rect.top_left();
        let bottom_right = rect.bottom_right();

        let top = R::new(
            top_left,
            Vec2::new(bottom_right.x, top_left.y + top_height),
        );

        let bottom = R::new(
            Vec2::new(top_left.x, top_left.y + top_height),
            bottom_right,
        );

        [top, bottom]
    }
}
