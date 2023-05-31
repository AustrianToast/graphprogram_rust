pub mod graph;

pub fn main() {
    tests("graph")
}

pub fn tests(arg: &str) {
    match arg {
        "graph" => graph::test(),
        "matrix" => graph::matrix::test(),
        &_ => println!("{arg} is not a valid option"),
    }
}