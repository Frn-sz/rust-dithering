use std::path::PathBuf;

use image::{Rgb, RgbImage};

pub fn quantify(channel: &mut Vec<Vec<u8>>, x: usize, y: usize, palette: &Vec<u8>) -> i16 {
    let old = channel[y][x] as i16;
    let new = find_closest_color(palette, old as u8) as i16;
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
fn find_closest_color(palette: &Vec<u8>, value: u8) -> u8 {
    if palette.is_empty() {
        return 0;
    }

    let mut closest = palette[0];
    let mut smallest_diff = (value as i16 - closest as i16).abs();

    for &color in palette {
        let diff = (value as i16 - color as i16).abs();
        if diff < smallest_diff {
            smallest_diff = diff;
            closest = color;
        }
    }

    closest
}
