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

    // array of both lines
    let data = read_input(&file_string);

    // tuples of the coordinates
    let mut first_line_path: Vec<(i32,i32)> = Vec::new();
    let mut second_line_path: Vec<(i32,i32)> = Vec::new();
    let mut cross_points: Vec<i32> = Vec::new();

    loop_through_instructions(&data[0], &mut first_line_path);
    loop_through_instructions(&data[1], &mut second_line_path);

    println!("{}", first_line_path.len());

    // find the matching tuples
    find_cross_points(&mut cross_points, &mut first_line_path, &mut second_line_path);
    cross_points.sort_unstable();
    let shortest_distance = cross_points[0];

    println!("This is the shortest_distance : {}", shortest_distance);
    // calculate distances to the center
}

fn read_input(file_string: &str) -> Vec<Vec<&str>> {
    let split_string: Vec<&str> = file_string.split("\n").collect();

    print!("{}\n", split_string.len());

    let mut two_line_data: Vec<Vec<&str>> = Vec::new();

    for i in split_string {
        let split_points: Vec<&str> = i.split(",").collect();
        two_line_data.push(split_points);
    }

    println!("{}", two_line_data[0].len());

    for i in &two_line_data[0] {
        print!("{}\n", i);
    }

    two_line_data
}

fn gather_points(instructions: Vec<&str>) {

}

fn find_cross_points(cross_points: &mut Vec<i32>, first_line: &mut Vec<(i32,i32)>, second_line: &mut Vec<(i32,i32)>) {
    for i in first_line {
        if second_line.contains(i) {
            let val = i.0.abs() + i.1.abs();
            println!("{}-{}", i.0, i.1);
            cross_points.push(val);
        }
    }
}

fn loop_through_instructions(line_instructions: &Vec<&str>, line_path: &mut Vec<(i32,i32)>) {
    for i in line_instructions {
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
        let number_of_steps = match string_int.parse::<i32>() {
            Ok(num) => num,
            Err(err) => panic!()
        };
        print!("{}\n", number_of_steps);

        add_points_to_path(line_path, &direction, &number_of_steps);
    }
}

fn add_points_to_path(path: &mut Vec<(i32,i32)>, direction: &char, steps_in_direction: &i32) {
    
    for u in 0..*steps_in_direction {
        // get last point so we know where to add from
        let last_point = match path.last() {
            Some(point) => *point,
            None => (0 as i32,0 as i32) 
        };

        let (x,y) = last_point;

        if *direction == 'U' {
            path.push((x,y+1));
        } else if *direction == 'R' {
            path.push((x+1,y));
        } else if *direction == 'D' {
            path.push((x,y-1));
        } else {
            path.push((x-1,y));
        }

        println!("{}-{}",last_point.0,last_point.1);
    }
}

fn gather_intersection_points() {

}
