use image::{DynamicImage, Rgba, RgbaImage};
use rusttype::{point, Font, Scale, PositionedGlyph};
use std::collections::{HashMap, HashSet};
use unicode_normalization::UnicodeNormalization;

#[derive(Debug, Clone)]
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

    pub fn width(&self) -> u32 {
        (self.x2 - self.x1) as u32
    }

    pub fn height(&self) -> u32 {
        (self.y2 - self.y1) as u32
    }

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

fn prepare_glyphs(data:Vec<u8>, text: &str, height: f32) -> (DynamicImage, Vec<PositionedGlyph>, (u32, u32)) {
    let font = Font::from_bytes(data).unwrap();
    let scale = Scale::uniform(height);

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

    let mut image: RgbaImage = DynamicImage::new_rgba8(glyphs_width + height as u32, glyphs_height).to_rgba();

    for x in 0..image.width() {
        for y in 0..image.height() {
            image.put_pixel(
                x,
                y,
                Rgba([255, 255, 255, 0]),
            )
        }
    }
    for glyph in glyphs.iter() {
        if let Some(bounding_box) = glyph.pixel_bounding_box() {
            glyph.draw(|x, y, v| {
                if v > 0.2 {
                    image.put_pixel(
                        x + bounding_box.min.x as u32,
                        y + bounding_box.min.y as u32,
                        Rgba([color.0, color.1, color.2, 255]),
                    )
                }
            });
        }
    }
    (DynamicImage::ImageRgba8(image), glyphs, (glyphs_width, glyphs_height))
}

pub fn load_bitmap(data: Vec<u8>) -> (DynamicImage, HashMap<char, BoundingBox>) {
    let text = "╚║╗╝═╔╚║╗╝abcdefghijklmnopqrstuvwxyzABCDEFGHIJKLMNOPQRSTUVWXYZ12345567890!@#$%^&/\\*()?<>|,.1234567890:-'\"";
    let height = 100.0;
    let (image, glyphs, (glyphs_width, glyphs_height)) = prepare_glyphs(data, text, height);

    let mut map = HashMap::new();
    let mut counter = 0;

    for glyph in glyphs.iter() {
        if let Some(bounding_box) = glyph.pixel_bounding_box() {
            map.insert(text.chars().nth(counter).unwrap(), BoundingBox{
                x1: glyph.position().x as i32,
                x2: bounding_box.max.x,
                y1: 0,
                y2: height as i32,
            });
        }
        counter += 1;
    }

    map.insert(' ', BoundingBox{
        x1: glyphs_width as i32,
        x2: glyphs_width as i32 + 1,
        y1: 0,
        y2: 1,
    });

    (image, map)
}
