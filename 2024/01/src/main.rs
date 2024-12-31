use std::fs;
use std::io::Error;

mod difference;
mod shared;
mod similarity;

fn main() -> Result<(), Error> {
    let input = fs::read_to_string("input.txt")?;
    println!(
        "Difference is: {}",
        difference::distance_from_string(&input)
    );
    println!(
        "Similarity is: {}",
        similarity::similarity_from_string(&input)
    );

    Ok(())
}
