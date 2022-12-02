use std::error::Error;

pub fn parse_input(input: &str) -> Result<Vec<Vec<u64>>, Box<dyn Error>> {
    // remove bottom and top whitespace if exists
    let input = input.trim_end();

    // convert to total calories
    let mut elves_calories = vec![];
    let mut elf_counter = 0;

    for line in input.lines() {
        let line = line.trim_end(); // to remove spaces from the digits

        // check if line or parsed is empty array
        if line.is_empty() || elves_calories.is_empty() {
            elf_counter = elves_calories.len();
            elves_calories.push(vec![]);

            // skip if line is empty, which can happen in first iteration
            if line.is_empty() {
                continue;
            }
        }

        elves_calories[elf_counter].push(line.parse()?);
    }

    // return if there is no error on calories
    Ok(elves_calories)
}

pub fn part_one(input: &Vec<Vec<u64>>) -> u64 {
    input
        .iter()
        .map(|elf| elf.iter().sum::<u64>())
        .max()
        .unwrap_or(0)
}

pub fn part_two(input: &Vec<Vec<u64>>) -> u64 {
    let mut sorted = vec![];

    // sum the input and push to top_three_elves
    for elf in input {
        sorted.push(elf.iter().sum::<u64>());
    }

    sorted.sort();

    sorted[sorted.len() - 3..].iter().sum()
}
