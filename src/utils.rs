use std::path::PathBuf;

use image::{Rgb, RgbImage};

pub fn quantify(channel: &mut Vec<Vec<u8>>, x: usize, y: usize) -> i16 {
    let old = channel[y][x] as i16;
    let new = if old > 127 { 255 } else { 0 };
    channel[y][x] = new as u8;

    old - new
}

pub fn saturating_add(value: u8, increment: i8) -> u8 {
    if increment.is_negative() {
        value.saturating_sub(increment.wrapping_abs() as u8)
    } else {
        value.saturating_add(increment as u8)
    }
}

pub fn save_as_rgb(r: &Vec<Vec<u8>>, g: &Vec<Vec<u8>>, b: &Vec<Vec<u8>>, path: &PathBuf) {
    let height = r.len();
    let width = r[0].len();

    let mut img = RgbImage::new(width as u32, height as u32);

    for y in 0..height {
        for x in 0..width {
            img.put_pixel(x as u32, y as u32, Rgb([r[y][x], g[y][x], b[y][x]]));
        }
    }

    img.save(path).expect("Erro ao salvar imagem");
}
