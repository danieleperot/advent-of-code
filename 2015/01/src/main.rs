use std::fs;
use std::io::Error;

mod solution;

fn main() -> Result<(), Error> {
    let input = fs::read_to_string("input.txt")?;
    println!("The answer is {}", solution::solve(&input, solution::StopAt::Floor(-1)));

    Ok(())
}
