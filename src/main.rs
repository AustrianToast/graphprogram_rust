use crate::graph::{*, matrix::{read_csv, show}};

pub mod graph;

pub fn main() {
    let file_name = "24n.csv";
    let mut adjazenz_matrix = read_csv(file_name);
    //let mut adjazenz_matrix = fill_with_random(45); // with this many verteces, it runs in about 10.2 seconds (2023-06-02 14:39)
    let distanz_matrix = calculate_distanz_matrix(&adjazenz_matrix);
    let weg_matrix = calculate_weg_matrix(&adjazenz_matrix);

    println!("adjazen matrix:");
    show(&adjazenz_matrix);
    println!("\ndistanz matrix:");
    show(&distanz_matrix);
    println!("\nweg matrix:");
    show(&weg_matrix);

    let exzentrizitaeten = calculate_exzentrizitaeten(distanz_matrix);
    let properties = calculate_properties(&exzentrizitaeten);

    if properties.3 {
        println!("\nexzentrizitäten: {exzentrizitaeten:?}");
        println!(
            "radius: {}\ndiameter: {}\ncentre: {:?}",
            properties.0, properties.1, properties.2
        );
    } else {
        println!("\nexzentrizitäten: not connected");
        println!("radius/diameter/centre: not connected");
    }

    let components = find_components(weg_matrix);
    println!("components: {components:?}");

    let result = find_articulations_and_bridges(&mut adjazenz_matrix, &components);
    println!("bridges: {:?}", result.1);
    println!("articulations: {:?}", result.0);
}
