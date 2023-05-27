extern crate soft_skia;
mod utils;

use base64;
use wasm_bindgen::prelude::*;
use soft_skia::instance::Instance;
use soft_skia::shape::{Circle, Line, Points, RoundRect, Shapes};
use soft_skia::shape::Rect;
use soft_skia::shape::ColorU8;
use soft_skia::tree::Node;
use soft_skia::shape::Shape;
use soft_skia::shape::Pixmap;

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
    color: [u8; 4]
}

#[derive(Serialize, Deserialize, Debug)]
pub struct WASMCircleAttr {
    cx: u32,
    cy: u32,
    r: u32,
    color: [u8; 4]
}

#[derive(Serialize, Deserialize, Debug)]
pub struct WASMRoundRectAttr {
    width: u32,
    height: u32,
    r: u32,
    x: u32,
    y: u32,
    color: [u8; 4]
}

#[derive(Serialize, Deserialize, Debug)]
pub struct WASMLineAttr {
    p1: [u32; 2],
    p2: [u32; 2],
    color: [u8; 4]
}

#[derive(Serialize, Deserialize, Debug)]
pub struct WASMPointsAttr {
    points: Vec<[u32; 2]>,
    color: [u8; 4],
    stroke_width: u32
}

#[derive(Serialize, Deserialize, Debug)]
pub enum WASMShapesAttr {
    R(WASMRectAttr),
    C(WASMCircleAttr),
    RR(WASMRoundRectAttr),
    L(WASMLineAttr),
    P(WASMPointsAttr),
}

impl WASMShapesAttr {

}

#[derive(Serialize, Deserialize, Debug)]
pub struct WASMShape {
    pub attr: WASMShapesAttr
}

#[wasm_bindgen]
impl SoftSkiaWASM {
    #[wasm_bindgen(constructor)]
    pub fn new() -> Self {
        let mut instance = Instance::default();
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
            Shapes::R( Rect { x, y, width, height, color }) => {
                Pixmap::new(width, height).unwrap()
            },
            _ => {
                Pixmap::new(0, 0).unwrap()
            }
        };

        Self::__debug_recursive_node_to_pixmap_vec(root, &mut pixmap);

        let data = pixmap.clone().encode_png().unwrap();
        let data_url = base64::encode(&data);
        format!("data:image/png;base64,{}", data_url)
    }

    ///
    /// DEBUG
    ///
    fn __debug_recursive_node_to_pixmap_vec<'a>(node: &mut Node, pixmap: &mut Pixmap) -> () {
        for item in node.children_iter_mut() {
            item.shape.draw(pixmap);
            Self::__debug_recursive_node_to_pixmap_vec(&mut (*item), pixmap);
        }
    }


    #[wasm_bindgen(js_name = setShapeBySerde)]
    pub fn set_shape_by_serde(&mut self, id: usize, value: JsValue) {
        let message: WASMShape = serde_wasm_bindgen::from_value(value).unwrap();

        match message.attr {
            WASMShapesAttr::R(WASMRectAttr{ width, height, x, y , color}) => {
                self.0.set_shape_to_child(id, Shapes::R(Rect { x, y, width, height, color: ColorU8::from_rgba(color[0], color[1], color[2], color[3]) }))
            },
            WASMShapesAttr::C(WASMCircleAttr{ cx, cy, r, color }) => {
                self.0.set_shape_to_child(id, Shapes::C(Circle { cx, cy, r, color: ColorU8::from_rgba(color[0], color[1], color[2], color[3]) }))
            },
            WASMShapesAttr::RR(WASMRoundRectAttr{ width, height, r, x, y , color}) => {
                self.0.set_shape_to_child(id, Shapes::RR(RoundRect { x, y, r, width, height, color: ColorU8::from_rgba(color[0], color[1], color[2], color[3]) }))
            },
            WASMShapesAttr::L(WASMLineAttr{ p1, p2 , color}) => {
                self.0.set_shape_to_child(id, Shapes::L(Line { p1, p2, color: ColorU8::from_rgba(color[0], color[1], color[2], color[3]) }))
            },
            WASMShapesAttr::P(WASMPointsAttr{ points , color, stroke_width }) => {
                self.0.set_shape_to_child(id, Shapes::P(Points { points, stroke_width, color: ColorU8::from_rgba(color[0], color[1], color[2], color[3]) }))
            },
        };
    }
}
