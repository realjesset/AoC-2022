use day_1::{parse_input, part_one, part_two};
use shared::input::get_input_from_file;
use std::error::Error;

fn main() -> Result<(), Box<dyn Error>> {
    let input = get_input_from_file("./src/input.txt");
    let parsed = parse_input(&input)?;

    println!("Part one: {}", part_one(&parsed));
    println!("Part two: {}", part_two(&parsed));

    Ok(())
}
