use csv::ReaderBuilder;

pub fn fill_with_random(size: usize) -> Vec<Vec<u8>> {
    let mut matrix: Vec<Vec<u8>> = vec![];
    for i in 0..size {
        matrix.push(vec![]);
        for j in 0..size {
            if i == j {
                matrix[i].push(0);
                continue;
            }
            matrix[i].push(fastrand::u8(0..2));
        }
    }
    matrix
}

pub fn mult(matrix1: &Vec<Vec<usize>>, matrix2: &Vec<Vec<u8>>) -> Vec<Vec<usize>> {
    let mut product: Vec<Vec<usize>> = vec![vec![]; matrix2.len()];
    let mut vector: Vec<usize>;

    for (index, k) in (0..matrix1.len()).enumerate() {
        vector = vec![];
        for array in matrix2 {
            vector.push(array[index].into());
        }
        for array in matrix1 {
            let sum = array.iter().zip(vector.iter()).map(|(x, y)| x * y).sum();
            //println!("{sum}");
            product[k].push(sum);
        }
    }
    product
}

pub fn clone(matrix: &Vec<Vec<u8>>) -> Vec<Vec<usize>> {
    let mut new_matrix: Vec<Vec<usize>> = vec![];

    for i in 0..matrix.len() {
        new_matrix.push(vec![]);
        for j in 0..matrix.len() {
            new_matrix[i].push(matrix[i][j].into());
        }
    }

    new_matrix
}

pub fn show(matrix: &Vec<Vec<u8>>) {
    for vector in matrix {
        for value in vector {
            print!("{value} ");
        }
        println!();
    }
}

pub fn read_csv(file_name: &str) -> Vec<Vec<u8>> {
    let mut matrix: Vec<Vec<u8>> = vec![];
    let dir: String = "/home/rene/projects/Java/graphprogram/csv/".into();
    let mut csv = ReaderBuilder::new()
        .has_headers(false)
        .delimiter(b';')
        .from_path(dir + file_name)
        .unwrap();

    for (index, result) in csv.records().enumerate() {
        let record = result.unwrap();
        matrix.push(vec![]);
        for field in record.iter() {
            matrix[index].push(field.parse().unwrap());
        }
    }
    matrix
}
