use std::fs;
use std::io::Error;

mod wrap;

fn main() -> Result<(), Error> {
    let input = fs::read_to_string("input.txt")?;
    println!("The answer is {}", wrap::wrap_boxes(&input));

    Ok(())
}