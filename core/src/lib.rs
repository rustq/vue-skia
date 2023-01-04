use neon::prelude::*;
mod image;
mod canvas;


#[neon::main]
fn main(mut cx: ModuleContext) -> NeonResult<()> {

  cx.export_function("Image_new", image::new)?;
  cx.export_function("Canvas_new", canvas::new)?;
  cx.export_function("Canvas_hello", canvas::hello)?;
  cx.export_function("Canvas_draw_triangle", canvas::draw_triangle)?;
  cx.export_function("Canvas_draw_circle", canvas::draw_circle)?;

  Ok(())
}
