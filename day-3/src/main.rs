use shared::input::get_input_from_file;
use std::{error::Error, ops::Index};

const PRIORITY: &str = "_abcdefghijklmnopqrstuvwxyzABCDEFGHIJKLMNOPQRSTUVWXYZ";

fn main() -> Result<(), Box<dyn Error>> {
    let input = get_input_from_file("src/input.txt");

    println!("Part one: {}", part_one(&input));
    println!("Part two: {}", part_two(&input));

    Ok(())
}

fn part_one(input: &str) -> String {
    let mut sum: Vec<usize> = vec![];
    // iterate over each line
    for (line_index, line) in input.lines().into_iter().enumerate() {
        // split line in half
        let (left, right) = line.split_at(line.len() / 2);
        // get matching characters in left n right
        left.chars()
            .map(|c| {
                // right branch contains left char
                if right.contains(c) {
                    // get index of left char in priority
                    let index_of_char = PRIORITY.find(c).unwrap();
                    if sum.len() == 0 || sum.len() == line_index {
                        sum.push(index_of_char); // push index to sum
                    }
                    Some(c);
                }
            })
            .count();
    }

    sum.iter().sum::<usize>().to_string()
}

fn part_two(input: &str) -> String {
    let mut sum_of_item_types: Vec<usize> = vec![];

    // loop every 3 lines in input
    for line in input.lines().into_iter().step_by(3) {
        let input_iterator = input.lines().into_iter();
        // line index
        let line_index = input_iterator.to_owned().position(|l| l == line).unwrap();

        let first_elf = line;
        let second_elf = input_iterator.to_owned().nth(line_index + 1).unwrap();
        let third_elf = input_iterator.to_owned().nth(line_index + 2).unwrap();

        let mut temp_char = None;

        first_elf
            .chars()
            .map(|c| {
                if third_elf.contains(c) && second_elf.contains(c) {
                    let index_of_char = PRIORITY.find(c).unwrap();

                    if !(temp_char == Some(c)) {
                        temp_char = Some(c);
                        sum_of_item_types.push(index_of_char);
                        println!("c: {}, index: {}", c, index_of_char);
                    }
                    Err(())
                } else {
                    Ok(())
                }
            })
            .count();
    }

    sum_of_item_types.iter().sum::<usize>().to_string()
}
