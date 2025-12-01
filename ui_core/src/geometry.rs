use core::ops::*;

pub trait Arithmetic: 
    Copy + PartialOrd + PartialEq
    + Add<Output = Self> 
    + Sub<Output = Self> 
    + Mul<Output = Self> 
    + Div<Output = Self>
{}

impl<T> Arithmetic for T
where
    T: Copy
        + PartialOrd
        + Add<Output = T>
        + Sub<Output = T>
        + Mul<Output = T>
        + Div<Output = T>
{}

#[derive(Debug, Clone, Copy, PartialEq, PartialOrd)]
pub struct Vec2<A: Arithmetic> {
    pub x: A,
    pub y: A,
}

impl<A: Arithmetic> Vec2<A> {
    pub fn new(x: A, y: A) -> Self {
        Self { x, y }
    }
}
pub trait Region<T: Arithmetic> {
    fn is_inside(&self, position: Vec2<T>) -> bool;
}

pub trait Rectangle<T: Arithmetic>: Region<T> {
    fn new(top_left: Vec2<T>, bottom_right: Vec2<T>) -> Self;
    fn top_left(&self) -> Vec2<T>;
    fn bottom_right(&self) -> Vec2<T>;
    fn width(&self) -> T;
    fn height(&self) -> T;
}
