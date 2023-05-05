use tiny_skia::{ColorU8};

#[derive(Debug)]
pub enum Shape {
    Rect { x: u32, y: u32, width: u32, height: u32, color: ColorU8 },
    Circle { c: u32, r: u32, color: ColorU8 },
}
