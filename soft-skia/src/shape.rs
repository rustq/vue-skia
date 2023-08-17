use std::collections::HashMap;

pub use tiny_skia::{ColorU8, FillRule, Mask, Paint, PathBuilder, Pixmap, Stroke, Transform};
use tiny_skia::{LineCap, LineJoin, Path, PixmapPaint};

#[derive(Debug)]
pub enum Shapes {
    R(Rect),
    C(Circle),
    RR(RoundRect),
    L(Line),
    P(Points),
    I(Image),
}

#[derive(Debug)]
pub struct DrawContext {
    pub offset_x: u32,
    pub offset_y: u32,
    pub color: Option<ColorU8>,
    pub style: Option<PaintStyle>,
    pub stroke_width: Option<u32>,
    pub mask: Option<Mask>,
    pub inactive_nodes_map: Option<HashMap<usize, bool>>,
}

impl DrawContext {
    pub fn default() -> Self {
        DrawContext {
            offset_x: 0,
            offset_y: 0,
            color: None,
            style: None,
            stroke_width: None,
            mask: None,
            inactive_nodes_map: None,
        }
    }
}

pub trait Shape {
    fn default() -> Self;
    fn draw(&self, pixmap: &mut Pixmap, context: &DrawContext) -> ();
    fn get_path(&self, context: &DrawContext) -> Path;
}

#[derive(Debug, Clone, Copy)]
pub enum PaintStyle {
    Stroke,
    Fill,
}

#[derive(Debug)]
pub struct Rect {
    pub x: u32,
    pub y: u32,
    pub width: u32,
    pub height: u32,
    pub color: Option<ColorU8>,
    pub style: Option<PaintStyle>,
}

#[derive(Debug)]
pub struct Circle {
    pub cx: u32,
    pub cy: u32,
    pub r: u32,
    pub color: Option<ColorU8>,
    pub style: Option<PaintStyle>,
}

#[derive(Debug)]
pub struct RoundRect {
    pub x: u32,
    pub y: u32,
    pub width: u32,
    pub height: u32,
    pub r: u32,
    pub color: Option<ColorU8>,
    pub style: Option<PaintStyle>,
}

#[derive(Debug)]
pub struct Line {
    pub p1: [u32; 2],
    pub p2: [u32; 2],
    pub color: Option<ColorU8>,
    pub stroke_width: Option<u32>,
}

#[derive(Debug)]
pub struct Points {
    pub points: Vec<[u32; 2]>,
    pub color: Option<ColorU8>,
    pub stroke_width: Option<u32>,
    pub style: Option<PaintStyle>,
}

#[derive(Debug)]
pub struct Image {
    pub image: String,
    pub x: i32,
    pub y: i32,
    pub width: u32,
    pub height: u32,
}

impl Shapes {
    pub fn draw(&self, pixmap: &mut Pixmap, context: &DrawContext) -> () {
        match self {
            Shapes::R(rect) => rect.draw(pixmap, context),
            Shapes::C(circle) => circle.draw(pixmap, context),
            Shapes::RR(round_rect) => round_rect.draw(pixmap, context),
            Shapes::L(line) => line.draw(pixmap, context),
            Shapes::P(points) => points.draw(pixmap, context),
            Shapes::I(image) => image.draw(pixmap, context),
        }
    }

    pub fn get_path(&self, context: &DrawContext) -> Path {
        match self {
            Shapes::R(rect) => rect.get_path(context),
            Shapes::C(circle) => circle.get_path(context),
            Shapes::RR(round_rect) => round_rect.get_path(context),
            Shapes::L(line) => line.get_path(context),
            Shapes::P(points) => points.get_path(context),
            Shapes::I(image) => image.get_path(context),
        }
    }
}

impl Shape for Rect {
    fn default() -> Self {
        Rect {
            x: 0,
            y: 0,
            width: 0,
            height: 0,
            color: None,
            style: None,
        }
    }

    fn draw(&self, pixmap: &mut Pixmap, context: &DrawContext) -> () {
        let DrawContext {
            color,
            style,
            stroke_width,
            mask,
            ..
        } = context;

        let path = self.get_path(context);

        let mut paint = Paint::default();
        let color = self
            .color
            .unwrap_or(color.unwrap_or(ColorU8::from_rgba(0, 0, 0, 255)));
        let style = self.style.unwrap_or(style.unwrap_or(PaintStyle::Fill));
        let mask = mask.as_ref();

        paint.set_color_rgba8(color.red(), color.green(), color.blue(), color.alpha());

        match style {
            PaintStyle::Stroke => {
                let mut stroke = Stroke::default();

                if let &Some(w) = stroke_width {
                    stroke.width = w as f32
                }

                pixmap.stroke_path(&path, &paint, &stroke, Transform::identity(), mask);
            }
            PaintStyle::Fill => {
                paint.anti_alias = true;
                pixmap.fill_path(
                    &path,
                    &paint,
                    FillRule::Winding,
                    Transform::identity(),
                    mask,
                );
            }
            _ => {}
        }
    }

