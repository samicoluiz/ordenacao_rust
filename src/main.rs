use std::io;
use rand::prelude::*;


fn bubblesort(arr: Vec<i32>) -> Vec<i32>{
    let mut ordenado = arr;
    let mut trocas = true;
    while trocas {
        trocas = false;
        for i in 0..ordenado.len() - 1 {
            if ordenado[i] > ordenado[i + 1] {
                ordenado.swap(i, i + 1);
                trocas = true;
            }
        }
    }
    return ordenado;
}

fn quick_sort(arr: Vec<i32>) -> Vec<i32> {
    if arr.len() < 2 {
    return arr;
    }
    let pivo = arr[0];
    let (menores, maiores) = arr[1..].iter().partition(|x| *x < &pivo);
    return [quick_sort(menores), vec![pivo], quick_sort(maiores)].concat();
    }

fn insertsort(arr: Vec<i32>) Vec<i32> -{
    let mut ordenado = Vec::new();
    for i in arr {
        let mut inserido = false;
        for j in 0..ordenado.len() {
            if i < ordenado[j] {
                ordenado.insert(j, i);
                inserido = true;
                break;
            }
        }
        if !inserido {
            ordenado.push(i);
        }
    }
    return ordenado;
}

fn selection_sort(Vec <i32>) -> Vec<i32> {
    let mut ordenado = arr;
    for i in 0..ordenado.len() - 1 {
        let mut menor = i;
        for j in i + 1..ordenado.len() {
            if ordenado[j] < ordenado[menor] {
                menor = j;
            }
        }
        ordenado.swap(i, menor);
    }
    return ordenado;
}

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
