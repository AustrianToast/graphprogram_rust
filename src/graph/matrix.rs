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
    let mut vector: Vec<usize>;
    let mut index = 0;
    
    for k in 0..matrix1.len() {
        vector = vec![];
        for array in matrix2 {
            vector.push(array[index]);
        }
        index += 1;
        product.push(vec![]);
        for array in matrix1 {
            let sum: usize = array.iter()
                .zip(vector.iter())
                .map(|(x, y)| x * y)
                .sum();
            product[k].push(sum);
        }
    }
    product
}

pub fn show(matrix: &Vec<Vec<usize>>) {
    for vector in matrix {
        for value in vector {
            print!("{value} ");
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
