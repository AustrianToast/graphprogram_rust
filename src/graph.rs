use self::matrix::clone;

pub mod matrix;

pub fn calculate_distanz_matrix(adjazenz_matrix: &Vec<Vec<u8>>) -> Vec<Vec<u8>> {
    let mut distanz_matrix: Vec<Vec<u8>> = vec![vec![]; adjazenz_matrix.len()];
    let mut potenz_matrix = clone(adjazenz_matrix);

    for k in 1..(adjazenz_matrix.len() + 1) {
        potenz_matrix = matrix::mult(&potenz_matrix, adjazenz_matrix);
        for i in 0..adjazenz_matrix.len() {
            for j in 0..adjazenz_matrix.len() {
                if k != 1 {
                    if potenz_matrix[i][j] != 0 && distanz_matrix[i][j] == u8::MAX {
                        distanz_matrix[i][j] = u8::try_from(k).unwrap();
                    }
                    continue;
                }
                if i == j {
                    distanz_matrix[i].push(0);
                } else if adjazenz_matrix[i][j] == 1 {
                    distanz_matrix[i].push(1);
                } else {
                    distanz_matrix[i].push(u8::MAX);
                }
            }
        }
    }
    distanz_matrix
}

pub fn calculate_weg_matrix(adjazenz_matrix: &Vec<Vec<u8>>) -> Vec<Vec<u8>> {
    let mut weg_matrix: Vec<Vec<u8>> = vec![vec![]; adjazenz_matrix.len()];
    let mut potenz_matrix = clone(adjazenz_matrix);

    for k in 1..(adjazenz_matrix.len() + 1) {
        potenz_matrix = matrix::mult(&potenz_matrix, adjazenz_matrix);
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

pub fn calculate_exzentrizitaeten(distanz_matrix: Vec<Vec<u8>>) -> Vec<u8> {
    let mut exzentrizitaeten: Vec<u8> = vec![];
    let mut exzentrizitaet: u8;

    for vector in distanz_matrix {
        exzentrizitaet = 0;

        for value in vector {
            if value == u8::MAX {
                continue;
            }
            exzentrizitaet = exzentrizitaet.max(value);
        }
        exzentrizitaeten.push(exzentrizitaet);
    }
    exzentrizitaeten
}

pub fn calculate_properties(exzentrizitaeten: &[u8]) -> (u8, u8, Vec<u8>, bool) {
    let mut radius = u8::MAX;
    let mut diameter: u8 = 0;
    let mut centre: Vec<u8> = vec![];
    let mut connected: bool = true;

    for (index, value) in exzentrizitaeten.iter().enumerate() {
        if value > &diameter {
            diameter = *value;
        }
        if value == &radius {
            centre.push(u8::try_from(index + 1).unwrap());
        }
        if value < &radius {
            radius = *value;
            centre.clear();
            centre.push(u8::try_from(index + 1).unwrap());
        }
        if value == &0 {
            connected = false;
        }
    }
    (radius, diameter, centre, connected)
}

pub fn find_components(weg_matrix: Vec<Vec<u8>>) -> Vec<Vec<u8>> {
    let mut components: Vec<Vec<u8>> = vec![];
    let mut component: Vec<u8>;

    for array in weg_matrix {
        component = vec![];
        for (index, value) in array.iter().enumerate() {
            if value == &1 {
                component.push(u8::try_from(index + 1).unwrap());
            }
        }
        if !components.contains(&component) {
            components.push(component);
        }
    }
    components
}

pub fn find_articulations_and_bridges(
    adjazenz_matrix: &mut Vec<Vec<u8>>,
    components: &Vec<Vec<u8>>,
) -> (Vec<u8>, Vec<Vec<u8>>) {
    let mut bridges: Vec<Vec<u8>> = vec![];
    let mut articulations: Vec<u8> = vec![];
    let mut temp_matrix = adjazenz_matrix.clone();

    for n in 0..temp_matrix.len() {
        for i in 0..temp_matrix.len() {
            for j in 0..temp_matrix.len() {
                temp_matrix[i][n] = 0;
                temp_matrix[n][j] = 0;

                if n != 0 {
                    continue;
                }
                let bridge = vec![
                    u8::try_from(usize::min(i + 1, j + 1)).unwrap(),
                    u8::try_from(usize::max(i + 1, j + 1)).unwrap(),
                ];
                let prev_value = adjazenz_matrix[i][j];
                adjazenz_matrix[i][j] = 0;
                adjazenz_matrix[j][i] = 0;

                if find_components(calculate_weg_matrix(adjazenz_matrix)).len() > components.len()
                    && !bridges.contains(&bridge)
                {
                    bridges.push(bridge);
                }
                adjazenz_matrix[i][j] = prev_value;
                adjazenz_matrix[j][i] = prev_value;
            }
        }

        if find_components(calculate_weg_matrix(&temp_matrix)).len() > (components.len() + 1) {
            articulations.push(u8::try_from(n + 1).unwrap());
        }
        temp_matrix = adjazenz_matrix.clone();
    }

    (articulations, bridges)
}
