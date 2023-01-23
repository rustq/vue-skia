use std::cell::RefCell;
use neon::{prelude::*, types::buffer::TypedArray};
use skia_safe::{Data, EncodedImageFormat, Paint, Rect, PaintStyle, Path, Surface};
use std::mem;
use std::fs::File;
use std::io::Write;

use skia_safe::{Color};
use colorsys::Rgb;

pub struct Canvas {
}

impl Canvas {
    pub fn new(width: i32, height: i32) -> Canvas {
        Canvas {
        }
    }
}
pub type BoxedCanvas = JsBox<RefCell<Canvas>>;
impl Finalize for Canvas {}

pub fn new(mut cx: FunctionContext) -> JsResult<BoxedCanvas> {
    let this = RefCell::new(Canvas::new(300, 200));
    Ok(cx.boxed(this))
  }

pub fn draw_triangle(mut cx: FunctionContext) -> JsResult<JsUndefined> {
    let mut surface = Surface::new_raster_n32_premul((200, 300)).expect("no surface!");
    let mut path = Path::new();
    let mut context = Paint::default();

    let color = Rgb::from_hex_str("336677").expect("Failed to create color");

    context.set_color(Color::from_rgb(color.red() as u8, color.green() as u8, color.blue() as u8));
    context.set_anti_alias(true);
    context.set_stroke_width(2.0);

    path.move_to((30, 60));
    path.line_to((80, 90));
    path.line_to((20, 100));
    path.line_to((30, 60));
    context.set_style(PaintStyle::Fill);
    surface.canvas().draw_path(&path, &context);

    let image = surface.image_snapshot();
    let d = image.encode_to_data(EncodedImageFormat::PNG).unwrap();

    let mut file = File::create("triangle.png").expect("Failed to open output png");
    let bytes = d.as_bytes();

    file.write_all(bytes).expect("Failed to write output png");
    Ok(cx.undefined())
}

pub fn draw_circle(mut cx: FunctionContext) -> JsResult<JsUndefined> {
    let mut surface = Surface::new_raster_n32_premul((200, 300)).expect("no surface!");
    let mut path = Path::new();
    let mut context = Paint::default();

    let color = Rgb::from_hex_str("ee2288").expect("Failed to create color");

    context.set_color(Color::from_rgb(color.red() as u8, color.green() as u8, color.blue() as u8));
    context.set_anti_alias(true);
    context.set_stroke_width(2.0);

    path.arc_to(Rect::new(100.0, 200.0, 140.0, 240.0), 0.0, 180.0, false);
    path.arc_to(Rect::new(100.0, 200.0, 140.0, 240.0), 180.0, 180.0, false);
    context.set_style(PaintStyle::Fill);
    surface.canvas().draw_path(&path, &context);

    let image = surface.image_snapshot();
    let d = image.encode_to_data(EncodedImageFormat::PNG).unwrap();

    let mut file = File::create("draw_circle.png").expect("Failed to open output png");
    let bytes = d.as_bytes();

    file.write_all(bytes).expect("Failed to write output png");
    Ok(cx.undefined())
}

pub fn hello(mut cx: FunctionContext) -> JsResult<JsUndefined> {
    let mut surface = Surface::new_raster_n32_premul((200, 300)).expect("no surface!");
    let mut path = Path::new();
    let mut context = Paint::default();

    let color = Rgb::from_hex_str("336677").expect("Failed to create color");

    context.set_color(Color::from_rgb(color.red() as u8, color.green() as u8, color.blue() as u8));
    context.set_anti_alias(true);
    context.set_stroke_width(2.0);

    path.move_to((30, 60));
    path.line_to((80, 90));
    path.line_to((110, 40));
    context.set_style(PaintStyle::Stroke);
    surface.canvas().draw_path(&path, &context);

    let image = surface.image_snapshot();
    let d = image.encode_to_data(EncodedImageFormat::PNG).unwrap();

    let mut file = File::create("hello.png").expect("Failed to open output png");
    let bytes = d.as_bytes();

    file.write_all(bytes).expect("Failed to write output png");
    Ok(cx.undefined())

    //
}