extern crate tree;
mod utils;

use wasm_bindgen::prelude::*;
use tree::tree::Tree;
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
pub struct Container {
    inner: Tree,
}

#[wasm_bindgen]
impl Container {
    #[wasm_bindgen(constructor)]
    pub fn new() -> Self {
        Container { inner: Tree::new() }
    }

    #[wasm_bindgen(js_name = initialQ)]
    pub fn initial_q(&mut self) {
        self.inner.set_root(Node::new());
    }

    #[wasm_bindgen(js_name = getWidth)]
    pub fn get_width(&mut self) -> i32 {
        self.inner.get_root().expect("must").get_width()
    }

    #[wasm_bindgen(js_name = setWidth200)]
    pub fn set_width_200(&mut self) {
        self.inner.get_root().expect("must").set_width(200);
    }
}