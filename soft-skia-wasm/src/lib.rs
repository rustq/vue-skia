extern crate soft_skia;
mod utils;

use base64;
use soft_skia::provider::{Providers, Group, GroupClip};
use wasm_bindgen::prelude::*;
use soft_skia::instance::Instance;
use soft_skia::shape::{Circle, Line, Points, RoundRect, Shapes, PaintStyle, Image, Text};
use soft_skia::shape::Rect;
use soft_skia::shape::ColorU8;
use soft_skia::tree::Node;
use soft_skia::shape::Pixmap;

use cssparser::{Color as CSSColor, Parser, ParserInput};

// When the `wee_alloc` feature is enabled, use `wee_alloc` as the global
// allocator.
#[cfg(feature = "wee_alloc")]
#[global_allocator]
static ALLOC: wee_alloc::WeeAlloc = wee_alloc::WeeAlloc::INIT;

#[wasm_bindgen]
pub struct SoftSkiaWASM(Instance);

use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct WASMRectAttr {
    width: u32,
    height: u32,
    x: u32,
    y: u32,
    color: Option<String>,
    style: Option<String>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct WASMCircleAttr {
    cx: u32,
    cy: u32,
    r: u32,
    color: Option<String>,
    style: Option<String>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct WASMRoundRectAttr {
    width: u32,
    height: u32,
    r: u32,
    x: u32,
    y: u32,
    color: Option<String>,
    style: Option<String>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct WASMLineAttr {
    p1: [u32; 2],
    p2: [u32; 2],
    color: Option<String>,
    stroke_width: Option<u32>
}

#[derive(Serialize, Deserialize, Debug)]
pub struct WASMPointsAttr {
    points: Vec<[u32; 2]>,
    color: Option<String>,
    stroke_width: Option<u32>,
    style: Option<String>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct WASMGroupAttr {
    x: Option<u32>,
    y: Option<u32>,
    color: Option<String>,
    style: Option<String>,
    stroke_width: Option<u32>,
    invert_clip: Option<bool>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct WASMGroupClipAttr {
    clip: usize,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct WASMImageAttr {
    image: String,
    x: i32,
    y: i32,
    width: u32,
    height: u32,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct WASMTextAttr {
    text: String,
    x: i32,
    y: i32,
    font_size: Option<f32>,
    color: Option<String>
}


#[derive(Serialize, Deserialize, Debug)]
pub enum WASMShapesAttr {
    R(WASMRectAttr),
    C(WASMCircleAttr),
    RR(WASMRoundRectAttr),
    L(WASMLineAttr),
    P(WASMPointsAttr),
    G(WASMGroupAttr),
    GC(WASMGroupClipAttr),
    I(WASMImageAttr),
    T(WASMTextAttr),
}

#[derive(Serialize, Deserialize, Debug)]
pub struct WASMShape {
    pub attr: WASMShapesAttr
}

#[wasm_bindgen]
impl SoftSkiaWASM {
    #[wasm_bindgen(constructor)]
    pub fn new(id: usize) -> Self {
        utils::set_panic_hook();
        let instance = Instance::new(id);
        SoftSkiaWASM(instance)
    }

    #[wasm_bindgen(js_name = createChildAppendToContainer)]
    pub fn create_child_append_to_container(&mut self, child_id: usize, container_id: usize) {
        self.0.create_child_append_to_container(child_id, container_id)
    }

    #[wasm_bindgen(js_name = createChildInsertBeforeElementOfContainer)]
    pub fn create_child_insert_before_element_of_container(&mut self, child_id: usize, insert_before_id: usize, container_id: usize) {
        self.0.create_child_insert_before_element_of_container(child_id, insert_before_id, container_id);
    }

    #[wasm_bindgen(js_name = removeChildFromContainer)]
    pub fn remove_child_from_container(&mut self, child_id: usize, container_id: usize) {
        self.0.remove_child_from_container(child_id, container_id)
    }

    #[cfg(debug_assertions)]
    #[wasm_bindgen(js_name = toDebug)]
    pub fn to_debug(&mut self) -> String {
        format!("{:?}", self.0)
    }

    #[wasm_bindgen(js_name = toBase64)]
    pub fn to_base64(&mut self) -> String {
        let root = self.0.tree.get_root();
        let mut pixmap = match root.shape {
            Shapes::R( Rect { x, y, width, height, color, style }) => {
                Pixmap::new(width, height).unwrap()
            },
            _ => {
                Pixmap::new(0, 0).unwrap()
            }
        };

        Self::recursive_rasterization_node_to_pixmap(root, &mut pixmap);

        let data = pixmap.clone().encode_png().unwrap();
        let data_url = base64::encode(&data);
        format!("data:image/png;base64,{}", data_url)
    }

    fn recursive_rasterization_node_to_pixmap(node: &mut Node, pixmap: &mut Pixmap) -> () {
        let context = node.provider.as_ref().and_then(|p| p.get_context());

        for item in node.children.iter_mut() {
            let inactive = *context
                .and_then(|c| c.inactive_nodes_map.as_ref().and_then(|m| m.get(&item.id)))
                .unwrap_or(&false);

            if inactive {
                continue;
            }

            item.draw(pixmap, context);

            if let Some(provider) = item.provider.as_mut() {
                provider.set_context(pixmap, node.provider.as_ref());

                // 这段感觉放这有点重，想搬走
                match provider {
                    Providers::G(group) => {
                        if let Some(clip) = &group.clip {
                            if let Some(clip_id) = clip.id {
                                if let Some(clip_path) = item
                                    .children
                                    .iter_mut()
                                    .find(|n| n.id == clip_id)
                                    .and_then(|n| Some(n.shape.get_path(group.context.as_ref().unwrap())))
                                {
                                    group.set_context_mask(pixmap, &clip_path);
                                }
                            }
                        }
                    }
                }

            }

            Self::recursive_rasterization_node_to_pixmap(item, pixmap);
        }
    }

    #[wasm_bindgen(js_name = setAttrBySerde)]
    pub fn set_attr_by_serde(&mut self, id: usize, value: JsValue) {
        let message: WASMShape = serde_wasm_bindgen::from_value(value).unwrap();
        match message.attr {
            WASMShapesAttr::R(WASMRectAttr{ width, height, x, y , color, style}) => {
                let color = parse_color(color);
                let style = parse_style(style);
                self.0.set_shape_to_child(id, Shapes::R(Rect { x, y, width, height, color, style }))
            },
            WASMShapesAttr::C(WASMCircleAttr{ cx, cy, r, color, style }) => {
                let color = parse_color(color);
                let style = parse_style(style);
                self.0.set_shape_to_child(id, Shapes::C(Circle { cx, cy, r, color, style }))
            },
            WASMShapesAttr::RR(WASMRoundRectAttr{ width, height, r, x, y , color, style}) => {
                let color = parse_color(color);
                let style = parse_style(style);

                self.0.set_shape_to_child(id, Shapes::RR(RoundRect { x, y, r, width, height, color, style }))
            },
            WASMShapesAttr::L(WASMLineAttr{ p1, p2, stroke_width, color}) => {
                let color = parse_color(color);
                self.0.set_shape_to_child(id, Shapes::L(Line { p1, p2, stroke_width, color }))
            },
            WASMShapesAttr::P(WASMPointsAttr{ points , color, stroke_width, style }) => {
                let color = parse_color(color);
                let style = parse_style(style);
                self.0.set_shape_to_child(id, Shapes::P(Points { points, stroke_width, color, style }))
            },
            WASMShapesAttr::G(WASMGroupAttr {
                x,
                y,
                color,
                stroke_width,
                style,
                invert_clip,
            }) => {
                let color = parse_color(color);
                let style = parse_style(style);
                let mut clip = GroupClip::default();
                clip.invert = invert_clip;

                self.0.set_provider_to_child(
                    id,
                    Providers::G(Group {
                        x,
                        y,
                        color,
                        style,
                        stroke_width,
                        clip: Some(clip),
                        context: None,
                    }),
                )
            },
            WASMShapesAttr::GC(WASMGroupClipAttr { clip }) => {
                let provider = self
                    .0
                    .get_tree_node_by_id(id)
                    .unwrap()
                    .provider
                    .as_mut()
                    .unwrap();

                match provider {
                    Providers::G(ref mut group) => group.set_clip_id(clip),
                }
            }
            WASMShapesAttr::I(WASMImageAttr {
                x,
                y,
                image,
                width,
                height
            }) => {
                self.0.set_shape_to_child(id, Shapes::I(Image { image, x, y, width, height }))
            }
            WASMShapesAttr::T(WASMTextAttr{
                x,
                y,
                text,
                font_size,
                color
            }) => {
                let color = parse_color(color);
                let font_size = font_size.unwrap_or(16.0);
                self.0.set_shape_to_child(id, Shapes::T(Text { text, x, y, font_size, color }))
            }
        };
    }
}

fn parse_color(color: Option<String>) -> Option<ColorU8> {
    if let Some(color_str) = color {
        let mut parser_input = ParserInput::new(&color_str);
        let mut parser = Parser::new(&mut parser_input);

        if let Ok(css_color) = CSSColor::parse(&mut parser) {
            if let CSSColor::RGBA(rgba) = css_color {
                return Some(ColorU8::from_rgba(
                    rgba.red, rgba.green, rgba.blue, rgba.alpha,
                ));
            }
        }
    }
    None
}

fn parse_style(style: Option<String>) -> Option<PaintStyle> {
    match style.as_deref() {
        Some("stroke") => Some(PaintStyle::Stroke),
        Some("fill") => Some(PaintStyle::Fill),
        _ => None,
    }
}