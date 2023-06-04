pub use tiny_skia::{ColorU8, Paint, PathBuilder, Pixmap, Stroke, Transform, FillRule};
use tiny_skia::{LineCap, LineJoin};

#[derive(Debug)]
pub enum Shapes {
    R(Rect),
    C(Circle),
    RR(RoundRect),
    L(Line),
    P(Points),
}

pub trait Shape {
    fn default() -> Self;
    fn draw(&self, pixmap: &mut Pixmap) -> ();
}

#[derive(Debug, Clone, Copy)]
pub enum PaintStyle {
    Stroke,
    Fill
}

#[derive(Debug)]
pub struct Rect {
    pub x: u32,
    pub y: u32,
    pub width: u32,
    pub height: u32,
    pub color: ColorU8,
    pub style: PaintStyle,
}

#[derive(Debug)]
pub struct Circle {
    pub cx: u32,
    pub cy: u32,
    pub r: u32,
    pub color: ColorU8,
    pub style: PaintStyle,
}

#[derive(Debug)]
pub struct RoundRect {
    pub x: u32,
    pub y: u32,
    pub width: u32,
    pub height: u32,
    pub r: u32,
    pub color: ColorU8,
    pub style: PaintStyle,
}

#[derive(Debug)]
pub struct Line {
    pub p1: [u32; 2],
    pub p2: [u32; 2],
    pub color: ColorU8,
    pub stroke_width: u32
}

#[derive(Debug)]
pub struct Points {
    pub points: Vec<[u32; 2]>,
    pub color: ColorU8,
    pub stroke_width: u32,
    pub style: PaintStyle,
}

impl Shapes {
    pub fn draw(&self, pixmap: &mut Pixmap) -> () {
        match self {
            Shapes::R(rect) => rect.draw(pixmap),
            Shapes::C(circle) => circle.draw(pixmap),
            Shapes::RR(round_rect) => round_rect.draw(pixmap),
            Shapes::L(line) => line.draw(pixmap),
            Shapes::P(points) => points.draw(pixmap),
        }
    }
}

impl Shape for Rect {
    fn default() -> Self {
        Rect { x: 0, y: 0, width: 0, height: 0, color: ColorU8::from_rgba(0, 0, 0, 255), style: PaintStyle::Fill }
    }

    fn draw(&self, pixmap: &mut Pixmap) -> () {
        let mut pb = PathBuilder::new();
        let mut paint = Paint::default();

        pb.move_to(self.x as f32, self.y as f32);
        pb.line_to((self.x + self.width) as f32, self.y as f32);
        pb.line_to((self.x + self.width) as f32, (self.y + self.height) as f32);
        pb.line_to(self.x as f32, (self.y + self.height) as f32);
        pb.line_to(self.x as f32, self.y as f32);
        pb.close();

        let path = pb.finish().unwrap();
        paint.set_color_rgba8(self.color.red(), self.color.green(), self.color.blue(), self.color.alpha());

        match self.style {
            PaintStyle::Stroke => {
                let stroke = Stroke::default();
                pixmap.stroke_path(&path, &paint, &stroke, Transform::identity(), None);
            },
            PaintStyle::Fill => {
                paint.anti_alias = true;
                pixmap.fill_path(
                    &path,
                    &paint,
                    FillRule::Winding,
                    Transform::identity(),
                    None,
                );
    
            },
            _ => {}
        }

    }
}

impl Shape for Circle {
    fn default() -> Self {
        todo!()
    }

    fn draw(&self, pixmap: &mut Pixmap) -> () {
        let mut paint = Paint::default();
        let mut pb = PathBuilder::new();

        paint.set_color_rgba8(self.color.red(), self.color.green(), self.color.blue(), self.color.alpha());
        paint.anti_alias = true;

        pb.push_circle(self.cx as f32, self.cy as f32, self.r as f32);
        pb.close();

        let path = pb.finish().unwrap();

        match self.style {
            PaintStyle::Stroke => {
                let stroke = Stroke::default();
                pixmap.stroke_path(&path, &paint, &stroke, Transform::identity(), None);
            },
            PaintStyle::Fill => {
                paint.anti_alias = true;
                pixmap.fill_path(
                    &path,
                    &paint,
                    FillRule::Winding,
                    Transform::identity(),
                    None,
                );
    
            },
            _ => {}
        }
    }
}

impl Shape for RoundRect {
    fn default() -> Self {
        todo!()
    }

