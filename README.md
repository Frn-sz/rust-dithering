# Algoritmo de ditherização de imagens - Relatório

O algoritmo escolhido foi o de Floyd-Steinberg na linguagem Rust

[Instalação do Rust](https://www.rust-lang.org/tools/install)

## Desenvolvimento

Primeiramente criamos uma struct para parsear os argumentos do terminal usando o clap:

- `image`: caminho para a imagem de entrada;
- `save`: caminho para salvar a imagem de saída;
- `gray`: indica de o programa deve converter a imagem para escala de cinza;
- `palette`: é o tamanho da paleta de cores, quanto maior for a paleta, menor é o intervalo entre cada cor. Por padrão é utilizado 2 (0,255).

Em seguida, temos a main que vai receber os argumentos do terminal, abrir a imagem, gerar a paleta de cores, converte (se necessário) a imagem para escala de cinza, aplica o algoritmo de ditherização, salva os canais ditherizados, calcula e imprime o tempo de execução.

Após a main, temos algumas funções:

- `gen_palette`: aloca um vetor com o tamanho desejado e gera a paleta de cores. Por padrão usa-se 2 cores por canal caso não houver argumento e se o tamanho for 0 ou 1;
- `dither`: separa a imagem em três canais e executa o algoritmo de Floyd Steinberg;
- `floyd_steinberg`: vai percorrer a matriz e pegar o erro após a quantização dos canais e espalhar o erro para os vizinhos.
  - soma 7/16 do erro no pixel da direita;
  - soma 3/16 do erro no pixel da diagonal inferior esquerda;
  - soma 5/16 do erro no pixel abaixo;
  - soma 1/16 do erro no pixel da diagonal inferior direita.

## Funções do arquivo utils.rs

- `quantize`: pega o valor da cor original do pixel e acha a cor mais próxima na paleta;
- `saturing_add`: limita os valores onde a soma passa de 255 ou fica menor que 0;
- `save_as_rgb`: cria uma nova imagem do mesmo tamanho, percorre os canais colocando os valores nos pixels e salva a imagem no caminho de saída;
- `find_closest_color`: vai encontrar a cor mais próxima de cada pixel, de acordo com o tamanho da paleta.
