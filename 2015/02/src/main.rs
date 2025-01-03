use std::fs;
use std::io::Error;

mod ribbon;
mod shared;
mod wrap;

fn main() -> Result<(), Error> {
    let input = fs::read_to_string("input.txt")?;
    println!("Wrapping paper: {} square feet", wrap::wrap_boxes(&input));
    println!("Ribbon: {} feet", ribbon::ribbon_for_all_boxes(&input));

    Ok(())
}
