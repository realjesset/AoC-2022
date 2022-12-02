use day_1::{parse_input, part_one, part_two};
use shared::input::get_input_from_file;
use std::error::Error;

fn main() -> Result<(), Box<dyn Error>> {
    // const FILE_PATH: &str = "src/input.txt";

    // let mut highest_calorie = 0;
    // let mut temporary_calorie = 0;

    // parse_input();
    let input = get_input_from_file("./src/input.txt");
    let parsed = parse_input(&input)?;
    // pass parsed to part_one
    println!("Part one: {}", part_one(&parsed));
    println!("Part two: {}", part_two(&parsed));

    Ok(())
    // println!("Part 1: {}", part_one(&input));

    // if let Ok(lines) = read_lines(FILE_PATH) {
    //     for line in lines {
    //         if let Ok(ip) = line {
    //             // check if line is empty, and compare temporary_calorie with highest_calorie
    //             if ip.is_empty() {
    //                 temporary_calorie = 0;
    //             } else {
    //                 // add calorie to temporary_calorie
    //                 temporary_calorie += ip.parse::<i32>().unwrap();
    //                 if temporary_calorie > highest_calorie {
    //                     highest_calorie = temporary_calorie;
    //                 }
    //             }
    //         }
    //     }
    // }

    // println!("Highest calorie: {}", highest_calorie);
}

// fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
// where
//     P: AsRef<Path>,
// {
//     let file = File::open(filename)?;
//     Ok(io::BufReader::new(file).lines())
// }
