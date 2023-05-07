use tiny_skia::{ColorU8, Paint, PathBuilder, Pixmap, Stroke, Transform, FillRule};

#[derive(Debug)]
pub enum Shape {
    Rect { x: u32, y: u32, width: u32, height: u32, color: ColorU8 },
    Circle { c: u32, r: u32, color: ColorU8 },
}

impl Shape {
    pub fn draw(&self, pixmap: &mut Pixmap) {
        let mut paint = Paint::default();
        let mut pb = PathBuilder::new();

        match self {
            Shape::Rect { x, y, width, height, color } => {
                paint.set_color_rgba8(color.red(), color.green(), color.blue(), color.alpha());
                paint.anti_alias = true;

                pb.move_to(*x as f32, *y as f32);
                pb.line_to((*x + width) as f32, *y as f32);
                pb.line_to((*x + width) as f32, (*y + height) as f32);
                pb.line_to(*x as f32, (*y + height) as f32);
                pb.line_to(*x as f32, *y as f32);
                pb.close();

                let path = pb.finish().unwrap();

                pixmap.fill_path(
                    &path,
                    &paint,
                    FillRule::Winding,
                    Transform::identity(),
                    None,
                );

                let stroke = Stroke::default();

                paint.set_color_rgba8(0, 0, 0, 255);
                pixmap.stroke_path(&path, &paint, &stroke, Transform::identity(), None);
            },
            Shape::Circle { c, r, color } => {
                todo!()
            },
        }

    }
}

#[cfg(test)]
mod test {
    use super::Shape;
    use super::{ColorU8, Paint, PathBuilder, Pixmap, Stroke, Transform, FillRule};

    #[test]
    fn test_draw_rect() {
        let mut pixmap = Pixmap::new(400 as u32, 400 as u32).unwrap();

        let shape_0 = Shape::Rect { x: 20, y: 20, width: 200, height: 200, color: ColorU8::from_rgba(0, 255, 0, 200) };
        let shape_1 = Shape::Rect { x: 120, y: 80, width: 200, height: 200, color: ColorU8::from_rgba(0, 255, 0, 200) };

        shape_0.draw(&mut pixmap);
        shape_1.draw(&mut pixmap);

        let data = pixmap.clone().encode_png().unwrap();
        let data_url = base64::encode(&data);

        println!("{}", format!("data:image/png;base64,{}", data_url));
    }
}