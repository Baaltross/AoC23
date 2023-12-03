use std::io::{BufRead, BufReader};
use std::str::FromStr;

#[derive(PartialEq, Debug, Clone)]
struct Number {
    value: u32,
    start_index: i64,
    end_index: i64,
}

impl Number {
    fn new(value: u32, start_index: i64, end_index: i64) -> Number {
        Number {
            value: value,
            start_index: start_index,
            end_index: end_index,
        }
    }

    fn is_index_adjacent(&self, index: i64) -> bool {
        index >= (self.start_index - 1) && index <= (self.end_index + 1)
    }
}

#[derive(PartialEq, Debug)]
struct Symbol {
    value: char,
    index: i64,
}

impl Symbol {
    fn new(value: char, index: i64) -> Symbol {
        Symbol {
            value: value,
            index: index,
        }
    }
}

#[derive(Default, PartialEq, Debug)]
struct Row {
    numbers: Vec<Number>,
    symbols: Vec<Symbol>,
}

impl Row {
    fn new() -> Row {
        Default::default()
    }

    fn is_number_adjacent_to_symbol(&self, number: &Number) -> bool {
        for symbol in self.symbols.iter() {
            // Note: indices are signed so we can go negative here without Rust panicking
            if number.is_index_adjacent(symbol.index) {
                return true;
            }
        }

        false
    }

    // Note: does not empty output first
    fn collect_numbers_symbol_is_adjacent_to(&self, symbol: &Symbol, output: &mut Vec<Number>) {
        for number in self.numbers.iter() {
            if number.is_index_adjacent(symbol.index) {
                output.push(number.clone());
            }
        }
    }
}

impl FromStr for Row {
    type Err = std::io::Error;

    fn from_str(input: &str) -> Result<Self, Self::Err> {
        let mut output: Row = Row::new();

        enum CharType {
            Number,
            Symbol,
            Null,
        }

        let mut last_char_type = CharType::Null;

        for (index, char) in input.chars().enumerate() {
            last_char_type = match char {
                '.' => CharType::Null,
                c if c.is_digit(10) => {
                    if matches!(last_char_type, CharType::Number) {
                        // The previous char was a digit of this same number, accumulate onto it
                        let last_number = &mut output.numbers.last_mut().unwrap();
                        last_number.end_index = index as i64;
                        last_number.value = last_number.value * 10 + char.to_digit(10).unwrap();
                    } else {
                        // If this is the start of a new number then add that to the row
                        output.numbers.push(Number::new(
                            char.to_digit(10).unwrap(),
                            index as i64,
                            index as i64,
                        ));
                    }
                    CharType::Number
                }
                c => {
                    output.symbols.push(Symbol::new(c, index as i64));
                    CharType::Symbol
                }
            };
        }

        Ok(output)
    }
}

fn part_1(input_path: &str) -> std::io::Result<u32> {
    let file_handle = std::fs::File::open(input_path)?;
    let reader = BufReader::new(file_handle);

    let mut result = 0;
    let mut rows: Vec<Row> = Vec::new();

    for line in reader.lines() {
        let unwrapped_line = line?;
        rows.push(Row::from_str(&unwrapped_line.as_str())?);
    }

    for (index, row) in rows.iter().enumerate() {
        for number in row.numbers.iter() {
            // Check number against previous row
            if index > 0 && rows[index - 1].is_number_adjacent_to_symbol(number) {
                result += number.value;
                continue;
            }

            // Check number against this row
            if row.is_number_adjacent_to_symbol(number) {
                result += number.value;
                continue;
            }

            // Check number against next row
            if index < (rows.len() - 1) && rows[index + 1].is_number_adjacent_to_symbol(number) {
                result += number.value;
                continue;
            }
        }
    }

    Ok(result)
}

fn part_2(input_path: &str) -> std::io::Result<u32> {
    let file_handle = std::fs::File::open(input_path)?;
    let reader = BufReader::new(file_handle);

    let mut result = 0;
    let mut rows: Vec<Row> = Vec::new();

    for line in reader.lines() {
        let unwrapped_line = line?;
        rows.push(Row::from_str(&unwrapped_line.as_str())?);
    }

    for (index, row) in rows.iter().enumerate() {
        for symbol in row.symbols.iter() {
            if symbol.value != '*' {
                continue;
            }

            let mut numbers_adjacent_to_symbol: Vec<Number> = Vec::new();

            // Check against previous row
            if index > 0 {
                rows[index - 1].collect_numbers_symbol_is_adjacent_to(symbol, &mut numbers_adjacent_to_symbol);
            }

            // Check against this row
            row.collect_numbers_symbol_is_adjacent_to(symbol, &mut numbers_adjacent_to_symbol);

            // Check against next row
            if index < (rows.len() - 1) {
                rows[index + 1].collect_numbers_symbol_is_adjacent_to(symbol, &mut numbers_adjacent_to_symbol);
            }

            if numbers_adjacent_to_symbol.len() == 2 {
                result += numbers_adjacent_to_symbol[0].value * numbers_adjacent_to_symbol[1].value;
            }
        }
    }

    Ok(result)
}

#[test]
fn test_parsing_single_line() -> std::io::Result<()> {
    let mut test_row = Row::new();
    test_row.numbers.push(Number::new(58, 7, 8));
    test_row.symbols.push(Symbol::new('+', 5));
    assert_eq!(Row::from_str(".....+.58.").unwrap(), test_row);
    Ok(())
}

#[test]
fn test_part_1_example() -> std::io::Result<()> {
    assert_eq!(part_1("test_data/day3/example.txt").unwrap(), 4361);
    Ok(())
}

#[test]
fn test_part_2_example() -> std::io::Result<()> {
    assert_eq!(part_2("test_data/day3/example.txt").unwrap(), 467835);
    Ok(())
}

pub fn run(input_path: &String) -> std::io::Result<()> {
    println!("Part one result: {}", part_1(&input_path)?);
    println!("Part two result: {}", part_2(&input_path)?);
    Ok(())
}
