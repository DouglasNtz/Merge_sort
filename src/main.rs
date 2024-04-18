use rand;
use std::time::{Duration, Instant};
use lib::{my_merge_sort, my_merge_sort_optmized};
use lib::{my_merge_sort_it, my_merge_sort_optmized_it};

enum My_merge_sort<T> {
    Times(for<'a> fn(&'a mut Vec<T>)),
    Iterations(for<'a> fn(&'a mut Vec<T>) -> usize)
}

enum My_merge_sort_optmized<T> {
    Times(for<'a> fn(&'a mut Vec<T>)),
    Iterations(for<'a> fn(&'a mut Vec<T>) -> usize)
}

fn main() {

    let args: Vec<String> = std::env::args().collect();

    let numero_experimentos = args[1].parse::<usize>().unwrap();

    let tamanho_lista = args[2].parse::<usize>().unwrap();

    let func_name = args[3].as_str();

    let kind_algorithm = args[4].as_str();

    let algorithms;

    match kind_algorithm {
        "Times" => {
            algorithms = (My_merge_sort::Times(my_merge_sort), My_merge_sort_optmized::Times(my_merge_sort_optmized))
        },
        "Iterations" => {
            algorithms = (My_merge_sort::Iterations(my_merge_sort_it), My_merge_sort_optmized::Iterations(my_merge_sort_optmized_it))
        },
        _ => {
            println!("Tipo de algoritmo não existe.");
            return;
        }

    }

    let mut times = Vec::with_capacity(numero_experimentos);

    let mut start_time;
    let mut duration;

    let mut vec_iterations = Vec::with_capacity(numero_experimentos);

    let mut iterations: usize;

    for _exp in 0..numero_experimentos {

        let mut v = Vec::with_capacity(tamanho_lista);
        for _i in 0..tamanho_lista {
            v.push(rand::random::<i32>());
        }

        match func_name {
            "my_merge_sort" => {
                start_time = Instant::now();
                iterations = match algorithms.0 {
                    My_merge_sort::Times(func) => {
                        func(&mut v);
                        0
                    },
                    My_merge_sort::Iterations(func) => {
                        func(&mut v)
                    }
                };
                duration = start_time.elapsed();
            },
            "my_merge_sort_optmized" => {
                start_time = Instant::now();
                iterations = match algorithms.1 {
                    My_merge_sort_optmized::Times(func) => {
                        func(&mut v);
                        0
                    },
                    My_merge_sort_optmized::Iterations(func) => {
                        func(&mut v)
                    }
                };
                duration = start_time.elapsed();
            },
            _ => {
                println!("Nome da função não existe.");
                return;
            }
        };

        times.push(duration);
        vec_iterations.push(iterations);

    }

    if kind_algorithm == "Times" {
        println!(r###"Function: {func_name}
Número de experimentos: {numero_experimentos}
Tamanho da lista de números: {tamanho_lista}
Tempo total: {:?}
"###, times.iter().sum::<Duration>());
    } else if kind_algorithm == "Iterations" {
        println!(r###"Function: {func_name}
Número de experimentos: {numero_experimentos}
Tamanho da lista de números: {tamanho_lista}
Itarações por execução: {:?}
"###, (vec_iterations.iter().sum::<usize>() as f64) / (numero_experimentos as f64));
    }

    let mut v = vec![5,6,2];
    let r = &mut v;
    let s = r[..r.len()].to_vec();
    r;

}


