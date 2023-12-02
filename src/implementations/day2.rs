use std::io::{BufRead, BufReader};
use std::str::FromStr;

struct Draw {
    r: usize,
    g: usize,
    b: usize,
}

impl Draw {
    fn power(&self) -> usize {
        self.r * self.g * self.b
    }
}

impl FromStr for Draw {
    type Err = Box<dyn std::error::Error>;

    fn from_str(input: &str) -> Result<Self, Self::Err> {
        let split: Vec<&str> = input.split(',').collect();

        let mut r = 0;
        let mut g = 0;
        let mut b = 0;

        for drawn_colour in split {
            let parsed = sscanf::sscanf!(drawn_colour.trim(), "{} {}", usize, str);

            match parsed {
                Ok((count, "red")) => r += count,
                Ok((count, "green")) => g += count,
                Ok((count, "blue")) => b += count,
                _ => return Err(format!("Failed to parse {}", drawn_colour))?,
            }
        }

        Ok(Draw { r, g, b })
    }
}

fn is_valid_draw_for_part_1(draw : &Draw) -> bool {
    draw.r <= 12 && draw.g <= 13 && draw.b <= 14
}

fn part_1(input_path: &str) -> std::io::Result<usize> {
    let file_handle = std::fs::File::open(input_path)?;
    let reader = BufReader::new(file_handle);

    let mut result = 0;

    for line in reader.lines() {
        let unwrapped_line = line?;
        let split_line: Vec<&str> = unwrapped_line.split(&[':', ';']).collect();
        let game_id = sscanf::sscanf!(split_line[0], "Game {}", usize).unwrap();
        
        let valid_draws_count = (&split_line[1..])
            .into_iter()
            .map(|draw_string| Draw::from_str(draw_string).unwrap())
            .filter(|draw| is_valid_draw_for_part_1(draw))
            .count();
        
        if valid_draws_count == (split_line.len() - 1) {
            //println!("Valid: {}", unwrapped_line);
            result += game_id;
        }
        else {
            //println!("Invalid: {}", unwrapped_line);
        }
    }

    Ok(result)
}

fn part_2(input_path: &str) -> std::io::Result<usize> {
    let file_handle = std::fs::File::open(input_path)?;
    let reader = BufReader::new(file_handle);

    let mut result = 0;

    for line in reader.lines() {
        let unwrapped_line = line?;
        let split_line: Vec<&str> = unwrapped_line.split(&[':', ';']).collect();
        
        let all_draws : Vec<Draw> = (&split_line[1..])
            .into_iter()
            .map(|draw_string| Draw::from_str(draw_string).unwrap())
            .collect();

        let mut componentwise_max_draw = Draw { r:0, g:0, b:0 };

        for draw in all_draws { 
            componentwise_max_draw.r = std::cmp::max(componentwise_max_draw.r, draw.r);
            componentwise_max_draw.g = std::cmp::max(componentwise_max_draw.g, draw.g);
            componentwise_max_draw.b = std::cmp::max(componentwise_max_draw.b, draw.b);
        }
        
        result += componentwise_max_draw.power();
    }

    Ok(result)
}

#[test]
pub fn run_test_1() -> std::io::Result<()> {
    assert_eq!(part_1("test_data/day2/example.txt").unwrap(), 8);
    Ok(())
}

#[test]
pub fn run_test_2() -> std::io::Result<()> {
    assert_eq!(part_2("test_data/day2/example.txt").unwrap(), 2286);
    Ok(())
}

pub fn run(input_path: &String) -> std::io::Result<()> {
    println!("Part one result: {}", part_1(&input_path)?);
    println!("Part two result: {}", part_2(&input_path)?);
    Ok(())
}
