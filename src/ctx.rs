use std::cell::RefCell;
use neon::{prelude::*};
use skia_safe::{
    EncodedImageFormat, Paint, Rect, RRect, PaintStyle, Path, PathDirection,
    Point, Surface,
};
use skia_safe::{Color};
use colorsys::Rgb;
use std::cmp;
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

pub fn opt_string_arg(cx: &mut FunctionContext, idx: usize) -> Option<String>{
  match cx.argument_opt(idx as i32) {
    Some(arg) => match arg.downcast::<JsString, _>(cx) {
      Ok(v) => Some(v.value(cx)),
      Err(_e) => None
    },
    None => None
  }
}

pub fn string_arg<'a>(cx: &mut FunctionContext<'a>, idx: usize) -> NeonResult<String> {
  let exists = cx.len() > idx as i32;
  match opt_string_arg(cx, idx){
    Some(v) => Ok(v),
    None => cx.throw_type_error(
      if exists { format!("must be a string") }
      else { format!("Missing argument: expected a string for it") }
    )
  }
}


pub struct Context2d{
    pub(crate) surface: Surface,
}

impl Finalize for Context2d {}

unsafe impl Send for Context2d {}

pub type BoxedContext2d = JsBox<RefCell<Context2d>>;


pub fn new(mut cx: FunctionContext) -> JsResult<BoxedContext2d> {
    let args_0_2 = opt_float_args(&mut cx, 0..2);
    if let [width, height] = args_0_2.as_slice(){
      let surface = Surface::new_raster_n32_premul((*width as i32, *height as i32)).expect("Failed to create surface!");
      let ctx2d = Context2d {
        surface,
      };
      let this = RefCell::new(ctx2d);
      Ok(cx.boxed(this))
    } else {
      Ok(
        cx.boxed(
          RefCell::new(
            Context2d {
              surface: Surface::new_raster_n32_premul((100, 100)).expect("Failed to create surface!"),
            }
          )
        )
      )
    }
  }

pub fn encode_png(mut cx: FunctionContext) -> JsResult<JsUndefined> {
    let this = cx.argument::<BoxedContext2d>(0)?;
    let mut this = this.borrow_mut();
    let file_path = string_arg(&mut cx, 1)?;

    let image = this.surface.image_snapshot();
    let d = image.encode_to_data(EncodedImageFormat::PNG).unwrap();

    let mut file = File::create(file_path).expect("Failed to open output png");
    let bytes = d.as_bytes();

    file.write_all(bytes).expect("Failed to write output png");
    Ok(cx.undefined())
}

/*
 * Create Triangle
 *
 * context.createTriangle
 * @ ax: number, 
 * @ ay: number, 
 * @ bx: number, 
 * @ by: number, 
 * @ cx: number, 
 * @ cy: number,
 * @ fill: string,
 * @ stroke: string,
 *
 */
pub fn create_triangle(mut cx: FunctionContext) -> JsResult<JsUndefined> {
  let this = cx.argument::<BoxedContext2d>(0)?;
  let args_1_7 = opt_float_args(&mut cx, 1..7);
  let fill = string_arg(&mut cx, 7)?;
  let stroke = opt_string_arg(&mut cx, 8);
  if let [ax, ay, bx, by, cx, cy] = args_1_7.as_slice(){
      let mut this = this.borrow_mut();
      let color = Rgb::from_hex_str(&fill).expect("Failed to create color");
      let canvas = &mut this.surface.canvas();
      let mut path = Path::new();
      let mut paint = Paint::default();
  
      paint.set_color(Color::from_rgb(color.red() as u8, color.green() as u8, color.blue() as u8));
      paint.set_anti_alias(true);
      paint.set_stroke_width(2.0);
  
      path.move_to((*ax, *ay));
      path.line_to((*bx, *by));
      path.line_to((*cx, *cy));
      path.line_to((*ax, *ay));
      paint.set_style(PaintStyle::Fill);
      canvas.draw_path(&path, &paint);
      if let Some(stroke_color) = stroke {
        paint.set_style(PaintStyle::Stroke);
        let color = Rgb::from_hex_str(&stroke_color).expect("Failed to create color");
        paint.set_color(Color::from_rgb(color.red() as u8, color.green() as u8, color.blue() as u8));
        canvas.draw_path(&path, &paint);
      }
    }

  Ok(cx.undefined())
}


/*
 * Create Rect
 *
 * context.createRect
 * @ x: number, 
 * @ y: number, 
 * @ w: number, 
 * @ h: number,
 * @ fill: string,
 * @ stroke: string,
 *
 */
