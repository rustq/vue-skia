use neon::prelude::*;
mod image;
mod canvas;
mod ctx;


#[neon::main]
fn main(mut cx: ModuleContext) -> NeonResult<()> {

  // cx.export_function("Image_new", image::new)?;
  // cx.export_function("Canvas_new", canvas::new)?;
  // cx.export_function("Canvas_hello", canvas::hello)?;
  // cx.export_function("Canvas_draw_triangle", canvas::draw_triangle)?;
  // cx.export_function("Canvas_draw_circle", canvas::draw_circle)?;
  // cx.export_function("Canvas_create_a_context", ctx::new)?;
  // cx.export_function("Context_read_cc", ctx::read_cc)?;
  // cx.export_function("Context_make_a_triangle", ctx::make_a_triangle)?;
  // cx.export_function("Context_make_a_circle", ctx::make_a_circle)?;
  cx.export_function("Context_make_a_draw", ctx::make_a_draw)?;

  // APIs
  cx.export_function("createContext", ctx::new)?;
  cx.export_function("createTriangle", ctx::create_triangle)?;
  cx.export_function("createRect", ctx::create_rect)?;
  cx.export_function("createCircle", ctx::create_circle)?;

  Ok(())
}
