use std::fs;
use std::io::Error;

mod solution;

fn main() -> Result<(), Error> {
    let input = fs::read_to_string("input.txt")?;
    println!("Answer is: {}", solution::visited_houses(&input, 2).len());

    Ok(())
}
