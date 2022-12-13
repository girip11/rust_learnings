// https://pola-rs.github.io/polars-book/user-guide/notebooks/introduction_polars-rs.html
use polars::prelude::*;

fn read_tracking_data() -> (usize, usize) {
    let _file_path = "/Users/girishpasupathy/girip_code_repos/github/rust_learnings/tryouts/\
                      data_analysis_polars/data/133020_tracking.csv";
    let df = CsvReader::from_path(_file_path).unwrap().finish().unwrap();
    df.shape()
}
fn main() {
    println!("Starting to read the file");
    use std::time::Instant;
    let now = Instant::now();
    let shape = read_tracking_data();
    let elapsed = now.elapsed();
    println!("Elapsed: {elapsed:.2?}");
    println!("{shape:?}");
}
