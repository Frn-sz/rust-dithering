use std::path::PathBuf;

use image::{Rgb, RgbImage};

pub fn quantize(channel: &mut Vec<Vec<u8>>, x: usize, y: usize, palette: &Vec<u8>) -> i16 {
    //Pega o valor da cor original no pixel (x,y)
    let old = channel[y][x] as i16;
    //Acha a cor mais próxima na paleta
    let new = find_closest_color(palette, old as u8) as i16;

    //Altera o valor no canal para a nova cor
    channel[y][x] = new as u8;

    //Retorna o erro
    old - new
}

pub fn saturating_add(value: u8, increment: i8) -> u8 {
    // Verifica se o incremento é negativo
    if increment.is_negative() {
        /*  Se for negativo, converte para positivo com wrapping_abs
        e subtrai do valor original usando saturating_sub. Se o resultado for menor que 0, fica truncado em 0. */
        value.saturating_sub(increment.wrapping_abs() as u8)
    } else {
        // Se for positivo, converte para u8 e usa saturating_add. Se for maior que 255, fica truncado em 255.
        value.saturating_add(increment as u8)
    }
}

pub fn save_as_rgb(
    r: &Vec<Vec<u8>>,
    g: &Vec<Vec<u8>>,
    b: &Vec<Vec<u8>>,
    height: usize,
    width: usize,
    path: &PathBuf,
) {
    //Cria imagem nova com mesma largura e altura
    let mut img = RgbImage::new(width as u32, height as u32);

    //Percorre os canais colocando os valores nos pixels da imagem recém criada
    for y in 0..height {
        for x in 0..width {
            img.put_pixel(x as u32, y as u32, Rgb([r[y][x], g[y][x], b[y][x]]));
        }
    }

    //Salva imagem no caminho de saída
    img.save(path).expect("Erro ao salvar imagem");
}
fn find_closest_color(palette: &Vec<u8>, value: u8) -> u8 {
    if palette.is_empty() {
        return 0;
    }

    //Começa do início da paleta
    let mut closest = palette[0];
    /* Inicializa a menor diferença como sendo a diferença entre o pixel e o valor
    da primeira cor da paleta */
    let mut smallest_diff = (value as i16 - closest as i16).abs();

    //Percorre a paleta procurando a cor mais próxima do valor do pixel
    for &color in palette {
        let diff = (value as i16 - color as i16).abs();
        if diff < smallest_diff {
            smallest_diff = diff;
            closest = color;
        }
    }

    closest
}
