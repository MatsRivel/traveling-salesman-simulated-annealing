mod constructors;
mod constants;
mod validity_check;
use constants::{NVEHICLES,NNODES,NCALLS};
use std::fs::{File,write};
use std::io::{self, BufRead};
use std::path::Path;

fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where P: AsRef<Path>, {
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}


fn main(){
    let mut vehicle_details_idx = 0;
    let mut vehicle_details: [[i32;3usize]; NVEHICLES] = [[-1i32;3usize]; NVEHICLES];
    let mut valid_calls_idx = 0;
    let mut valid_calls: [[i32;NCALLS]; NVEHICLES] = [[-1i32;NCALLS]; NVEHICLES];
    let mut call_details_idx = 0;
    let mut call_details: [[i32;8usize]; NCALLS] = [[-1i32;8usize]; NCALLS];
    let mut travel_costs_idx = 0;
    let mut travel_costs: [[i32;4usize];NNODES*NNODES*NVEHICLES] = [[-1i32;4usize];NNODES*NNODES*NVEHICLES];
    let mut node_cost_idx = 0;
    let mut node_costs: [[i32;5usize];NCALLS*NVEHICLES] = [[-1i32;5usize];NCALLS*NVEHICLES];
    let line_buffer = read_lines(r"C:\Users\Mats\Rust Projects\sim_an\Call_7_Vehicle_3.txt")
                    .expect("Could not unwrap LineBuffer");
    let mut info_idx = -1;
    // Prepare all arrays.
    for result_line in line_buffer{
        let line = result_line.expect("Failed to unwrap result_line");
        if line.starts_with("%"){
            info_idx += 1;
            //println!("\n{}",line);
            continue;
        }
        //println!("{}",line);
        match info_idx {
            2 => {  vehicle_details[vehicle_details_idx] = constructors::construct_vehicle_details(line);
                    vehicle_details_idx+=1;
                },
            4 => {  valid_calls[valid_calls_idx] = constructors::construct_valid_calls(line);
                    valid_calls_idx += 1;
            },
            5 => {  call_details[call_details_idx] = constructors::construct_call_details(line);
                    call_details_idx += 1;
            },
            6 => {  travel_costs[travel_costs_idx] = constructors::construct_travel_costs(line);
                    travel_costs_idx += 1
            },
            7 => {  node_costs[node_cost_idx] = constructors::construct_node_costs(line);
                    node_cost_idx += 1;
            },
            0 | 1 | 3 => {continue;},
            ..=-1 | 7.. => {panic!("Out of lines?")}
        }

    }

    // Generate a bad solution.
    let mut solution = [0i32;NCALLS*2];
    
    // Check validity:
    println!("\n\nIs the solution valid? {:?}",validity_check::correctness_check(
                                                        &solution,
                                                        &vehicle_details,
                                                        &call_details,
                                                        &travel_costs,
                                                        &node_costs));



}