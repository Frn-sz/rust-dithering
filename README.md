# Algoritmo de ditherização de imagens - Relatório

O algoritmo escolhido foi o de Floyd-Steinberg, usando a linguagem Rust.

[Instalação do Rust](https://www.rust-lang.org/tools/install)

## Desenvolvimento

Primeiramente criamos uma struct para parsear os argumentos do terminal usando o clap:

- `image`: caminho para a imagem de entrada;
- `save`: caminho para salvar a imagem de saída;
- `gray`: indica se o programa deve converter a imagem para escala de cinza;
- `palette`: é o tamanho da paleta de cores, quanto maior for a paleta, menor é o intervalo entre cada cor. Por padrão é utilizado 2 (0,255).

Na main, temos as seguintes operações (quebradas em diversas funções):
- Receber os argumentos do terminal;
- Abrir a imagem;
- Gerar a paleta de cores; 
- Converter (se necessário) a imagem para escala de cinza;
- Aplicar o algoritmo de ditherização;
- Salvar os canais ditherizados;
- Calcular e imprimir o tempo de execução.

Após a main, temos a declaração das funções auxíliares:

- `gen_palette`: aloca um vetor com o tamanho desejado e gera a paleta de cores. Por padrão usa-se 2 cores por canal caso não houver argumento (--palette/-p) e se o tamanho for 0 ou 1;
- `dither`: separa a imagem em três canais e executa o algoritmo de Floyd Steinberg;
- `floyd_steinberg`: vai percorrer a matriz e pegar o erro após a quantização dos canais e espalhar o erro para os vizinhos.
  - soma 7/16 do erro no pixel da direita;
  - soma 3/16 do erro no pixel da diagonal inferior esquerda;
  - soma 5/16 do erro no pixel abaixo;
  - soma 1/16 do erro no pixel da diagonal inferior direita.

## Funções do arquivo utils.rs

- `quantize`: pega o valor da cor original do pixel e acha a cor mais próxima na paleta;
- `saturing_add`: realiza uma adição e limita os valores onde a soma passa de 255 ou fica menor que 0;
- `save_as_rgb`: cria uma nova imagem do mesmo tamanho, percorre os canais colocando os valores nos pixels e salva a imagem no caminho de saída;
- `find_closest_color`: vai encontrar a cor mais próxima de cada pixel, de acordo com as cores na paleta.

## Como executar o program

Com o `cargo` já instalado, basta executar o seguinte comando no terminal

*: `--release` é uma flag para executar uma build de release mais otimizida.
**: Todos os argumentos para o programa devem ser passados depois do `--` 

```bash
cargo run --release -- -i <Caminho da imagem> -s <Caminho de saída>
```

Existem também 2 argumentos opcionais, são eles:

- `-g`: Com essa flag, o programa converte a imagem da entrada para escala de cinza antes de aplicar o algoritmo.
- `-p <Tamanho da paleta>`: Com essa flag, é possível aumentar o tamanho da paleta de cores (e.g 2 = [0,255], 3 = [0,127,255], 4 = [0,85,170,255])

Todas as flags tem seus tamanhos curtos e longos

- `-i` ou `--image`
- `-s` ou `--save`
- `-p` ou `--palette`
- `-g` ou `--gray`

É possível utilizar `--help` ou `-h` para um exemplo de uso dos argumentos do programa