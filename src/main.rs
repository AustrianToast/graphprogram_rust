pub mod graph;

pub fn main() {
    output();
}

// This is just for the pupose of visualising the output
fn output() {
    let mut adjazenz_matrix: Vec<Vec<usize>> = graph::matrix::read_csv();
    let distanz_matrix: Vec<Vec<usize>> = graph::calculate_distanz_matrix(&adjazenz_matrix);
    let weg_matrix: Vec<Vec<usize>> = graph::calculate_weg_matrix(&adjazenz_matrix);

    println!("adjazen matrix:");
    graph::matrix::show(&adjazenz_matrix);
    println!("\ndistanz matrix:");
    graph::matrix::show(&distanz_matrix);
    println!("\nweg matrix:");
    graph::matrix::show(&weg_matrix);

    let exzentrizitaeten = graph::calculate_exzentrizitaeten(&distanz_matrix);
    let mut connected: bool = true;

    if exzentrizitaeten.contains(&0) {
        connected = false;
    }
    println!("\nis the graph connected: {connected}");
    println!("exzentrizitäten: {:?}", exzentrizitaeten);

    let properties = graph::calculate_properties(&exzentrizitaeten);
    let radius: usize = properties.0;
    let diameter: usize = properties.1;
    let centre: Vec<usize> = properties.2;

    println!("radius: {radius}\ndiameter: {diameter}\ncentre: {:?}", centre);

    let components = graph::find_components(&weg_matrix);
    println!("components: {:?}", components);

    let bridges = graph::find_bridges(&mut adjazenz_matrix, &components);
    println!("bridges: {:?}", bridges);

    let articulations = graph::find_articulations(&adjazenz_matrix, &components);
    println!("articulations: {:?}", articulations);
}