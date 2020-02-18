// figure out all the points that the two paths draw
// make two arrays and return a new array of the cross points
// make function that checks 

// todo - learn to parse into new types

use std::fs;

fn main() {
    let file = fs::read_to_string("/Users/michaelmostachetti/dev/advent-of-code/day-3/day_three_part_one/src/input.txt");

    let file_string = match file {
        Ok(file_contents) => file_contents,
        Err(error) => panic!("An error reaidng file : {}\n", error)
    };

    let data = read_input(&file_string);

    let first_line_path: Vec<(u64,u64)> = Vec::new();
    let second_line_path: Vec<(u64,u64)> = Vec::new();

    for i in &data[0] {
        // get direction
        let mut characters = i.chars();
        let direction = match characters.next() {
            Some(c) => c,
            None => panic!()
        };
        print!("{}\n", direction);

        // build string and parse as number
        let mut string_int = String::new();
        for z in 0..characters.as_str().len() {
            match characters.next() {
                Some(character) => string_int.push(character),
                None => panic!()
            };
        }
        let number_of_steps = match string_int.parse::<u32>() {
            Ok(num) => num,
            Err(err) => panic!()
        };
        print!("{}\n", number_of_steps);

    }

}

fn read_input(file_string: &str) -> Vec<Vec<&str>> {
    let split_string: Vec<&str> = file_string.split("\n").collect();

    print!("{}\n", split_string.len());

    let mut two_line_data: Vec<Vec<&str>> = Vec::new();

    for i in split_string {
        let split_points: Vec<&str> = i.split(",").collect();
        two_line_data.push(split_points);
    }

    print!("{}\n", two_line_data[0].len());

    for i in &two_line_data[0] {
        print!("{}\n", i);
    }

    two_line_data
}

fn gather_points(instructions: Vec<&str>) {

}

fn add_points_to_path(instruction: &str) {

}

fn gather_intersection_points() {

}
