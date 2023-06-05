#[allow(unused_imports)]
use crate::graph::{
    matrix::{fill_with_random, read_csv, show},
    *,
};

pub mod graph;

pub fn main() {
    let file_name = "24n.csv";
    let adjazenz_matrix = read_csv(file_name);
    //let adjazenz_matrix = fill_with_random(320); // with 320 verteces, it runs in about 10 seconds on an Intel i5-10300H @4.3 GHz (2023-06-05 15:48)
    let distanz_matrix = calculate_distanz_matrix(&adjazenz_matrix);
    let weg_matrix = calculate_weg_matrix(&adjazenz_matrix);

    println!("adjazenz matrix:");
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

    println!("bridges: {:?}", find_bridges(&adjazenz_matrix));
    println!("articulations: {:?}", find_articulations(&adjazenz_matrix));
}