    fn draw(&self, pixmap: &mut Pixmap) -> () {
        let mut paint = Paint::default();
        let mut pb = PathBuilder::new();

        paint.set_color_rgba8(self.color.red(), self.color.green(), self.color.blue(), self.color.alpha());
        paint.anti_alias = true;

        pb.move_to((self.x + self.r) as f32, self.y as f32);
        pb.line_to((self.x + self.width - self.r) as f32, self.y as f32);
        pb.quad_to((self.x + self.width) as f32, self.y as f32, (self.x + self.width) as f32, (self.y + self.r) as f32);
        pb.line_to((self.x + self.width) as f32, (self.y + self.height - self.r) as f32);
        pb.quad_to((self.x + self.width) as f32, (self.y + self.height) as f32, (self.x + self.width - self.r) as f32, (self.y + self.height) as f32);
        pb.line_to((self.x + self.r) as f32, (self.y + self.height) as f32);
        pb.quad_to(self.x as f32, (self.y + self.height) as f32, self.x as f32, (self.y + self.height - self.r) as f32);
        pb.line_to(self.x as f32, (self.y + self.r) as f32);
        pb.quad_to(self.x as f32, self.y as f32, (self.x + self.r) as f32, self.y as f32);
        pb.close();

        let path = pb.finish().unwrap();

        match self.style {
            PaintStyle::Stroke => {
                let stroke = Stroke::default();
                pixmap.stroke_path(&path, &paint, &stroke, Transform::identity(), None);
            },
            PaintStyle::Fill => {
                paint.anti_alias = true;
                pixmap.fill_path(
                    &path,
                    &paint,
                    FillRule::Winding,
                    Transform::identity(),
                    None,
                );
    
            },
            _ => {}
        }
    }
}


impl Shape for Line {
    fn default() -> Self {
        todo!()
    }

    fn draw(&self, pixmap: &mut Pixmap) -> () {
        let mut pb = PathBuilder::new();
        let mut paint = Paint::default();

        pb.move_to(self.p1[0] as f32, self.p1[1] as f32);
        pb.line_to(self.p2[0] as f32, self.p2[1] as f32);
        pb.close();

        let path = pb.finish().unwrap();
        let stroke = Stroke {
            width: self.stroke_width as f32,
            miter_limit: 4.0,
            line_cap: LineCap::Butt,
            line_join: LineJoin::Miter,
            dash: None,
        };

        paint.set_color_rgba8(self.color.red(), self.color.green(), self.color.blue(), self.color.alpha());
        pixmap.stroke_path(&path, &paint, &stroke, Transform::identity(), None);
    }
}

impl Shape for Points {
    fn default() -> Self {
        todo!()
    }

    fn draw(&self, pixmap: &mut Pixmap) -> () {
        let mut pb = PathBuilder::new();
        let mut paint = Paint::default();
        paint.set_color_rgba8(self.color.red(), self.color.green(), self.color.blue(), self.color.alpha());

        pb.move_to(self.points[0][0] as f32, self.points[0][1] as f32);
        for i in 1..self.points.len() {
            pb.line_to(self.points[i][0] as f32, self.points[i][1] as f32);
        }
        pb.close();

        let path = pb.finish().unwrap();

        match self.style {
            PaintStyle::Stroke => {
                let stroke = Stroke {
                    width: self.stroke_width as f32,
                    miter_limit: 4.0,
                    line_cap: LineCap::Butt,
                    line_join: LineJoin::Miter,
                    dash: None,
                };
                pixmap.stroke_path(&path, &paint, &stroke, Transform::identity(), None);
            },
            PaintStyle::Fill => {
                paint.anti_alias = true;
                pixmap.fill_path(
                    &path,
                    &paint,
                    FillRule::Winding,
                    Transform::identity(),
                    None,
                );
    
            },
            _ => {}
        }
    }
}

#[cfg(test)]
mod test {
    use crate::shape::Rect;
    use crate::shape::Circle;
    use crate::shape::PaintStyle;
    use crate::shape::Shape;
    use super::{ColorU8, Paint, PathBuilder, Pixmap, Stroke, Transform, FillRule};

    #[test]
    fn test_draw_rect() {
        let mut pixmap = Pixmap::new(400 as u32, 400 as u32).unwrap();

        let shape_0 = Rect { x: 20, y: 20, width: 200, height: 200, color: ColorU8::from_rgba(0, 255, 0, 200), style: PaintStyle::Fill };
        let shape_1 = Rect { x: 120, y: 80, width: 200, height: 200, color: ColorU8::from_rgba(0, 255, 0, 200), style: PaintStyle::Fill };

        shape_0.draw(&mut pixmap);
        shape_1.draw(&mut pixmap);

        let data = pixmap.clone().encode_png().unwrap();
        let data_url = base64::encode(&data);

        println!("{}", format!("data:image/png;base64,{}", data_url));
    }
}