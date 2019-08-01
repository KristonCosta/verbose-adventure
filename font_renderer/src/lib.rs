use image::{DynamicImage, Rgba, RgbaImage};
use rusttype::{point, Font, Scale};
use std::collections::HashMap;

#[derive(Debug)]
pub struct BoundingBox {
    x1: i32,
    x2: i32,
    y1: i32,
    y2: i32,
}

fn s_d(x: i32, y: i32) -> f32 {
    (x as f32 / y as f32)
}

impl BoundingBox {

    pub fn top_left(&self, scale: (i32, i32)) -> (f32, f32) {
        (s_d(self.x1, scale.0), s_d(self.y2, scale.1))

    }

    pub fn top_right(&self, scale: (i32, i32)) -> (f32, f32) {
        (s_d(self.x2, scale.0), s_d(self.y2, scale.1))
    }

    pub fn bottom_left(&self, scale: (i32, i32)) -> (f32, f32) {
        (s_d(self.x1, scale.0), s_d(self.y1, scale.1))
    }

    pub fn bottom_right(&self, scale: (i32, i32)) -> (f32, f32) {
        (s_d(self.x2, scale.0), s_d(self.y1, scale.1))
    }
}

pub fn load_bitmap(data: Vec<u8>) -> (DynamicImage, HashMap<char, BoundingBox>) {
    let font = Font::from_bytes(data).unwrap();
    let height = 100.0;
    let scale = Scale::uniform(height);
    let text = "abcdefghijklmnopqrstuvwxyzABCDEFGHIJKLMNOPQRSTUVWXYZ12345567890!@#$%^&*()?<>|";
    // let text = "ABC";
    let color = (255, 255, 255);
    let v_metrics = font.v_metrics(scale);
    let glyphs_height = (v_metrics.ascent - v_metrics.descent).ceil() as u32;
    let glyphs: Vec<_> = font
        .layout(text, scale, point(0.0, v_metrics.ascent))
        .collect();


    let glyphs_width = {
        let min_x = glyphs
            .first()
            .map(|g| g.pixel_bounding_box().unwrap().min.x)
            .unwrap();
        let max_x = glyphs
            .last()
            .map(|g| g.pixel_bounding_box().unwrap().max.x)
            .unwrap();
        (max_x - min_x) as u32
    };

    let mut image: RgbaImage = DynamicImage::new_rgba8(glyphs_width + 100, glyphs_height).to_rgba();

    for x in 0..image.width() {
        for y in 0..image.height() {
            image.put_pixel(
                x,
                y,
                Rgba([0, 0, 0, 255]),
            )
        }
    }

    let mut map = HashMap::new();
    let mut counter = 0;

    for glyph in glyphs {
        if let Some(bounding_box) = glyph.pixel_bounding_box() {
            glyph.draw(|x, y, v| {
                if v >  0.2 {
                    image.put_pixel(
                        x + bounding_box.min.x as u32,
                        y + bounding_box.min.y as u32,
                        Rgba([color.0, color.1, color.2, 255]),
                    )
                }
            });
            
            map.insert(text.chars().nth(counter).unwrap(), BoundingBox{
                x1: bounding_box.min.x,
                x2: bounding_box.max.x,
                y1: height as i32 - bounding_box.max.y - 1,
                y2: height as i32 - bounding_box.min.y + 1,
            });
        }
        counter += 1;
    }

    (DynamicImage::ImageRgba8(image), map)
}
