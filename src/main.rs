use std::env;
use std::io::{self, Read};

mod day1;
mod day2;

fn parse_day_arg() -> Result<impl Fn(String) -> (), String> {
    let args: Vec<String> = env::args().collect();
    let day = args.get(1);
    match day {
        None => Err("Missing day".to_string()),
        Some(day) => {
            match day.as_str() {
                "day1" => Ok(day1::day1 as fn(String)),
                "day2" => Ok(day2::day2 as fn(String)),
                _ => Err("No such day implemented: ".to_string() + day),
            }
        }
    }
}

fn main() {
    let day_fn = parse_day_arg().unwrap();
    let input = {
        let mut buffer = String::new();
        io::stdin().read_to_string(&mut buffer).unwrap();
        buffer
    };
    day_fn(input);
}
