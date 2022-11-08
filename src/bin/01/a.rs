use std::error::Error;
use std::fs;

fn main() -> Result<(), Box<dyn Error>> {
    let input = fs::read_to_string("src/bin/01/input.txt")?;
    print!("{}\n", input);

    Ok(())
}
