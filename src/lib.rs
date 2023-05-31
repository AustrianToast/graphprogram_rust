pub mod graph;

#[cfg(test)]
mod tests {
    use crate::graph::{self, matrix::*};

    #[test]
    fn graph() {
        let mut adjazenz_matrix: Vec<Vec<usize>> = graph::matrix::read_csv();
        let distanz_matrix: Vec<Vec<usize>> = graph::calculate_distanz_matrix(&adjazenz_matrix);
        let weg_matrix: Vec<Vec<usize>> = graph::calculate_weg_matrix(&adjazenz_matrix);
        let exzentrizitaeten = graph::calculate_exzentrizitaeten(&distanz_matrix);
        let properties = graph::calculate_properties(&exzentrizitaeten);
        let components = graph::find_components(&weg_matrix);

        assert_eq!(adjazenz_matrix,  vec![
            vec![0, 0, 1, 1, 0],
            vec![0, 0, 1, 1, 0],
            vec![1, 1, 0, 1, 0],
            vec![1, 1, 1, 0, 1],
            vec![0, 0, 0, 1, 0]
        ]);
        assert_eq!(distanz_matrix, vec![
            vec![0, 2, 1, 1, 2],
            vec![2, 0, 1, 1, 2],
            vec![1, 1, 0, 1, 2],
            vec![1, 1, 1, 0, 1],
            vec![2, 2, 2, 1, 0]
        ]);
        assert_eq!(weg_matrix, vec![
            vec![1, 1, 1, 1, 1],
            vec![1, 1, 1, 1, 1],
            vec![1, 1, 1, 1, 1],
            vec![1, 1, 1, 1, 1],
            vec![1, 1, 1, 1, 1]
        ]);
        assert_eq!(exzentrizitaeten, vec![2, 2, 2, 1, 2]);
        assert_eq!(properties.0, 1);
        assert_eq!(properties.1, 2);
        assert_eq!(properties.2, vec![4]);
        assert_eq!(properties.3, true);
        assert_eq!(components, vec![vec![1, 2, 3, 4, 5]]);
        assert_eq!(graph::find_bridges(&mut adjazenz_matrix, &components), vec![vec![4, 5]]);
        assert_eq!(graph::find_articulations(&adjazenz_matrix, &components), vec![4]);
    }

    #[test]
    fn matrix() {
        let adjazenz_matrix: Vec<Vec<usize>> = read_csv();

        assert_eq!(adjazenz_matrix, vec![
            vec![0, 0, 1, 1, 0],
            vec![0, 0, 1, 1, 0],
            vec![1, 1, 0, 1, 0],
            vec![1, 1, 1, 0, 1],
            vec![0, 0, 0, 1, 0]
        ]);
        assert_eq!(mult(&adjazenz_matrix, &adjazenz_matrix), vec![
            vec![2, 2, 1, 1, 1],
            vec![2, 2, 1, 1, 1],
            vec![1, 1, 3, 2, 1],
            vec![1, 1, 2, 4, 0],
            vec![1, 1, 1, 0, 1]
        ]);
    }
}