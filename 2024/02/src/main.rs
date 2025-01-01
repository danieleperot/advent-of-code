use crate::solution::count_safe_reports;
use std::fs;
use std::io::Error;

mod solution;

fn main() -> Result<(), Error> {
    let input = fs::read_to_string("input.txt")?;
    println!("Safe records: {}", count_safe_reports(input));

    Ok(())
}
