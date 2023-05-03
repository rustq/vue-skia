extern crate tree;
mod utils;
use tiny_skia::*;
use wasm_bindgen::prelude::*;
use tree::tree::Tree;
use tree::color::Color;
use tree::tree::Node;
// use base64;
use base64;
use std::cmp;
use core::ops::Range;

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

    #[wasm_bindgen(js_name = renderRootToStream)]
    pub fn render_root_to_stream(&mut self) -> Vec<u8> {

        let mut pixmap = Pixmap::new(400, 400).unwrap();

        let mut paint = PixmapPaint::default();
        paint.quality = FilterQuality::Bicubic;

        let root: &mut Node = self.0.get_root().unwrap();
        let mut result_vec: Vec<Pixmap> = Vec::new();

        let mut result_vec = Self::recursive_node_to_pixmap_vec(root, &mut result_vec);

        let transform = Transform::default();


        for pix in result_vec.iter_mut() {
            pixmap.draw_pixmap(0, 0, pix.as_ref(), &paint, transform, None);
        }
        pixmap.clone().take()
        //
        // let data = pixmap.clone().take();
        // let data_url = base64::encode(data);
        // format!("data:image/png;base64,{}", data_url.as_str())
    }

    #[wasm_bindgen(js_name = renderRootToBase64)]
    pub fn render_root_to_base64(&mut self) -> String {

        let mut pixmap = Pixmap::new(400, 400).unwrap();

        let mut paint = PixmapPaint::default();
        paint.quality = FilterQuality::Bicubic;

        let root: &mut Node = self.0.get_root().unwrap();
        let mut result_vec: Vec<Pixmap> = Vec::new();

        let mut result_vec = Self::recursive_node_to_pixmap_vec(root, &mut result_vec);

        let transform = Transform::default();


        for pix in result_vec.iter_mut() {
            pixmap.draw_pixmap(0, 0, pix.as_ref(), &paint, transform, None);
        }
        // pixmap.clone().take()
        //
        let data = pixmap.clone().encode_png().unwrap();
        let data_url = base64::encode(&data);
        format!("data:image/png;base64,{}", data_url)
    }

    fn recursive_node_to_pixmap_vec<'a>(node: &mut Node, result_vec: &'a mut Vec<Pixmap>) -> &'a mut Vec<Pixmap> {
        for item in node.node_vec_iter_mut() {
            let rect: Pixmap = Self::create_rect(item.get_x(), item.get_y(), item.get_width(), item.get_height(), item.get_background_color());
            result_vec.push(rect);
            Self::recursive_node_to_pixmap_vec(&mut (*item), result_vec);
        }
        result_vec
    }

    fn create_rect(x: i32, y: i32, width: i32, height: i32, background_color: &Color) -> Pixmap {
        let mut paint = Paint::default();
        paint.set_color_rgba8(background_color.r(), background_color.g(), background_color.b(), background_color.a());
        paint.anti_alias = true;

        let mut pb = PathBuilder::new();
        pb.move_to(x as f32, y as f32);
        pb.line_to((x + width) as f32, y as f32);
        pb.line_to((x + width) as f32, (y + height) as f32);
        pb.line_to(x as f32, (y + height) as f32);
        pb.line_to(x as f32, y as f32);
        pb.close();
        let path = pb.finish().unwrap();

        let mut pixmap = Pixmap::new(400, 400).unwrap();

        pixmap.fill_path(
            &path,
            &paint,
            FillRule::Winding,
            Transform::identity(),
            None,
        );

        let path = PathBuilder::from_rect(Rect::from_ltrb(x as f32, y as f32, x as f32 + width as f32, y as f32 + height as f32).unwrap());
        let stroke = Stroke::default();
        paint.set_color_rgba8(0, 0, 0, 255);

        pixmap.stroke_path(&path, &paint, &stroke, Transform::identity(), None); // TODO: stroke_rect

        pixmap
    }

}
