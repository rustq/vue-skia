use std::collections::HashMap;

use tiny_skia::{ColorU8, FillRule, Pixmap, Transform};
pub use tiny_skia::{Mask, Path};

use crate::shape::{DrawContext, PaintStyle};

#[derive(Debug)]
pub enum Providers {
    G(Group),
}

pub trait Provider {
    fn default() -> Self;
    fn get_context(&self) -> Option<&DrawContext>;
    fn set_context(&mut self, pixmap: &mut Pixmap, parent: Option<&Providers>) -> ();
}

impl Providers {
    pub fn get_context(&self) -> Option<&DrawContext> {
        match self {
            Providers::G(group) => group.get_context(),
        }
    }
    pub fn set_context(&mut self, pixmap: &mut Pixmap, parent: Option<&Providers>) -> () {
        match self {
            Providers::G(group) => group.set_context(pixmap, parent),
        }
    }
}

#[derive(Debug, Default, Clone)]
pub struct GroupClip {
    pub id: Option<usize>,
    pub invert: Option<bool>,
}

impl GroupClip {
    pub fn default() -> Self {
        GroupClip {
            id: None,
            invert: None,
        }
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

    fn get_context(&self) -> Option<&DrawContext> {
        return self.context.as_ref();
    }

    fn set_context(&mut self, _pixmap: &mut Pixmap, parent: Option<&Providers>) {
        let mut inactive_nodes_map = HashMap::new();

        if let Some(clip_id) = self.clip.as_ref().and_then(|c| c.id) {
            inactive_nodes_map.insert(clip_id, true);
        }

        let mut context = DrawContext {
            offset_x: self.x.unwrap_or(0),
            offset_y: self.y.unwrap_or(0),
            color: self.color,
            style: self.style,
            stroke_width: self.stroke_width,
            mask: None,
            inactive_nodes_map: Some(inactive_nodes_map),
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

impl Group {
    pub fn set_clip_id(&mut self, id: usize) {
        if self.clip.is_none() {
            self.clip = Some(GroupClip::default());
        }

        self.clip.as_mut().unwrap().id = Some(id);
    }

    pub fn set_context_mask(&mut self, pixmap: &mut Pixmap, path: &Path) {
        let mut mask: Option<Mask> = None;

        if let Some(GroupClip { invert, .. }) = &self.clip {
            let mut clip_mask = Mask::new(pixmap.width(), pixmap.height()).unwrap();

            clip_mask.fill_path(path, FillRule::Winding, true, Transform::default());

            if let Some(true) = invert {
                clip_mask.invert();
            }

            mask = Some(clip_mask);
        }

        self.context.as_mut().unwrap().mask = mask;
    }
}
