use std::cell::RefCell;
use neon::{prelude::*, types::buffer::TypedArray};
use skia_safe::{
    Data, EncodedImageFormat, Paint, Rect, PaintStyle, Path,
    scalar, Budgeted, ImageInfo, ColorType, Size, Surface,
};
use skia_safe::{Color};
use colorsys::Rgb;
use std::cmp;
use std::mem;
use std::fs::File;
use std::io::Write;
use core::ops::Range;

pub fn opt_float_args(cx: &mut FunctionContext, rng: Range<usize>) -> Vec<f32>{
    let end = cmp::min(rng.end, cx.len() as usize);
    let rng = rng.start..end;
  
    let mut args:Vec<f32> = Vec::new();
    for i in rng.start..end{
      if let Some(arg) = cx.argument_opt(i as i32) {
        if let Ok(num) = arg.downcast::<JsNumber, _>(cx){
          let val = num.value(cx) as f32;
          if val.is_finite(){
            args.push(val);
          }
        }
      }
    }
    args
  }


pub struct Context2d{
    cc:String,
    path: Path,
    paint: Paint,
}

impl Context2d {
}
impl Finalize for Context2d {}

pub type BoxedContext2d = JsBox<RefCell<Context2d>>;


pub fn new(mut cx: FunctionContext) -> JsResult<BoxedContext2d> {
    let this = RefCell::new(Context2d{ cc:"qqqq".to_string(), path: Path::new(), paint: Paint::default() });
    Ok(cx.boxed(this))
  }

pub fn read_cc(mut cx: FunctionContext) -> JsResult<JsString> {
    let this = cx.argument::<BoxedContext2d>(0)?;
    let this = this.borrow();
    Ok(cx.string(this.cc.clone() as String))
}

pub fn make_a_triangle(mut cx: FunctionContext) -> JsResult<JsUndefined> {
    let this = cx.argument::<BoxedContext2d>(0)?;
    let xy = opt_float_args(&mut cx, 1..3);
    if let [x, y] = xy.as_slice(){
        let mut this = this.borrow_mut();
        let color = Rgb::from_hex_str("00ff00").expect("Failed to create color");
    
        this.paint.set_color(Color::from_rgb(color.red() as u8, color.green() as u8, color.blue() as u8));
        this.paint.set_anti_alias(true);
        this.paint.set_stroke_width(2.0);
    
        this.path.move_to((*x, *y));
        this.path.line_to((*x + 80.0, *y + 90.0));
        this.path.line_to((*x + 20.0, *y + 100.0));
        this.path.line_to((*x, *y));
        this.paint.set_style(PaintStyle::Fill);
      }

    Ok(cx.undefined())
}

pub fn make_a_circle(mut cx: FunctionContext) -> JsResult<JsUndefined> {
    let this = cx.argument::<BoxedContext2d>(0)?;
    let mut this = this.borrow_mut();
    let color = Rgb::from_hex_str("ff0000").expect("Failed to create color");

    this.paint.set_color(Color::from_rgb(color.red() as u8, color.green() as u8, color.blue() as u8));
    this.paint.set_anti_alias(true);
    this.paint.set_stroke_width(2.0);

    this.path.arc_to(Rect::new(100.0, 200.0, 140.0, 240.0), 0.0, 180.0, false);
    this.path.arc_to(Rect::new(100.0, 200.0, 140.0, 240.0), 180.0, 180.0, false);
    this.paint.set_style(PaintStyle::Fill);
    Ok(cx.undefined())
}

pub fn make_a_draw(mut cx: FunctionContext) -> JsResult<JsUndefined> {
    let this = cx.argument::<BoxedContext2d>(0)?;
    let mut this = this.borrow_mut();
    let mut surface = Surface::new_raster_n32_premul((200, 300)).expect("no surface!");
    surface.canvas().draw_path(&this.path, &this.paint);

    let image = surface.image_snapshot();
    let d = image.encode_to_data(EncodedImageFormat::PNG).unwrap();

    let mut file = File::create("make_a_draw.png").expect("Failed to open output png");
    let bytes = d.as_bytes();

    file.write_all(bytes).expect("Failed to write output png");
    Ok(cx.undefined())
}

