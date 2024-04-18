# Merge_sort

OBS: A ordem do algoritmo merge sort é c * n * log2(n).

Algoritmos de ordenação de vetor de números escritos em Rust

Para executar um dos algoritmos, basta digitar o seguinte comando:

cargo run numero_experimentos tamanho_vetor metodo_de_ordenacao tipo_output >> nome_arquivo.txt

- numero_experimento é um número inteior que representa a quantidade de vezes que geraremos um vetor aleatório e faremos sua ordenação in-place. Faremos parse dessa String para usize.

- tamanho_vetor é um número inteiro que representa a quantidade números aleatórios que serão gerados e armazenados no vetor, o qual faremos sua ordenação in-place. Deve ser possível fazer parse para usize.

Em metodo_de_ordenacao devemos digitar um dentre os quatro metodos disponíveis:

- my_merge_sort: implementação clássica do merge sort.
  
Em tipo_output devemos digitar uma das seguintes opções:

- Times: mostra o tempo total de todas as execuções (a soma dos tempos de cada execução).

- Iterations: mostra a média do número simplificado de iterações por execução, no sentido em que cada looping registra uma iteração.

Exemplos:

cargo run --release 100 5000 my_merge_sort Times >> resultados.txt

Escreve o seguinte output no arquivo:

//---------------------------

Function: my_merge_sort

Número de experimentos: 100

Tamanho da lista de números: 5000

Tempo total: 47.191591ms

//--------------------------

cargo run --release 100 5000 my_merge_sort Iterations >> resultados.txt

Escreve o seguinte output no arquivo:

//--------------------------

Function: my_insertion_sort

Número de experimentos: 100

Tamanho da lista de números: 5000

Iterações por execução: 66808.0
