pub mod utils;

use clap::Parser;
use image::{DynamicImage, GenericImageView, ImageReader};
use std::path::PathBuf;
use std::time::Instant;
use utils::{quantify, saturating_add, save_as_rgb};

//Struct de argumentos que serão parseados pela clap
#[derive(Parser, Debug)]
struct Args {
    // Caminho para a imagem
    #[arg(short, long)]
    image: PathBuf,

    #[arg(short, long)]
    save: PathBuf,
    // Boolean para indicar se o programa deve ou não converter a imagem para escala de cinza
    #[arg(short, long, default_value_t = false)]
    gray: bool,

    #[arg(short, long, default_value_t = 2)]
    palette: usize,
}

fn main() -> Result<(), anyhow::Error> {
    // Captura o tempo de início
    let start = Instant::now();

    //Recebendo parâmetros que foram passados pela CLI
    let args = Args::parse();

    //Abrindo a imagem
    let mut img = ImageReader::open(args.image)?.decode()?;

    //Gerando a paleta de cores de acordo com o parâmetro da CLI
    let palette = gen_palette(args.palette);

    //Verificando se precisamos aplicar algoritmo com cores ou para escala de cinza
    if args.gray {
        img = img.grayscale();
    }

    dither(img, &args.save, palette);

    // Calcula o tempo decorrido
    let duration: std::time::Duration = start.elapsed();

    // Imprime o tempo em milissegundos
    println!("Tempo de execução: {} ms", duration.as_millis());
    Ok(())
}

fn gen_palette(size: usize) -> Vec<u8> {
    if size == 0 {
        return vec![];
    }
    if size == 1 {
        return vec![0];
    }

    let mut palette = Vec::with_capacity(size);
    for i in 0..size {
        let value = ((i as u32 * 255) / (size as u32 - 1)) as u8;
        palette.push(value);
    }
    palette
}

fn dither(img: DynamicImage, path_to_save: &PathBuf, palette: Vec<u8>) {
    //Pegando as dimensões da imagem aberta
    let (width, height) = img.dimensions();

    let mut r_channel = vec![vec![0u8; width as usize]; height as usize];
    let mut g_channel = vec![vec![0u8; width as usize]; height as usize];
    let mut b_channel = vec![vec![0u8; width as usize]; height as usize];

    //Percorrer todos os pixels da matriz
    for y in 0..height {
        for x in 0..width {
            //Pegar pixel atual (posição (x,y))
            let pixel = img.get_pixel(x, y);

            let [r, g, b, _] = pixel.0;

            r_channel[y as usize][x as usize] = r;
            g_channel[y as usize][x as usize] = g;
            b_channel[y as usize][x as usize] = b;
        }
    }

    floyd_steinberg(
        &mut r_channel,
        &mut g_channel,
        &mut b_channel,
        path_to_save,
        palette,
    );
}

fn floyd_steinberg(
    r: &mut Vec<Vec<u8>>,
    g: &mut Vec<Vec<u8>>,
    b: &mut Vec<Vec<u8>>,
    path_to_save: &PathBuf,
    palette: Vec<u8>,
) {
    //Pegando dimensões da imagem
    let height = r.len();
    let width = r[0].len();

    //Percorrendo a matriz dos canais para aplicar o algoritmo
    for y in 0..height {
        for x in 0..width {
            //Quantizando canais e pegando erro de cada pixel
            let error_r = quantify(r, x, y, &palette);
            let error_g = quantify(g, x, y, &palette);
            let error_b = quantify(b, x, y, &palette);

            // Espalhar erro para os vizinhos
            if x + 1 < width {
                r[y][x + 1] = saturating_add(r[y][x + 1], (error_r * 7 / 16) as i8);
                g[y][x + 1] = saturating_add(g[y][x + 1], (error_g * 7 / 16) as i8);
                b[y][x + 1] = saturating_add(b[y][x + 1], (error_b * 7 / 16) as i8);
            }

            if y + 1 < height {
                if x > 0 {
                    r[y + 1][x - 1] = saturating_add(r[y + 1][x - 1], (error_r * 3 / 16) as i8);
                    g[y + 1][x - 1] = saturating_add(g[y + 1][x - 1], (error_g * 3 / 16) as i8);
                    b[y + 1][x - 1] = saturating_add(b[y + 1][x - 1], (error_b * 3 / 16) as i8);
                }
                r[y + 1][x] = saturating_add(r[y + 1][x], (error_r * 5 / 16) as i8);
                g[y + 1][x] = saturating_add(g[y + 1][x], (error_g * 5 / 16) as i8);
                b[y + 1][x] = saturating_add(b[y + 1][x], (error_b * 5 / 16) as i8);

                if x + 1 < width {
                    r[y + 1][x + 1] = saturating_add(r[y + 1][x + 1], (error_r * 1 / 16) as i8);
                    g[y + 1][x + 1] = saturating_add(g[y + 1][x + 1], (error_g * 1 / 16) as i8);
                    b[y + 1][x + 1] = saturating_add(b[y + 1][x + 1], (error_b * 1 / 16) as i8);
                }
            }
        }
    }

    save_as_rgb(r, g, b, path_to_save);
}
