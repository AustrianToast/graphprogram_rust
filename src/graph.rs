use matrix::{clone, dfs_articulations, dfs_bridges, mult};

pub mod matrix;

pub fn calculate_distanz_matrix(adjazenz_matrix: &Vec<Vec<usize>>) -> Vec<Vec<usize>> {
    let mut distanz_matrix: Vec<Vec<usize>> = vec![vec![]; adjazenz_matrix.len()];
    let mut potenz_matrix = clone(adjazenz_matrix);

    for k in 1..=adjazenz_matrix.len() {
        potenz_matrix = mult(&potenz_matrix, adjazenz_matrix);
        for i in 0..adjazenz_matrix.len() {
            for j in 0..adjazenz_matrix.len() {
                if k != 1 {
                    if potenz_matrix[i][j] != 0 && distanz_matrix[i][j] == usize::MAX {
                        distanz_matrix[i][j] = k;
                    }
                    continue;
                }
                if i == j {
                    distanz_matrix[i].push(0);
                } else if adjazenz_matrix[i][j] == 1 {
                    distanz_matrix[i].push(1);
                } else {
                    distanz_matrix[i].push(usize::MAX);
                }
            }
        }
    }
    distanz_matrix
}

pub fn calculate_weg_matrix(adjazenz_matrix: &Vec<Vec<usize>>) -> Vec<Vec<usize>> {
    let mut weg_matrix: Vec<Vec<usize>> = vec![vec![]; adjazenz_matrix.len()];
    let mut potenz_matrix = clone(adjazenz_matrix);

    for k in 1..=adjazenz_matrix.len() {
        potenz_matrix = mult(&potenz_matrix, adjazenz_matrix);
        for i in 0..adjazenz_matrix.len() {
            for j in 0..adjazenz_matrix.len() {
                if k != 1 {
                    if potenz_matrix[i][j] != 0 {
                        weg_matrix[i][j] = 1;
                    }
                    continue;
                }
                if i == j || adjazenz_matrix[i][j] == 1 {
                    weg_matrix[i].push(1);
                } else {
                    weg_matrix[i].push(0);
                }
            }
        }
    }
    weg_matrix
}

pub fn calculate_exzentrizitaeten(distanz_matrix: Vec<Vec<usize>>) -> Vec<usize> {
    let mut exzentrizitaeten = vec![];
    let mut exzentrizitaet: usize;

    for vector in distanz_matrix {
        exzentrizitaet = 0;

        for value in vector {
            if value == usize::MAX {
                continue;
            }
            exzentrizitaet = exzentrizitaet.max(value);
        }
        exzentrizitaeten.push(exzentrizitaet);
    }
    exzentrizitaeten
}

pub fn calculate_properties(exzentrizitaeten: &[usize]) -> (usize, usize, Vec<usize>, bool) {
    let mut radius = usize::MAX;
    let mut diameter = 0;
    let mut centre = vec![];
    let mut connected = true;

    for (index, value) in exzentrizitaeten.iter().enumerate() {
        if value > &diameter {
            diameter = *value;
        }
        if value == &radius {
            centre.push(index + 1);
        }
        if value < &radius {
            radius = *value;
            centre.clear();
            centre.push(index + 1);
        }
        if value == &0 {
            connected = false;
        }
    }
    (radius, diameter, centre, connected)
}

pub fn find_components(weg_matrix: Vec<Vec<usize>>) -> Vec<Vec<usize>> {
    let mut components: Vec<Vec<usize>> = vec![];
    let mut component: Vec<usize>;

    for array in weg_matrix {
        component = vec![];
        for (index, value) in array.iter().enumerate() {
            if value == &1 {
                component.push(index + 1);
            }
        }
        if !components.contains(&component) {
            components.push(component);
        }
    }
    components
}

pub fn find_bridges(adjazenz_matrix: &Vec<Vec<usize>>) -> Vec<Vec<usize>> {
    let size = adjazenz_matrix.len();
    let mut bridges: Vec<Vec<usize>> = vec![];
    let mut visited = vec![false; size];
    let mut discovery_time = vec![0; size];
    let mut low_time = vec![0; size];
    let time = 0;

    for i in 0..size {
        if !visited[i] {
            dfs_bridges(
                &mut bridges,
                adjazenz_matrix,
                &mut visited,
                &mut discovery_time,
                &mut low_time,
                time,
                i,
                usize::MAX,
            );
        }
    }

    bridges
}

pub fn find_articulations(adjazenz_matrix: &Vec<Vec<usize>>) -> Vec<usize> {
    let size = adjazenz_matrix.len();
    let mut ariculations = vec![];
    let mut is_articulation = vec![false; size];
    let mut visited = vec![false; size];
    let mut discovery_time = vec![0; size];
    let mut low_time = vec![0; size];
    let time = 0;

    for i in 0..size {
        if !visited[i] {
            dfs_articulations(
                &mut ariculations,
                &mut is_articulation,
                adjazenz_matrix,
                &mut visited,
                &mut discovery_time,
                &mut low_time,
                time,
                i,
                usize::MAX,
            );
        }
    }

    ariculations
}