pub fn create_rect(mut cx: FunctionContext) -> JsResult<JsUndefined> {
  let this = cx.argument::<BoxedContext2d>(0)?;
  let args_1_5 = opt_float_args(&mut cx, 1..5);
  let fill = string_arg(&mut cx, 5)?;
  let stroke = opt_string_arg(&mut cx, 6);
  if let [x, y, w, h] = args_1_5.as_slice(){
      let mut this = this.borrow_mut();
      let color = Rgb::from_hex_str(&fill).expect("Failed to create color");
      let canvas = &mut this.surface.canvas();
      let mut path = Path::new();
      let mut paint = Paint::default();
  
      paint.set_color(Color::from_rgb(color.red() as u8, color.green() as u8, color.blue() as u8));
      paint.set_anti_alias(true);
      paint.set_stroke_width(2.0);
  
      path.move_to((*x, *y));
      path.line_to((*x + *w, *y));
      path.line_to((*x + *w, *y + *h));
      path.line_to((*x, *y + *h));
      path.line_to((*x, *y));
      paint.set_style(PaintStyle::Fill);
      canvas.draw_path(&path, &paint);
      if let Some(stroke_color) = stroke {
        paint.set_style(PaintStyle::Stroke);
        let color = Rgb::from_hex_str(&stroke_color).expect("Failed to create color");
        paint.set_color(Color::from_rgb(color.red() as u8, color.green() as u8, color.blue() as u8));
        canvas.draw_path(&path, &paint);
      }
    }

  Ok(cx.undefined())
}

/*
 * Create RoundRect
 *
 * context.createRoundRect
 * @ x: number, 
 * @ y: number, 
 * @ w: number, 
 * @ h: number,
 * @ radius: number,
 * @ fill: string,
 * @ stroke: string,
 *
 */
pub fn create_round_rect(mut cx: FunctionContext) -> JsResult<JsUndefined> {
  let this = cx.argument::<BoxedContext2d>(0)?;
  let args_1_6 = opt_float_args(&mut cx, 1..6);
  let fill = string_arg(&mut cx, 6)?;
  let stroke = opt_string_arg(&mut cx, 7);
  if let [x, y, w, h, radius] = args_1_6.as_slice(){
    let mut this = this.borrow_mut();
    let color = Rgb::from_hex_str(&fill).expect("Failed to create color");
    let canvas = &mut this.surface.canvas();
    let mut path = Path::new();
    let mut paint = Paint::default();
    paint.set_color(Color::from_rgb(color.red() as u8, color.green() as u8, color.blue() as u8));
    paint.set_anti_alias(true);
    paint.set_stroke_width(2.0);

    let rect = Rect::from_xywh(*x, *y, *w, *h);
    let rrect = RRect::new_rect_radii(rect, &[Point::new(*radius, *radius), Point::new(*radius, *radius), Point::new(*radius, *radius), Point::new(*radius, *radius)]);

    path.add_rrect(rrect, Some((PathDirection::CW, 0)));
    paint.set_style(PaintStyle::Fill);
    canvas.draw_path(&path, &paint);
    if let Some(stroke_color) = stroke {
      paint.set_style(PaintStyle::Stroke);
      let color = Rgb::from_hex_str(&stroke_color).expect("Failed to create color");
      paint.set_color(Color::from_rgb(color.red() as u8, color.green() as u8, color.blue() as u8));
      canvas.draw_path(&path, &paint);
    }
  }

  Ok(cx.undefined())
}


/*
 * Create Circle
 *
 * context.createCircle
 * @ cx: number, 
 * @ cy: number, 
 * @ r: number, 
 * @ fill: string,
 * @ stroke: string,
 *
 */
pub fn create_circle(mut cx: FunctionContext) -> JsResult<JsUndefined> {
  let this = cx.argument::<BoxedContext2d>(0)?;
  let xyr = opt_float_args(&mut cx, 1..4);
  let fill = string_arg(&mut cx, 4)?;
  let stroke = opt_string_arg(&mut cx, 5);
  if let [x, y, r] = xyr.as_slice(){
      let mut this = this.borrow_mut();
      let color = Rgb::from_hex_str(&fill).expect("Failed to create color");
      let canvas = &mut this.surface.canvas();
      let mut path = Path::new();
      let mut paint = Paint::default();

      paint.set_color(Color::from_rgb(color.red() as u8, color.green() as u8, color.blue() as u8));
      paint.set_anti_alias(true);
      paint.set_stroke_width(2.0);

      path.arc_to(Rect::new(*x, *y, *x + *r * 2.0, *y + *r * 2.0), 0.0, 180.0, false);
      path.arc_to(Rect::new(*x, *y, *x + *r * 2.0, *y + *r * 2.0), 180.0, 180.0, false);
      paint.set_style(PaintStyle::Fill);
      canvas.draw_path(&path, &paint);
      if let Some(stroke_color) = stroke {
        paint.set_style(PaintStyle::Stroke);
        let color = Rgb::from_hex_str(&stroke_color).expect("Failed to create color");
        paint.set_color(Color::from_rgb(color.red() as u8, color.green() as u8, color.blue() as u8));
        canvas.draw_path(&path, &paint);
      }
  }
  Ok(cx.undefined())
}