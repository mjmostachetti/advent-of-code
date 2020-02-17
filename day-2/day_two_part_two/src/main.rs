// Figure out what values return - 19690720
use std::process;
use std::fs;

fn main() {
    
    let _memory_store = create_new_memory_store();

    let all_tuples_to_try = create_vector_of_all_tests(99);

    for tup in all_tuples_to_try {
        let (first,second) = tup;
        let mut cloned_memory = create_new_memory_store();
        
        cloned_memory[1] = first;
        cloned_memory[2] = second;

        print!("new first and second - {},{}\n ", cloned_memory[1], cloned_memory[2]);
        
        let final_position_zero = run_program(cloned_memory, 0);

        print!("Trying - {},{},{}\n", first, second, final_position_zero);

        if does_end_game(&final_position_zero) {
            print!("winning pair - {},{}", first, second);
            print!("final calculation : {}\n", (100 * first * second));
            process::exit(0x0100);
        }
    }
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
    
    print!("new opcode position - {}\n", new_opcode_position);

    if opcode == (1 as u64) {
        let first_add = v.get(second_index).unwrap();
        let second_add = v.get(third_index).unwrap();
        let sum = v[*first_add as usize] + v[*second_add as usize];
        v[set_position as usize] = sum;
    } else if opcode == (2 as u64) {
        let first_mult = v.get(second_index).unwrap();
        let second_mult = v.get(third_index).unwrap();
        let product = v[*first_mult as usize] * v[*second_mult as usize];
        v[set_position as usize] = product;
    } else {
        print!("{}", opcode);

        // let stuff_str: String = v.into_iter().map(|i| i.to_string()).collect::<String>();

        // print!("Memory at the end - {}\n", stuff_str);

        return *v.get(0).unwrap();  
    }

    return run_program(v,new_opcode_position);
}

fn create_vector_of_all_tests(max: u64) -> Vec<(u64,u64)> {
    
    let mut all_potential_pairs: Vec<(u64, u64)> = Vec::new();

    for i in 0..max {
        for z in 0..max {
            all_potential_pairs.push((i,z));
        }
    }

    all_potential_pairs
}

fn does_end_game(final_score: &u64) -> bool {
    *final_score == (19690720 as u64)
}

fn create_new_memory_store() -> Vec<u64> {
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

    let x: Vec<u64> = v.iter().map(string_to_int).collect();

    x
}