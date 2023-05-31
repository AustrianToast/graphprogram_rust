pub mod graph;

#[cfg(test)]
mod tests {
    use crate::graph::{self, matrix::*};

    #[test]
    fn graph() {
        let mut adjazenz_matrix: Vec<Vec<usize>> = graph::matrix::read_csv();
    let distanz_matrix: Vec<Vec<usize>> = graph::calculate_distanz_matrix(&adjazenz_matrix);
    let weg_matrix: Vec<Vec<usize>> = graph::calculate_weg_matrix(&adjazenz_matrix);
    let expected_adjazenz_matrix: Vec<Vec<usize>> = vec![
        vec![0, 0, 1, 1, 0],
        vec![0, 0, 1, 1, 0],
        vec![1, 1, 0, 1, 0],
        vec![1, 1, 1, 0, 1],
        vec![0, 0, 0, 1, 0]
    ];
    let expected_distanz_matrix: Vec<Vec<usize>> = vec![
        vec![0, 2, 1, 1, 2],
        vec![2, 0, 1, 1, 2],
        vec![1, 1, 0, 1, 2],
        vec![1, 1, 1, 0, 1],
        vec![2, 2, 2, 1, 0]
    ];
    let expected_weg_matrix: Vec<Vec<usize>> = vec![
        vec![1, 1, 1, 1, 1],
        vec![1, 1, 1, 1, 1],
        vec![1, 1, 1, 1, 1],
        vec![1, 1, 1, 1, 1],
        vec![1, 1, 1, 1, 1]
    ];

    assert_eq!(adjazenz_matrix,  expected_adjazenz_matrix);
    assert_eq!(distanz_matrix, expected_distanz_matrix);
    assert_eq!(weg_matrix, expected_weg_matrix);

    let exzentrizitaeten = graph::calculate_exzentrizitaeten(&distanz_matrix);
    let mut connected: bool = true;

    if exzentrizitaeten.contains(&0) {
        connected = false;
    }
    assert_eq!(exzentrizitaeten, vec![2, 2, 2, 1, 2]);
    assert_eq!(connected, true);

    let properties = graph::calculate_properties(&exzentrizitaeten);
    let radius: usize = properties.0;
    let diameter: usize = properties.1;
    let centre: Vec<usize> = properties.2;

    assert_eq!(radius, 1);
    assert_eq!(diameter, 2);
    assert_eq!(centre, vec![4]);

    let components = graph::find_components(&weg_matrix);
    let bridges = graph::find_bridges(&mut adjazenz_matrix, &components);
    let articulations = graph::find_articulations(&adjazenz_matrix, &components);

    assert_eq!(components, vec![vec![1, 2, 3, 4, 5]]);
    assert_eq!(bridges, vec![vec![4, 5]]);
    assert_eq!(articulations, vec![4]);
    }

    #[test]
    fn matrix() {
        let matrix1 = read_csv();
        let expected_matrix: Vec<Vec<usize>> = vec![
            vec![0, 0, 1, 1, 0],
            vec![0, 0, 1, 1, 0],
            vec![1, 1, 0, 1, 0],
            vec![1, 1, 1, 0, 1],
            vec![0, 0, 0, 1, 0]
        ];
        let matrix2: Vec<Vec<usize>> = matrix1.clone();

        assert_eq!(matrix1, expected_matrix);

        let mut product: Vec<Vec<usize>> = mult(&matrix1, &matrix2);
        let mut expected_product: Vec<Vec<usize>> = vec![
            vec![2, 2, 1, 1, 1],
            vec![2, 2, 1, 1, 1],
            vec![1, 1, 3, 2, 1],
            vec![1, 1, 2, 4, 0],
            vec![1, 1, 1, 0, 1]
        ];

        assert_eq!(product, expected_product);

        product = mult(&product, &matrix1);
        expected_product = vec![
            vec![2, 2, 5, 6, 1],
            vec![2, 2, 5, 6, 1],
            vec![5, 5, 4, 6, 2],
            vec![6, 6, 6, 4, 4],
            vec![1, 1, 2, 4, 0]
        ];

        assert_eq!(product, expected_product)
    }
}