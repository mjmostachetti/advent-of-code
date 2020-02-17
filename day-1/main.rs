use std::fs;
use math::round;

fn main() {
    // (fuel / 3) -> round down -> subtract 2
    let path = "./input.txt";

    let data_str = match fs::read_to_string(path)
    {
        Err(why) => panic!("couldn't read {}: {}", path, why),
        Ok(string) =>
        {
            string
        }
    };

    let mut data : Vec<u32> = vec![];

    let total = 0;

    for str in data_str.lines()
    {
        let int_parsed = str.trim().parse::<u32>().unwrap();
        let modified_int = round::floor(int_parsed / 3, 0) - 2;
        total += modified_int;
    }

    println!("{}", total);
    //println!("{}",data.len());

    // for x in data {
    //     println!("{}", x);
    // }
}

// fn read_a_file() -> std::io::Result<Vec<u32>> {
//     let mut file = File::open("./input.txt")?;

//     let mut data = Vec::new();
//     file.read_to_end(&mut data)?;

//     return Ok(data);
// }

// fn print_array(array: Vec<u32>) {
//     for x in array {
//         println!("{}", x);
//     }
// }