pub fn fill_with_random(size: usize) -> Vec<Vec<usize>> {
    let mut matrix = vec![vec![0; size]; size];
    let mut random_value: usize;

    for i in 0..size {
        for j in 0..size {
            random_value = fastrand::usize(0..=1);
            matrix[i][j] = random_value;
            matrix[j][i] = random_value;
            matrix[i][i] = 0;
        }
    }
    matrix
}

pub fn mult(matrix1: &Vec<Vec<usize>>, matrix2: &Vec<Vec<usize>>) -> Vec<Vec<usize>> {
    let mut product = vec![vec![]; matrix2.len()];
    let mut vector: Vec<usize>;

    for (index, k) in (0..matrix1.len()).enumerate() {
        vector = vec![];
        for array in matrix2 {
            vector.push(array[index]);
        }
        for array in matrix1 {
            product[k].push(array.iter().zip(vector.iter()).map(|(x, y)| x * y).sum());
        }
    }
    product
}

pub fn dfs_bridges(
    bridges: &mut Vec<Vec<usize>>,
    adjazenz_matrix: &Vec<Vec<usize>>,
    visited: &mut Vec<bool>,
    discovery_time: &mut Vec<usize>,
    low_time: &mut Vec<usize>,
    time: usize,
    vertex: usize,
    parent: usize,
) {
    let time = time + 1;
    visited[vertex] = true;
    discovery_time[vertex] = time;
    low_time[vertex] = time;

    for neighbor in 0..adjazenz_matrix.len() {
        if adjazenz_matrix[vertex][neighbor] != 1 {
            continue;
        }
        
        if !visited[neighbor] {
            dfs_bridges(
                bridges,
                adjazenz_matrix,
                visited,
                discovery_time,
                low_time,
                time,
                neighbor,
                vertex,
            );
    
            low_time[vertex] = usize::min(low_time[vertex], low_time[neighbor]);
    
            if discovery_time[vertex] < low_time[neighbor] {
                bridges.push(vec![vertex + 1, neighbor + 1]);
            }
        } else if neighbor != parent {
            low_time[vertex] = usize::min(low_time[vertex], discovery_time[neighbor]);
        }
    }
}

pub fn dfs_articulations(
    articulations: &mut Vec<usize>,
    is_articulation: &mut Vec<bool>,
    adjazenz_matrix: &Vec<Vec<usize>>,
    visited: &mut Vec<bool>,
    discovery_time: &mut Vec<usize>,
    low_time: &mut Vec<usize>,
    time: usize,
    vertex: usize,
    parent: usize,
) {
    let time = time + 1;
    visited[vertex] = true;
    discovery_time[vertex] = time;
    low_time[vertex] = time;
    let mut child_count: usize = 0;
    let mut articulation = false;

    for neighbor in 0..adjazenz_matrix.len() {
        if adjazenz_matrix[vertex][neighbor] != 1{
            continue;
        }
        if !visited[neighbor] {
            child_count += 1;
            dfs_articulations(
                articulations,
                is_articulation,
                adjazenz_matrix,
                visited,
                discovery_time,
                low_time,
                time,
                neighbor,
                vertex,
            );

            low_time[vertex] = usize::min(low_time[vertex], low_time[neighbor]);

            if parent != usize::MAX && discovery_time[vertex] <= low_time[neighbor] {
                articulation = true;
            }
        } else if neighbor != parent {
            low_time[vertex] = usize::min(low_time[vertex], discovery_time[neighbor]);
        }
    }

    if parent == usize::MAX && child_count > 1 {
        articulation = true;
    }

    if articulation {
        is_articulation[vertex] = true;
        articulations.push(vertex + 1);
    }
}

pub fn clone(matrix: &Vec<Vec<usize>>) -> Vec<Vec<usize>> {
    let mut new_matrix: Vec<Vec<usize>> = vec![];

    for i in 0..matrix.len() {
        new_matrix.push(vec![]);
        for j in 0..matrix.len() {
            new_matrix[i].push(matrix[i][j]);
        }
    }

    new_matrix
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
    let dir: String = "/home/rene/projects/Java/graphprogram/csv/".into();
    let mut csv = csv::ReaderBuilder::new()
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
