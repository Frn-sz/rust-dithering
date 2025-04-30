pub mod utils;

use clap::Parser;
use image::{DynamicImage, GenericImageView, ImageReader};
use std::path::PathBuf;
use std::time::Instant;
use utils::{quantize, saturating_add, save_as_rgb};

type RgbChannels = (Vec<Vec<u8>>, Vec<Vec<u8>>, Vec<Vec<u8>>);
//Struct de argumentos que serão parseados pela clap
#[derive(Parser, Debug)]
struct Args {
    // Caminho para a imagem
    #[arg(short, long)]
    image: PathBuf,

    //Caminho para salvar o arquivo de saída
    #[arg(short, long)]
    save: PathBuf,

    // Boolean para indicar se o programa deve converter a imagem para escala de cinza
    #[arg(short, long, default_value_t = false)]
    gray: bool,

    //Tamanho da paleta de cores. Por padrão 2 (0,255). Quanto maior a paleta, menor o gap entre cada cor (e.g 3 = [0,127,255])
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

    //Verifica se é necessário converter a imagem para cinza
    if args.gray {
        img = img.grayscale();
    }

    let (width, height) = img.dimensions();

    //Aplica algoritmo de ditherização e salva os canais retornados
    let (r, g, b) = dither(img, palette);

    save_as_rgb(&r, &g, &b, height as usize, width as usize, &args.save);

    // Calcula o tempo decorrido
    let duration: std::time::Duration = start.elapsed();

    // Imprime o tempo em milissegundos
    println!("Tempo de execução: {} ms", duration.as_millis());

    Ok(())
}

fn gen_palette(size: usize) -> Vec<u8> {
    //Se o tamanho for 1,0 ou negativo, usa por padrão 2 cores por canal
    if size <= 1 {
        return vec![0, 255];
    }

    //Aloca um vetor com o tamanho desejado de paleta
    let mut palette = Vec::with_capacity(size);

    //Gera a paleta de cores de acordo com o tamanho passado
    for i in 0..size {
        let value = ((i as u32 * 255) / (size as u32 - 1)) as u8;
        palette.push(value);
    }

    palette
}

fn dither(img: DynamicImage, palette: Vec<u8>) -> RgbChannels {
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

            //Pega o valor de cada canal do pixel
            let [r, g, b, _] = pixel.0;

            //Coloca o valor de cada canal nas matrizes de canais
            r_channel[y as usize][x as usize] = r;
            g_channel[y as usize][x as usize] = g;
            b_channel[y as usize][x as usize] = b;
        }
    }

    //Executa algoritmo de Floyd Steinberg
    floyd_steinberg(
        &mut r_channel,
        &mut g_channel,
        &mut b_channel,
        height as usize,
        width as usize,
        palette,
    )
}

fn floyd_steinberg(
    r: &mut Vec<Vec<u8>>,
    g: &mut Vec<Vec<u8>>,
    b: &mut Vec<Vec<u8>>,
    height: usize,
    width: usize,
    palette: Vec<u8>,
) -> RgbChannels {
    //Percorrendo a matriz dos canais para aplicar o algoritmo
    for y in 0..height {
        for x in 0..width {
            //Quantizando canais e pegando erro de cada pixel
            let error_r = quantize(r, x, y, &palette);
            let error_g = quantize(g, x, y, &palette);
            let error_b = quantize(b, x, y, &palette);

            // Espalhar erro para os vizinhos
            if x + 1 < width {
                //Soma 7/16 do erro no pixel à direita
                r[y][x + 1] = saturating_add(r[y][x + 1], (error_r * 7 / 16) as i8);
                g[y][x + 1] = saturating_add(g[y][x + 1], (error_g * 7 / 16) as i8);
                b[y][x + 1] = saturating_add(b[y][x + 1], (error_b * 7 / 16) as i8);
            }

            if y + 1 < height {
                if x > 0 {
                    //Soma 3/16 do erro no pixel à diagonal inferior esquerda
                    r[y + 1][x - 1] = saturating_add(r[y + 1][x - 1], (error_r * 3 / 16) as i8);
                    g[y + 1][x - 1] = saturating_add(g[y + 1][x - 1], (error_g * 3 / 16) as i8);
                    b[y + 1][x - 1] = saturating_add(b[y + 1][x - 1], (error_b * 3 / 16) as i8);
                }
                //Soma 5/16 do erro no pixel abaixo
                r[y + 1][x] = saturating_add(r[y + 1][x], (error_r * 5 / 16) as i8);
                g[y + 1][x] = saturating_add(g[y + 1][x], (error_g * 5 / 16) as i8);
                b[y + 1][x] = saturating_add(b[y + 1][x], (error_b * 5 / 16) as i8);

                if x + 1 < width {
                    //Soma 1/16 do erro no pixel à diagonal inferior direita
                    r[y + 1][x + 1] = saturating_add(r[y + 1][x + 1], (error_r * 1 / 16) as i8);
                    g[y + 1][x + 1] = saturating_add(g[y + 1][x + 1], (error_g * 1 / 16) as i8);
                    b[y + 1][x + 1] = saturating_add(b[y + 1][x + 1], (error_b * 1 / 16) as i8);
                }
            }
        }
    }

    (r.to_vec(), g.to_vec(), b.to_vec())
}
