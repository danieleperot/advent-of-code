use std::fs;
use std::io::Error;

mod solution;

fn main() -> Result<(), Error> {
    let input = fs::read_to_string("input.txt")?;
    println!("Answer is: {}", solution::find_match_for_key(&input, 6));

    Ok(())
}