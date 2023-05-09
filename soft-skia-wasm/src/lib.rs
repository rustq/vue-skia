extern crate soft_skia;
mod utils;
use base64;
use wasm_bindgen::prelude::*;
use soft_skia::instance::Instance;
use soft_skia::shape::Shapes;
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

    #[wasm_bindgen(js_name = setShapeToChild)]
    pub fn set_shape_rect_to_child(&mut self, child_id: usize, x: u32, y: u32, width: u32, height: u32, r: u8, g: u8, b: u8, a: u8) {
        self.0.set_shape_to_child(child_id, Shapes::R(Rect { x, y, width, height, color: ColorU8::from_rgba(r, g, b, a) }))
    }

    #[wasm_bindgen(js_name = removeChildFromContainer)]
    pub fn remove_child_from_container(&mut self, child_id: usize, container_id: usize) {
        self.0.remove_child_from_container(child_id, container_id)
    }

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

}