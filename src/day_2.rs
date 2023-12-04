use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;
use std::collections::HashMap;

pub fn main() {
    println!("DAY 2:");
    part_1();
    println!("------------");
}


fn part_1() {
    if let Ok(lines) = read_lines("src/inputs/day_2") {

        let max_in_bag = HashMap::from([
            ("red", 12),
            ("green", 13),
            ("blue", 14)
        ]);

        let mut valid_games: Vec<i32> = (1..=100).collect();
        let mut invalid_games = Vec::new();

        let mut i = 1;
        for raw_line in lines.flatten() {
            let mut split = raw_line.split(": ");
            let _game_no = split.next();
            let handfuls = split.next().expect("Why is there no handfuls").split("; ");

            for handful in handfuls {
                let colours = handful.split(", ");

                for colour in colours {
                    let mut count_and_colour = colour.split(' ');
                    let mut count = 0;
                    if let Some(number) = count_and_colour.next() {
                        count = number.parse().unwrap();
                    }
                    if let Some(current_colour) = count_and_colour.next() {
                        if max_in_bag.get(current_colour) < Some(&count) {
                            invalid_games.push(i);
                        }
                    }
                }
            }
            i += 1;
        }
        valid_games.retain(|&game| !invalid_games.contains(&game));
        println!("Part 1 Answer: {}", valid_games.iter().sum::<i32>());
    }
}


fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where P: AsRef<Path>, {
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}
