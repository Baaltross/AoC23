use clap::Parser;

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    #[arg(short, long)]
    day: u16,
}

mod implementations;
mod utility;

pub fn main() -> utility::generic_error::GenericResult<()> {
    let args = Args::parse();
    let path_to_data = format!("data/day{}.txt", args.day);
    match &args.day {
        1 => implementations::day1::run(&path_to_data)?,
        2 => implementations::day2::run(&path_to_data)?,
        3 => implementations::day3::run(&path_to_data)?,
        4 => implementations::day4::run(&path_to_data)?,
        5 => implementations::day5::run(&path_to_data)?,
        _ => return Err(utility::generic_error::GenericError::IOError(std::io::Error::new(std::io::ErrorKind::InvalidInput, format!("Unknown day {}", args.day)))),
    }
    Ok(())
}
