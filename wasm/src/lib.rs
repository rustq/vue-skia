extern crate tree;
mod utils;

use wasm_bindgen::prelude::*;
use tree::tree::Tree;
use tree::color::Color;
use tree::tree::Node;

// When the `wee_alloc` feature is enabled, use `wee_alloc` as the global
// allocator.
#[cfg(feature = "wee_alloc")]
#[global_allocator]
static ALLOC: wee_alloc::WeeAlloc = wee_alloc::WeeAlloc::INIT;

#[wasm_bindgen]
extern {
    fn alert(s: &str);
}


#[wasm_bindgen]
pub struct Container(Tree);

#[wasm_bindgen]
impl Container {
    #[wasm_bindgen(constructor)]
    pub fn new() -> Self {
        let mut container = Container(Tree::new());
        container.0.set_root(Node::new());
        container
    }

    #[wasm_bindgen(js_name = createNodeOnParentReturnNodeID)]
    pub fn create_node_on_parent_ret_node_id(&mut self, parent_id: usize) -> usize {
        let root: &mut Node = self.0.get_root().unwrap();
        let parent = root.find_node_by_id(parent_id).unwrap();
        let node = Box::new(Node::new());
        let id = node.id;
        parent.append_boxed(node);
        id
    }

    #[wasm_bindgen(js_name = setXYWHBByNodeID)]
    pub fn set_x_y_width_height_background_color_by_node_id(&mut self, node_id: usize, x: i32, y: i32, width: i32, height: i32, a: u8, r: u8, g: u8, b: u8) {
        let root: &mut Node = self.0.get_root().unwrap();
        let node = root.find_node_by_id(node_id).unwrap();
        node.set_x(x);
        node.set_y(y);
        node.set_width(width);
        node.set_height(height);
        node.set_background_color(Color::from_argb(a, r, g, b));
    }

    #[wasm_bindgen(js_name = debug)]
    pub fn debug(&self) -> String {
        format!("{:?}", self.0)
    }

}
