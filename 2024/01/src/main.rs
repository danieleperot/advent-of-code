use std::fs;
use std::io::Error;

mod solution;

fn main() -> Result<(), Error> {
    let input = fs::read_to_string("input.txt")?;
    println!("Answer is: {}", solution::distance_from_string(&input));

    Ok(())
}
