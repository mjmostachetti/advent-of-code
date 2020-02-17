use std::fs;
use libm::floor;

fn main() {
    // (fuel / 3) -> round down -> subtract 2
    let path = "/Users/michaelmostachetti/dev/advent-of-code/day-1/day_one/src/input.txt";

    let data_str = match fs::read_to_string(path)
    {
        Err(why) => panic!("couldn't read {}: {}", path, why),
        Ok(string) =>
        {
            string
        }
    };

    let mut total: f64 = 0.0;

    for str in data_str.lines()
    {
        let int_parsed = str.trim().parse::<f64>().unwrap();
        total += get_total_fuel(int_parsed, 0.0);
    }

    println!("{}", total);
}

fn get_total_fuel(fuel: f64, mut total: f64) -> f64 {

    let modified_int = floor(fuel / (3 as f64)) - (2 as f64);

    if modified_int <= 0.0 {
        return total;
    } else {
        total += modified_int;
        return get_total_fuel(modified_int, total);
    }
}