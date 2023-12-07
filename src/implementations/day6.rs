use crate::utility::generic_error::GenericResult;

fn count_ways_to_beat(time: f64, distance_to_beat: f64) -> usize {
    // Classic quadratic equation
    let right_part = (time * time - 4.0 * distance_to_beat).sqrt();
    let lower_bound = (time - right_part) / 2.0;
    let upper_bound = (time + right_part) / 2.0;

    // Count integers between the two floats
    (upper_bound.ceil() - lower_bound.floor() - 1.0) as usize
}

fn part_1(input_path: &str) -> GenericResult<usize> {
    let file_contents = std::fs::read_to_string(input_path)?;

    let mut result = 1;

    let split_str: Vec<&str> = file_contents.split_whitespace().collect();

    let num_races = (split_str.len() / 2) - 1;

    for race_num in 0..num_races {
        let time: f64 = split_str[race_num + 1].parse()?;
        let distance_to_beat: f64 = split_str[race_num + num_races + 2].parse()?;
        result *= count_ways_to_beat(time, distance_to_beat);
    }

    Ok(result)
}

fn part_2(input_path: &str) -> GenericResult<usize> {
    let file_contents = std::fs::read_to_string(input_path)?;

    let split_str: Vec<&str> = file_contents.split_whitespace().collect();
    let num_races = (split_str.len() / 2) - 1;
    let time_str = (&split_str[1..(num_races + 1)]).concat();
    let distance_str = (&split_str[(num_races + 2)..(2 * num_races + 2)]).concat();

    let time: f64 = time_str.parse()?;
    let distance: f64 = distance_str.parse()?;

    Ok(count_ways_to_beat(time, distance))
}

#[test]
pub fn run_test_1() -> GenericResult<()> {
    assert_eq!(part_1("test_data/day6/example.txt")?, 288);
    Ok(())
}

#[test]
pub fn run_test_2() -> GenericResult<()> {
    assert_eq!(part_2("test_data/day6/example.txt")?, 71503);
    Ok(())
}

pub fn run(input_path: &String) -> GenericResult<()> {
    println!("Part one result: {}", part_1(&input_path)?);
    println!("Part two result: {}", part_2(&input_path)?);
    Ok(())
}
