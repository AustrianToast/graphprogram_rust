pub mod graph;

pub fn main() {
    let file_name = String::from("50n.csv");

    let mut adjazenz_matrix: Vec<Vec<usize>> = graph::matrix::read_csv(file_name);
    let distanz_matrix: Vec<Vec<usize>> = graph::calculate_distanz_matrix(&adjazenz_matrix);
    let weg_matrix: Vec<Vec<usize>> = graph::calculate_weg_matrix(&adjazenz_matrix);

    println!("adjazen matrix:");
    graph::matrix::show(&adjazenz_matrix);
    println!("\ndistanz matrix:");
    graph::matrix::show(&distanz_matrix);
    println!("\nweg matrix:");
    graph::matrix::show(&weg_matrix);

    let exzentrizitaeten = graph::calculate_exzentrizitaeten(&distanz_matrix);
    let properties = graph::calculate_properties(&exzentrizitaeten);

    if properties.3 {
        println!("\nexzentrizitäten: {:?}", exzentrizitaeten);
        println!("radius: {}\ndiameter: {}\ncentre: {:?}", properties.0, properties.1, properties.2);
    } else {
        println!("\nexzentrizitäten: not connected");
        println!("radius/diameter/centre: not connected");
    }

    let components: Vec<Vec<usize>> = graph::find_components(&weg_matrix);
    println!("components: {:?}", components);
    println!("bridges: {:?}", graph::find_bridges(&mut adjazenz_matrix, &components));
    println!("articulations: {:?}", graph::find_articulations(&adjazenz_matrix, &components));
}
