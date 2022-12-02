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

fn sum_calories<'a>(calories: &'a Vec<Vec<u64>>) -> Box<dyn Iterator<Item = u64> + 'a> {
    Box::new(calories.iter().map(|elf| elf.iter().sum::<u64>()))
}

pub fn part_one(input: &Vec<Vec<u64>>) -> u64 {
    sum_calories(input).max().unwrap_or(0)
}

pub fn part_two(input: &Vec<Vec<u64>>) -> u64 {
    // get sum
    let sum = sum_calories(input);

    // sort sum of elves' calories
    let mut sorted = sum.collect::<Vec<u64>>();
    sorted.sort();

    // get sum of top 3 calories
    sorted[sorted.len() - 3..].iter().sum::<u64>()
}
