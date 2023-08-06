use tiny_skia::{ColorU8, Pixmap};
pub use tiny_skia::{Mask, Path};

use crate::shape::{PaintStyle, DrawContext};

#[derive(Debug)]
pub enum Providers {
    G(Group),
}

pub trait Provider {
    fn default() -> Self;
    fn set_context(&mut self, pixmap: &mut Pixmap, parent: Option<&Providers>) -> ();
}

#[derive(Debug)]
pub struct GroupClip {
    pub path: Path,
    pub invert: Option<bool>,
}

impl GroupClip {
    pub fn default(path: Path) -> Self {
        GroupClip { path, invert: None }
    }
}

#[derive(Debug)]
pub struct Group {
    pub x: Option<u32>,
    pub y: Option<u32>,
    pub color: Option<ColorU8>,
    pub style: Option<PaintStyle>,
    pub stroke_width: Option<u32>,
    pub clip: Option<GroupClip>,
    pub context: Option<DrawContext>,
}

impl Provider for Group {
    fn default() -> Self {
        Group {
            x: None,
            y: None,
            color: None,
            style: None,
            stroke_width: None,
            clip: None,
            context: None,
        }
    }

    fn set_context(&mut self, pixmap: &mut Pixmap, parent: Option<&Providers>) {
        let mask: Option<Mask> = Mask::new(pixmap.width(), pixmap.height());

        let mut context = DrawContext {
            offset_x: self.x.unwrap_or(0),
            offset_y: self.y.unwrap_or(0),
            color: self.color,
            style: self.style,
            stroke_width: self.stroke_width,
            mask: None,
        };

        match parent {
            Some(Providers::G(group)) => {
                if let Some(parent_context) = group.context.as_ref() {
                    context.offset_x = parent_context.offset_x + context.offset_x;
                    context.offset_y = parent_context.offset_y + context.offset_y;
                    context.color = context.color.or(parent_context.color);
                    context.style = context.style.or(parent_context.style);
                    context.stroke_width = context.stroke_width.or(parent_context.stroke_width);
                }
            }
            _ => {}
        };

        self.context = Some(context);
    }
}
