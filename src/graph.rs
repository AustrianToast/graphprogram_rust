pub mod matrix;

pub fn calculate_distanz_matrix(adjazenz_matrix: &Vec<Vec<usize>>) -> Vec<Vec<usize>> {
    let mut distanz_matrix: Vec<Vec<usize>> = vec![];
    let mut potenz_matrix = adjazenz_matrix.clone();

    for k in 1..adjazenz_matrix.len() {
        potenz_matrix = matrix::mult(&potenz_matrix, &adjazenz_matrix);
        for i in 0..adjazenz_matrix.len() {
            if k == 1 {
                distanz_matrix.push(vec![]);
            }
            for j in 0..adjazenz_matrix.len() {
                if k != 1 {
                    if potenz_matrix[i][j] != 0 && distanz_matrix[i][j] == usize::MAX {
                        distanz_matrix[i][j] = k;
                    }
                    continue;
                }
                if i == j {
                    distanz_matrix[i].push(0)
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
    let mut weg_matrix: Vec<Vec<usize>> = vec![];
    let mut potenz_matrix = adjazenz_matrix.clone();

    for k in 1..adjazenz_matrix.len() {
        potenz_matrix = matrix::mult(&potenz_matrix, &adjazenz_matrix);
        for i in 0..adjazenz_matrix.len() {
            if k == 1 {
                weg_matrix.push(vec![]);
            }
            for j in 0..adjazenz_matrix.len() {
                if k != 1 {
                    if potenz_matrix[i][j] != 0 {
                        weg_matrix[i][j] = 1;
                    }
                    continue;
                }
                if i == j {
                    weg_matrix[i].push(1)
                } else if adjazenz_matrix[i][j] == 1 {
                    weg_matrix[i].push(1);
                } else {
                    weg_matrix[i].push(0);
                }
            }
        }
    }
    weg_matrix
}

pub fn calculate_exzentrizitaeten(distanz_matrix: &Vec<Vec<usize>>) -> Vec<usize> {
    let mut exzentrizitaeten: Vec<usize> = vec![];
    let mut exzentrizitaet: usize;

    for i in 0..distanz_matrix.len() {
        exzentrizitaet = 0;

        for j in 0..distanz_matrix.len() {
            if i != j && distanz_matrix[i][j] > exzentrizitaet {
                exzentrizitaet = distanz_matrix[i][j];
            }
        }
        exzentrizitaeten.push(exzentrizitaet);
    }
    exzentrizitaeten
}

pub fn calculate_properties(exzentrizitaeten: &Vec<usize>) -> (usize, usize, Vec<usize>, bool) {
    let mut radius: usize = usize::MAX;
    let mut diameter: usize = 0;
    let mut centre: Vec<usize> = vec![];
    let mut connected: bool = true;
    
    for i in 0..exzentrizitaeten.len() {
        if exzentrizitaeten[i] > diameter {
            diameter = exzentrizitaeten[i];
        }
        if exzentrizitaeten[i] == radius {
            centre.push(i + 1);
        }
        if exzentrizitaeten[i] < radius {
            radius = exzentrizitaeten[i];
            centre.clear();
            centre.push(i + 1);
        }
        if exzentrizitaeten[i] == 0 {
            connected = false;
        }
    }
    (radius, diameter, centre, connected)
}

pub fn find_components(weg_matrix: &Vec<Vec<usize>>) -> Vec<Vec<usize>> {
    let mut components: Vec<Vec<usize>> = vec![];
    let mut component: Vec<usize>;
    
    for i in 0..weg_matrix.len() {
        component = vec![];

        for j in 0..weg_matrix.len() {
            if weg_matrix[i][j] == 1 {
                component.push(j + 1);
            }
        }
        if !components.contains(&component) {
            components.push(component);
        }
    }
    components
}

pub fn find_articulations_and_bridges(adjazenz_matrix: &mut Vec<Vec<usize>>, components: &Vec<Vec<usize>>) -> (Vec<usize>,Vec<Vec<usize>>) {
    let mut bridges: Vec<Vec<usize>> = vec![];
    let mut bridge: Vec<usize>;
    let mut prev_value: usize;
    let mut articulations: Vec<usize> = vec![];
    let mut new_components: Vec<Vec<usize>>;
    let mut temp_matrix: Vec<Vec<usize>> = adjazenz_matrix.clone();
    let mut weg_matrix: Vec<Vec<usize>>;
    let mut done: bool = false;

    for n in 0..temp_matrix.len() {
        for i in 0..temp_matrix.len() {
            for j in 0..temp_matrix.len() {
                temp_matrix[i][n] = 0;
                temp_matrix[n][j] = 0;

                if done || i == j {
                    continue;
                }
                bridge = vec![];
                bridge.push(usize::min(i + 1, j + 1));
                bridge.push(usize::max(i + 1, j + 1));
    
                prev_value = adjazenz_matrix[i][j];
                adjazenz_matrix[i][j] = 0;
                adjazenz_matrix[j][i] = 0;
                
                weg_matrix = calculate_weg_matrix(&adjazenz_matrix);
                new_components = find_components(&weg_matrix);
    
                if new_components.len() > components.len() && !bridges.contains(&bridge) {
                    bridges.push(bridge);
                }
                adjazenz_matrix[i][j] = prev_value;
                adjazenz_matrix[j][i] = prev_value;
            }
        }
        done = true;

        weg_matrix = calculate_weg_matrix(&temp_matrix);
        new_components = find_components(&weg_matrix);

        if new_components.len() > (components.len() + 1) {
            articulations.push(n + 1);
        }
        temp_matrix = adjazenz_matrix.clone();
    }

    (articulations, bridges)
}
