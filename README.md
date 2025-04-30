# Algoritmo de ditherização de imagens - Relatório

O algoritmo escolhido foi o de Floyd-Steinberg na linguagem Rust

## Instalação

https://www.rust-lang.org/tools/install

## Desenvolvimento

Primeiramente criamos uma struct para parsear os argumentos do terminal usando o clap:

- _image_: caminho para a imagem de entrada;
- _save_: caminho para salvar a imagem de saída;
- _gray_: indica de o programa deve converter a imagem para escala de cinza;
- _palette_: é o tamanho da paleta de cores, quanto maior for a paleta, menor é o intervalo entre cada cor. Por padrão é utilizado 2 (0,255).

Em seguida, temos a main que vai receber os argumentos do terminal, abrir a imagem, gerar a paleta de cores, converte (se necessário) a imagem para escala de cinza, aplica o algoritmo de ditherização, calcula e imprime o tempo de execução.

Após a main, temos algumas funções:

- _gen_palette_: aloca um vetor com o tamanho desejado e gera a paleta de cores. Por padrão usa-se 2 cores por canal caso não houver argumento e se o tamanho for 0 ou 1;
- _dither_: separa a imagem em três canais e executa o algoritmo de Floyd Steinberg;
- _floyd_steinberg_: vai percorrer a matriz e pegar o erro após a quantização dos canais e espalhar o erro para os vizinhos.
  - soma 7/16 do erro no pixel da direita;
  - soma 3/16 do erro no pixel da diagonal inferior esquerda;
  - soma 5/16 do erro no pixel abaixo;
  - soma 1/16 do erro no pixel da diagonal inferior direita.
- \*\*
- \*\*
