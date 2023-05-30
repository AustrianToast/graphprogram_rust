pub mod matrix;

pub fn tests(arg: &str) {
    match arg {
        "matrix" => matrix::test(),
        "graph" => test(),
        &_ => println!("not a valid option"),
    }
}

pub fn test() {
    let adjazenz_matrix: Vec<Vec<usize>> = matrix::read_csv();
    let distanz_matrix: Vec<Vec<usize>> = calculate_distanz_matrix(&adjazenz_matrix);
    let weg_matrix: Vec<Vec<usize>> = calculate_weg_matrix(&adjazenz_matrix);

    println!("adjazen matrix:");
    matrix::show(&adjazenz_matrix);
    println!("\ndistanz matrix:");
    matrix::show(&distanz_matrix);
    println!("\nweg matrix:");
    matrix::show(&weg_matrix);

    let exzentrizitaeten = calculate_exzentrizitaeten(&distanz_matrix);
    let mut connected: bool = true;

    if exzentrizitaeten.contains(&0) {
        connected = false;
    }

    println!("\n{:?}", exzentrizitaeten);
    println!("is the graph connected: {connected}");

    let properties = calculate_properties(&exzentrizitaeten);
    let radius: usize = properties.0;
    let diameter: usize = properties.1;
    let centre: Vec<usize> = properties.2;

    println!("radius: {radius}\ndiameter: {diameter}\ncentre: {:?}", centre);
}

fn calculate_distanz_matrix(adjazenz_matrix: &Vec<Vec<usize>>) -> Vec<Vec<usize>> {
    #[allow(unused_assignments)]
    let mut distanz_matrix: Vec<Vec<usize>> = vec![];
    let mut potenz_matrix = adjazenz_matrix.clone();

    for i in 0..adjazenz_matrix.len() {
        distanz_matrix.push(vec![]);
        for j in 0..adjazenz_matrix.len() {
            if i == j {
                distanz_matrix[i].push(0)
            } else if adjazenz_matrix[i][j] == 1 {
                distanz_matrix[i].push(1);
            } else {
                distanz_matrix[i].push(usize::MAX);
            }
        }
    }

    for k in 2..adjazenz_matrix.len() {
        potenz_matrix = matrix::mult(&potenz_matrix, &adjazenz_matrix);
        for i in 0..adjazenz_matrix.len() {
            for j in 0..adjazenz_matrix.len() {
                if potenz_matrix[i][j] != 0 && distanz_matrix[i][j] == usize::MAX {
                    distanz_matrix[i][j] = k;
                }
            }
        }
    }
    distanz_matrix
}

fn calculate_weg_matrix(adjazenz_matrix: &Vec<Vec<usize>>) -> Vec<Vec<usize>> {
    #[allow(unused_assignments)]
    let mut weg_matrix: Vec<Vec<usize>> = vec![];
    let mut potenz_matrix = adjazenz_matrix.clone();

    for i in 0..adjazenz_matrix.len() {
        weg_matrix.push(vec![]);
        for j in 0..adjazenz_matrix.len() {
            if i == j {
                weg_matrix[i].push(1)
            } else if adjazenz_matrix[i][j] == 1 {
                weg_matrix[i].push(1);
            } else {
                weg_matrix[i].push(0);
            }
        }
    }

    for _k in 2..adjazenz_matrix.len() {
        potenz_matrix = matrix::mult(&potenz_matrix, &adjazenz_matrix);
        for i in 0..adjazenz_matrix.len() {
            for j in 0..adjazenz_matrix.len() {
                if potenz_matrix[i][j] != 0 {
                    weg_matrix[i][j] = 1;
                }
            }
        }
    }
    weg_matrix
}

fn calculate_exzentrizitaeten(distanz_matrix: &Vec<Vec<usize>>) -> Vec<usize> {
    let mut exzentrizitaeten: Vec<usize> = vec![];
    let mut exzentrizitaet: usize;

    for i in 0..distanz_matrix.len() {
        exzentrizitaet = 0;

        for j in 0..distanz_matrix.len() {
            if distanz_matrix[i][j] > exzentrizitaet && i != j {
                exzentrizitaet = distanz_matrix[i][j];
            }
        }
        exzentrizitaeten.push(exzentrizitaet);
    }
    exzentrizitaeten
}

fn calculate_properties(exzentrizitaeten: &Vec<usize>) -> (usize, usize, Vec<usize>) {
    let mut radius: usize = usize::MAX;
    let mut diameter: usize = 0;
    let mut centre: Vec<usize> = vec![];
    
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
    }

    let results: (usize, usize, Vec<usize>) = (radius, diameter, centre);
    results
}