    fn get_path(
        &self,
        DrawContext {
            offset_x, offset_y, ..
        }: &DrawContext,
    ) -> Path {
        let mut pb = PathBuilder::new();
        let x = self.x + offset_x;
        let y = self.y + offset_y;

        pb.move_to(x as f32, y as f32);
        pb.line_to((x + self.width) as f32, y as f32);
        pb.line_to((x + self.width) as f32, (y + self.height) as f32);
        pb.line_to(x as f32, (y + self.height) as f32);
        pb.line_to(x as f32, y as f32);
        pb.close();

        pb.finish().unwrap()
    }
}

impl Shape for Circle {
    fn default() -> Self {
        todo!()
    }

    fn draw(&self, pixmap: &mut Pixmap, context: &DrawContext) -> () {
        let DrawContext {
            color,
            style,
            stroke_width,
            mask,
            ..
        } = context;

        let path = self.get_path(context);

        let mut paint = Paint::default();
        let color = self
            .color
            .unwrap_or(color.unwrap_or(ColorU8::from_rgba(0, 0, 0, 255)));
        let style = self.style.unwrap_or(style.unwrap_or(PaintStyle::Fill));
        let mask = mask.as_ref();

        paint.set_color_rgba8(color.red(), color.green(), color.blue(), color.alpha());
        paint.anti_alias = true;

        match style {
            PaintStyle::Stroke => {
                let mut stroke = Stroke::default();

                if let &Some(w) = stroke_width {
                    stroke.width = w as f32
                }

                pixmap.stroke_path(&path, &paint, &stroke, Transform::identity(), mask);
            }
            PaintStyle::Fill => {
                paint.anti_alias = true;
                pixmap.fill_path(
                    &path,
                    &paint,
                    FillRule::Winding,
                    Transform::identity(),
                    mask,
                );
            }
            _ => {}
        }
    }

    fn get_path(
        &self,
        DrawContext {
            offset_x, offset_y, ..
        }: &DrawContext,
    ) -> Path {
        let mut pb: PathBuilder = PathBuilder::new();
        let x = self.cx + offset_x;
        let y = self.cy + offset_y;

        pb.push_circle(x as f32, y as f32, self.r as f32);
        pb.close();

        pb.finish().unwrap()
    }
}

impl Shape for RoundRect {
    fn default() -> Self {
        todo!()
    }

    fn draw(&self, pixmap: &mut Pixmap, context: &DrawContext) -> () {
        let DrawContext {
            color,
            style,
            stroke_width,
            mask,
            ..
        } = context;

        let path = self.get_path(context);

        let mut paint = Paint::default();
        let color = self
            .color
            .unwrap_or(color.unwrap_or(ColorU8::from_rgba(0, 0, 0, 255)));
        let style = self.style.unwrap_or(style.unwrap_or(PaintStyle::Fill));
        let mask = mask.as_ref();

        paint.set_color_rgba8(color.red(), color.green(), color.blue(), color.alpha());
        paint.anti_alias = true;

        match style {
            PaintStyle::Stroke => {
                let mut stroke = Stroke::default();

                if let &Some(w) = stroke_width {
                    stroke.width = w as f32
                }

                pixmap.stroke_path(&path, &paint, &stroke, Transform::identity(), mask);
            }
            PaintStyle::Fill => {
                paint.anti_alias = true;
                pixmap.fill_path(
                    &path,
                    &paint,
                    FillRule::Winding,
                    Transform::identity(),
                    mask,
                );
            }
            _ => {}
        }
    }

    fn get_path(
        &self,
        DrawContext {
            offset_x, offset_y, ..
        }: &DrawContext,
    ) -> Path {
        let mut pb = PathBuilder::new();
        let x = self.x + offset_x;
        let y = self.y + offset_y;

        pb.move_to((x + self.r) as f32, y as f32);
        pb.line_to((x + self.width - self.r) as f32, y as f32);
        pb.quad_to(
            (x + self.width) as f32,
            y as f32,
            (x + self.width) as f32,
            (y + self.r) as f32,
        );
        pb.line_to((x + self.width) as f32, (y + self.height - self.r) as f32);
        pb.quad_to(
            (x + self.width) as f32,
            (y + self.height) as f32,
            (x + self.width - self.r) as f32,
            (y + self.height) as f32,
        );
        pb.line_to((x + self.r) as f32, (y + self.height) as f32);
        pb.quad_to(
            x as f32,
            (y + self.height) as f32,
            x as f32,
            (y + self.height - self.r) as f32,
        );
        pb.line_to(x as f32, (y + self.r) as f32);
        pb.quad_to(x as f32, y as f32, (x + self.r) as f32, y as f32);
        pb.close();

        pb.finish().unwrap()
    }
}

impl Shape for Line {
    fn default() -> Self {
        todo!()
    }

