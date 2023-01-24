#![allow(dead_code)]
#![allow(unused_variables)]
use neon::prelude::*;
mod ctx;

#[neon::main]
fn main(mut cx: ModuleContext) -> NeonResult<()> {

  cx.export_function("encodePNG", ctx::encode_png)?;

  // APIs
  cx.export_function("createContext", ctx::new)?;
  cx.export_function("createTriangle", ctx::create_triangle)?;
  cx.export_function("createRect", ctx::create_rect)?;
  cx.export_function("createRoundRect", ctx::create_round_rect)?;
  cx.export_function("createCircle", ctx::create_circle)?;

  Ok(())
}
