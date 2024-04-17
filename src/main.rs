use std::io::Take;
use rand;
use std::time::{Duration, Instant};
use Merge_sort::my_merge_sort;

fn main() {

    let args: Vec<String> = std::env::args().collect();

    let numero_experimentos = args[1].parse::<usize>().unwrap();

    let tamanho_lista = args[2].parse::<usize>().unwrap();

    let mut times = Vec::with_capacity(numero_experimentos);

    let mut start_time;
    let mut duration;

    for _exp in 0..numero_experimentos {

        let mut v = Vec::with_capacity(tamanho_lista);
        for _i in 0..tamanho_lista {
            v.push(rand::random::<i32>());
        }

        start_time = Instant::now();
        my_merge_sort(&mut v);
        duration = start_time.elapsed();

        times.push(duration);

    }


    println!(r###"Function: my_merge_sort
Número de experimentos: {numero_experimentos}
Tamanho da lista de números: {tamanho_lista}
Tempo total: {:?}
"###, times.iter().sum::<Duration>());

}


