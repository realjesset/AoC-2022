use day_2::{parse_input, part_one, part_two};
use shared::input::get_input_from_file;
use std::error::Error;

fn main() -> Result<(), Box<dyn Error>> {
    let input = get_input_from_file("src/input.txt");
    let games = parse_input(&input);

    println!("Part one: {}", part_one(&games));
    println!("Part two: {}", part_two(&games));

    Ok(())
}
