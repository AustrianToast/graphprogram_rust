pub mod graph;

#[cfg(test)]
mod tests {
    use crate::graph::{matrix::*, *};

    #[test]
    fn graph() {
        let adjazenz_matrix = read_csv("/home/rene/projects/Java/graphprogram/csv/art-brck.csv");
        let distanz_matrix = calculate_distanz_matrix(&adjazenz_matrix);
        let weg_matrix = calculate_weg_matrix(&adjazenz_matrix);

        assert_eq!(
            adjazenz_matrix,
            vec![
                vec![0, 0, 1, 1, 0],
                vec![0, 0, 1, 1, 0],
                vec![1, 1, 0, 1, 0],
                vec![1, 1, 1, 0, 1],
                vec![0, 0, 0, 1, 0]
            ]
        );
        assert_eq!(
            distanz_matrix,
            vec![
                vec![0, 2, 1, 1, 2],
                vec![2, 0, 1, 1, 2],
                vec![1, 1, 0, 1, 2],
                vec![1, 1, 1, 0, 1],
                vec![2, 2, 2, 1, 0]
            ]
        );
        assert_eq!(
            weg_matrix,
            vec![
                vec![1, 1, 1, 1, 1],
                vec![1, 1, 1, 1, 1],
                vec![1, 1, 1, 1, 1],
                vec![1, 1, 1, 1, 1],
                vec![1, 1, 1, 1, 1]
            ]
        );

        let exzentrizitaeten = calculate_exzentrizitaeten(distanz_matrix);
        let properties = calculate_properties(&exzentrizitaeten);
        let components = find_components(weg_matrix);
        let bridges = find_bridges(&adjazenz_matrix);
        let articulations = find_articulations(&adjazenz_matrix);

        assert_eq!(exzentrizitaeten, vec![2, 2, 2, 1, 2]);
        assert_eq!(properties.0, 1);
        assert_eq!(properties.1, 2);
        assert_eq!(properties.2, vec![4]);
        assert_eq!(properties.3, true);
        assert_eq!(components, vec![vec![1, 2, 3, 4, 5]]);
        assert_eq!(bridges, vec![vec![4, 5]]);
        assert_eq!(articulations, vec![4]);
    }

    #[test]
    fn matrix() {
        let adjazenz_matrix = read_csv("art-brck.csv");

        assert_eq!(
            adjazenz_matrix,
            vec![
                vec![0, 0, 1, 1, 0],
                vec![0, 0, 1, 1, 0],
                vec![1, 1, 0, 1, 0],
                vec![1, 1, 1, 0, 1],
                vec![0, 0, 0, 1, 0]
            ]
        );
        assert_eq!(
            mult(&clone(&adjazenz_matrix), &adjazenz_matrix),
            vec![
                vec![2, 2, 1, 1, 1],
                vec![2, 2, 1, 1, 1],
                vec![1, 1, 3, 2, 1],
                vec![1, 1, 2, 4, 0],
                vec![1, 1, 1, 0, 1]
            ]
        );
    }
}
