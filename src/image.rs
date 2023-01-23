use std::cell::RefCell;
use neon::{prelude::*, types::buffer::TypedArray};

pub struct Image{
    src:String
}
impl Finalize for Image {}

pub type BoxedImage = JsBox<RefCell<Image>>;


pub fn new(mut cx: FunctionContext) -> JsResult<BoxedImage> {
    let this = RefCell::new(Image{ src:"".to_string() });
    Ok(cx.boxed(this))
  }