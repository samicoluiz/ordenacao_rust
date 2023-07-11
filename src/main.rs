use std::io;
use rand::prelude::*;


fn bubblesort(){}

fn quick_sort(arr: Vec<i32>) -> Vec<i32> {
    if arr.len() < 2 {
    return arr;
    }
    let pivo = arr[0];
    let (menores, maiores) = arr[1..].iter().partition(|x| *x < &pivo);
    return [quick_sort(menores), vec![pivo], quick_sort(maiores)].concat();
    }

fn insertsort(){}

fn selection_sort(){}

fn merge_sort(){}

fn heap_sort(){}

fn gera_exemplo(qtd:i32) -> Vec<i32> {
    let mut rng = rand::thread_rng();
    let mut numeros: Vec<i32> = (1..qtd).collect();
    numeros.shuffle(&mut rng);
    return numeros;
}

fn main() {
    println!("Este programa ordena vetores de \
     números usando os algoritmos quicksort ou bubblesort.");
    let mut numeros = gera_exemplo(10);
    println!("Vetor gerado:");
    print!("{:?}", numeros);
    println!("\nVetor ordenado:");
    numeros = quick_sort(numeros);
    print!("{:?}", numeros);
}
