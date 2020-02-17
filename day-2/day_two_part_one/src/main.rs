use std::fs;

fn main() {
    let path = "/Users/michaelmostachetti/dev/advent-of-code/day-2/day_two_part_one/src/input.txt";

    // read the file
    let file_result = fs::read_to_string(path);

    let file_string = match file_result {
        Ok(file) => file,
        Err(_error) => {
            panic!("game over.")
        }
    };
    // split into a vector
    print!("{}\n", file_string);

    let v: Vec<&str> = file_string.split(',').collect();

    print!("{}\n", v.len());

    let num: u64 = 0;

    let x: Vec<u64> = v.iter().map(string_to_int).collect();

    print!("{}", x.len());
    // pass into recursive function that takes the vector, and the next starting position
    let final_position_zero = run_program(x, num);
    print!("\nfinal answer : {}\n", final_position_zero);
}

fn string_to_int(s: &&str) -> u64 {
    s.parse::<u64>().unwrap()
}

fn run_program(mut v: Vec<u64>, opcode_position: u64) -> u64 {

    let first_index = opcode_position as usize;
    let second_index = &first_index + 1;
    let third_index = &second_index + 1;
    let fourth_index = &third_index + 1;
    let set_position = v[fourth_index];

    let opcode = match v.get(first_index) {
        Some(number) => *number,
        None => panic!("something at this index")
    };

    let new_opcode_position = opcode_position + (4 as u64);
    
    print!("new opcode position - {}", new_opcode_position);

    if opcode == (1 as u64) {
        print!("sum\n");
        let first_add = v.get(second_index).unwrap();
        let second_add = v.get(third_index).unwrap();
        let sum = v[*first_add as usize] + v[*second_add as usize];
        print!("{},{},{}", first_add, second_add, sum);
        v[set_position as usize] = sum;
    } else if opcode == (2 as u64) {
        print!("mult\n");
        let first_mult = v.get(second_index).unwrap();
        let second_mult = v.get(third_index).unwrap();
        let product = v[*first_mult as usize] * v[*second_mult as usize];
        v[set_position as usize] = product;
    } else {
        return *v.get(0).unwrap();  
    }




    return run_program(v,new_opcode_position);
}
