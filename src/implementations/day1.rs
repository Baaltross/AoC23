use std::io::{BufRead, BufReader};

fn part_1(input_path : &String) -> std::io::Result<()> {
    let file_handle = std::fs::File::open(input_path)?;
    let reader = BufReader::new(file_handle);

    let mut result = 0;

    for line in reader.lines() {
        let line = line?;

        let first = line.find(|c: char| (c >= '0') && (c <= '9'));
        let last = line.rfind(|c: char| (c >= '0') && (c <= '9'));

        let offset = '0' as u8;

        let value = match (first, last) {
            (Some(tens), Some(units)) => {
                10 * (line.as_bytes()[tens] - offset) + (line.as_bytes()[units] - offset)
            }
            _ => {
                println!("No digits found in line {}", line);
                0
            }
        };

        //println!("Line: {} - Value: {}", line, value);
        result += value as usize;
    }

    println!("Part one result: {}", result);

    Ok(())
}

fn find_value<Find, Compare>(
    line: &String,
    find_lambda: Find,
    compare_lambda: Compare,
) -> Option<usize>
where
    Find: Fn(&String, &str) -> Option<usize>,
    Compare: Fn(usize, usize) -> bool,
{
    let string_value_pairs = [
        ("0", 0),
        ("one", 1),
        ("1", 1),
        ("two", 2),
        ("2", 2),
        ("three", 3),
        ("3", 3),
        ("four", 4),
        ("4", 4),
        ("five", 5),
        ("5", 5),
        ("six", 6),
        ("6", 6),
        ("seven", 7),
        ("7", 7),
        ("eight", 8),
        ("8", 8),
        ("nine", 9),
        ("9", 9),
    ];

    let mut current_match: Option<(usize, u8)> = None;

    for string_value_pair in string_value_pairs {
        let found_string = find_lambda(&line, string_value_pair.0);
        match found_string {
            Some(index) => {
                if current_match == None || compare_lambda(index, current_match.unwrap().0) {
                    current_match = Some((index, string_value_pair.1));
                }
            }
            None => (),
        };
    }

    // Unwrap and return
    match current_match {
        Some((_, value)) => Some(value as usize),
        None => None,
    }
}

fn part_2(input_path : &String) -> std::io::Result<()> {
    let file_handle = std::fs::File::open(input_path)?;
    let reader = BufReader::new(file_handle);

    let mut result = 0;

    for line in reader.lines() {
        let line = line?;

        let first = find_value(
            &line,
            |line: &String, pattern: &str| line.find(pattern),
            |left: usize, right: usize| left < right,
        );
        let last = find_value(
            &line,
            |line: &String, pattern: &str| line.rfind(pattern),
            |left: usize, right: usize| left > right,
        );

        let value = match (first, last) {
            (Some(tens), Some(units)) => 10 * tens + units,
            _ => {
                println!("No digits found in line {}", line);
                0
            }
        };

        //println!("Line: {} - Value: {}", line, value);
        result += value as usize;
    }

    println!("Part two result: {}", result);

    Ok(())
}

pub fn run(input_path : &String) -> std::io::Result<()> {
    part_1(&input_path)?;
    part_2(&input_path)?;
    Ok(())
}