    fn draw(&self, pixmap: &mut Pixmap, context: &DrawContext) -> () {
        let DrawContext {
            color,
            stroke_width,
            mask,
            ..
        } = context;

        let path = self.get_path(context);

        let mut paint = Paint::default();
        let color = self
            .color
            .unwrap_or(color.unwrap_or(ColorU8::from_rgba(0, 0, 0, 255)));
        let stroke_width = self.stroke_width.unwrap_or(stroke_width.unwrap_or(1));
        let mask = mask.as_ref();

        let stroke = Stroke {
            width: stroke_width as f32,
            miter_limit: 4.0,
            line_cap: LineCap::Butt,
            line_join: LineJoin::Miter,
            dash: None,
        };

        paint.set_color_rgba8(color.red(), color.green(), color.blue(), color.alpha());
        pixmap.stroke_path(&path, &paint, &stroke, Transform::identity(), mask);
    }

    fn get_path(
        &self,
        DrawContext {
            offset_x, offset_y, ..
        }: &DrawContext,
    ) -> Path {
        let mut pb = PathBuilder::new();
        let p1 = [self.p1[0] + offset_x, self.p1[1] + offset_y];
        let p2 = [self.p2[0] + offset_x, self.p2[1] + offset_y];

        pb.move_to(p1[0] as f32, p1[1] as f32);
        pb.line_to(p2[0] as f32, p2[1] as f32);
        pb.close();

        pb.finish().unwrap()
    }
}

impl Shape for Points {
    fn default() -> Self {
        todo!()
    }

    fn draw(&self, pixmap: &mut Pixmap, context: &DrawContext) -> () {
        let DrawContext {
            color,
            style,
            stroke_width,
            mask,
            ..
        } = context;

        let path = self.get_path(context);

        let mut paint = Paint::default();
        let color = self
            .color
            .unwrap_or(color.unwrap_or(ColorU8::from_rgba(0, 0, 0, 255)));
        let style = self.style.unwrap_or(style.unwrap_or(PaintStyle::Fill));
        let stroke_width = self.stroke_width.unwrap_or(stroke_width.unwrap_or(1));
        let mask = mask.as_ref();

        paint.set_color_rgba8(color.red(), color.green(), color.blue(), color.alpha());

        match style {
            PaintStyle::Stroke => {
                let stroke = Stroke {
                    width: stroke_width as f32,
                    miter_limit: 4.0,
                    line_cap: LineCap::Butt,
                    line_join: LineJoin::Miter,
                    dash: None,
                };
                pixmap.stroke_path(&path, &paint, &stroke, Transform::identity(), mask);
            }
            PaintStyle::Fill => {
                paint.anti_alias = true;
                pixmap.fill_path(
                    &path,
                    &paint,
                    FillRule::Winding,
                    Transform::identity(),
                    mask,
                );
            }
            _ => {}
        }
    }

    fn get_path(
        &self,
        DrawContext {
            offset_x, offset_y, ..
        }: &DrawContext,
    ) -> Path {
        let mut pb = PathBuilder::new();

        pb.move_to(
            (self.points[0][0] + offset_x) as f32,
            (self.points[0][1] + offset_y) as f32,
        );
        for i in 1..self.points.len() {
            pb.line_to(
                (self.points[i][0] + offset_x) as f32,
                (self.points[i][1] + offset_y) as f32,
            );
        }

        pb.close();

        pb.finish().unwrap()
    }
}

impl Shape for Image {
    fn default() -> Self {
        todo!()
    }

    fn draw(&self, pixmap: &mut Pixmap, context: &DrawContext) -> () {
        let u8_array = base64::decode(&self.image).expect("base64 decode failed");
        let p = Pixmap::decode_png(&u8_array).expect("decode png failed");
        let scale_x = self.width as f32 / p.width() as f32;
        let scale_y = self.height as f32 / p.height() as f32;
        pixmap.draw_pixmap(
            self.x,
            self.y,
            p.as_ref(),
            &PixmapPaint::default(),
            Transform::from_row(scale_x, 0.0, 0.0, scale_y, 0.0, 0.0),
            None,
        );
    }
    fn get_path(&self, context: &DrawContext) -> Path {
        let pb = PathBuilder::new();
        pb.finish().unwrap()
    }
}

#[cfg(test)]
mod test {
    use super::{ColorU8, FillRule, Paint, PathBuilder, Pixmap, Stroke, Transform};
    use crate::shape::Circle;
    use crate::shape::DrawContext;
    use crate::shape::PaintStyle;
    use crate::shape::Rect;
    use crate::shape::Shape;

    #[test]
    fn test_draw_rect() {
        let mut pixmap = Pixmap::new(400 as u32, 400 as u32).unwrap();

        let shape_0 = Rect {
            x: 20,
            y: 20,
            width: 200,
            height: 200,
            color: None,
            style: None,
        };
        let shape_1 = Rect {
            x: 120,
            y: 80,
            width: 200,
            height: 200,
            color: None,
            style: None,
        };

        shape_0.draw(&mut pixmap, &DrawContext::default());
        shape_1.draw(&mut pixmap, &DrawContext::default());

        let data = pixmap.clone().encode_png().unwrap();
        let data_url = base64::encode(&data);

        println!("{}", format!("data:image/png;base64,{}", data_url));
    }
}
