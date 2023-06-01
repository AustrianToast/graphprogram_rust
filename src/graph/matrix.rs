#[allow(unused_imports)]
use std::{fs::File, io::Read};
use csv::ReaderBuilder;

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
    let mut sum: u128;

    for i in 0..matrix1.len() {
        for j in 0..matrix1.len() {
            if i == 0 {
                product.push(vec![]);
            }
            sum = 0;
            for k in 0..matrix1.len() {
                sum += (matrix1[i][k] * matrix2[k][j]) as u128;   
            }
            product[i].push(sum as usize);
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

pub fn read_csv(file_name: &str) -> Vec<Vec<usize>> {
    let mut matrix: Vec<Vec<usize>> = vec![];
    let dir: String = String::from("/home/rene/projects/Java/graphprogram/csv/");
    let file_path = dir + &file_name;
    let mut csv = ReaderBuilder::new()
        .has_headers(false)
        .delimiter(b';')
        .from_path(file_path)
        .unwrap();

    let mut index = 0;
    for result in csv.records() {
        let record = result.unwrap();
        matrix.push(vec![]);
        for field in record.iter() {
            matrix[index].push(field.parse().unwrap());
        }
        index += 1;
    }
    matrix
}
