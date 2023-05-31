#[allow(unused_imports)]
use std::{fs::File, io::Read};

pub fn fill_with_random(size: usize) -> Vec<Vec<usize>> {
    let mut matrix: Vec<Vec<usize>> = vec![];
    for i in 0..size {
        matrix.push(vec![]);
        for j in 0..size {
            if i == j {
                matrix[i].push(0);
                continue;
            }
            matrix[i].push(fastrand::usize(0..2));
        }
    }
    matrix
}

pub fn mult(matrix1: &Vec<Vec<usize>>, matrix2: &Vec<Vec<usize>>) -> Vec<Vec<usize>> {
    let mut product: Vec<Vec<usize>> = vec![];
    let mut sum: usize;

    for i in 0..matrix1.len() {
        product.push(vec![]);
        for j in 0..matrix1.len() {
            sum = 0;
            for k in 0..matrix1.len() {
                sum += matrix1[i][k] * matrix2[k][j];   
            }
            product[i].push(sum);
        }
    }
    product
}

pub fn show(matrix: &Vec<Vec<usize>>) {
    for vector in matrix {
        for int in vector {
            print!("{int} ");
        }
        println!();
    }
}

pub fn read_csv() -> Vec<Vec<usize>> {
    let matrix: Vec<Vec<usize>> = vec![
        vec![0, 0, 1, 1, 0],
        vec![0, 0, 1, 1, 0],
        vec![1, 1, 0, 1, 0],
        vec![1, 1, 1, 0, 1],
        vec![0, 0, 0, 1, 0]
    ];
    /*
        let mut csv = File::open("").unwrap();
        let mut content = String::new();
        csv.read_to_string(&mut content).unwrap();
        println!("{content}");

        now I need regex to filter and put everything into a two dim vector

        See https://crates.io/crates/regex for regex crate
        See https://docs.rs/regex/1.8.3/regex/struct.Regex.html for regex crate documentation
        See https://doc.rust-lang.org/std/fs/struct.File.html for file ops
    */
    matrix
}
