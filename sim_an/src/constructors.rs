use crate::constants::{NVEHICLES,NCALLS, TRAVEL_TIME_SIZE,SOLUTION_SIZE};
use crate::constructors;
use std::convert::TryInto;
use std::env;
use std::path::Path;
use std::fs::File;
use std::io::{self, BufRead};

pub struct AllData{
    vehicle_details: [[i32; 3]; NVEHICLES],
    valid_calls: [[i32; NCALLS]; NVEHICLES],
    call_details: [[i32; 8]; NCALLS],
    travel_costs: [[i32; 4]; TRAVEL_TIME_SIZE],
    node_costs: [[i32; 5]; NCALLS*NVEHICLES]
}

impl AllData{
    pub fn deconstruct(&self) -> ([[i32; 3]; NVEHICLES], [[i32; NCALLS]; NVEHICLES], [[i32; 8]; NCALLS], [[i32; 4]; TRAVEL_TIME_SIZE], [[i32; 5]; NCALLS*NVEHICLES] ){
        (self.vehicle_details, self.valid_calls, self.call_details, self.travel_costs, self.node_costs)
    }
}
fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where P: AsRef<Path>, {
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}
pub fn construct_vehicle_details(s:String) -> [i32; 3usize] {
    s.split(",")
    .map(|s|
        s.to_string()
        .parse::<i32>()
        .expect("Could not convert str to i32 for VehicleDetails")
    ).skip(1)
    .collect::<Vec<i32>>().try_into().expect("Failed to convert Vec to [i32;4usize]")
}
pub fn construct_valid_calls(s:String)-> [i32;NCALLS] {
    let mut output = [-1i32;NCALLS];
    let iterator = s.split(",")
    .map(|s| 
        s.to_string()
        .parse::<i32>()
        .expect("Could not convert str to i32 for ValidVechicleCall.valid_calls")
    ).skip(1);
    for (idx, value) in iterator.enumerate(){
        output[idx] = value;
    }
    return output;
}
pub fn construct_call_details(s:String) -> [i32;8usize] {
    s.split(",")
        .map(|s|
            s.to_string()
            .parse::<i32>()
            .expect("Could not convert str to i32 for CallDetails")
        ).skip(1)
        .collect::<Vec<i32>>().try_into().expect("Failed to convert Vec to [i32;8usize]")
}
pub fn construct_travel_costs(s:String) -> [i32;4usize]{
    s.split(",")
    .map(|s|
        s.to_string()
        .parse::<i32>()
        .expect("Could not convert str to i32 for TravelCost")
    ).skip(1)
    .collect::<Vec<i32>>().try_into().expect("Failed to convert Vec to [i32;5usize]")
}
pub fn construct_node_costs(s:String) -> [i32;5usize] {
    s.split(",")
        .map(|s|
            s.to_string()
            .parse::<i32>()
            .expect("Could not convert str to i32 for NodeCost")
        ).skip(1)
        .collect::<Vec<i32>>().try_into().expect("Failed to convert Vec to [i32;6usize]")
}


pub fn get_all_data(path:&Path) -> AllData{
    let mut vehicle_details_idx = 0;
    let mut vehicle_details: [[i32;3usize]; NVEHICLES] = [[-1i32;3usize]; NVEHICLES];

    let mut valid_calls_idx = 0;
    let mut valid_calls: [[i32;NCALLS]; NVEHICLES] = [[-1i32;NCALLS]; NVEHICLES];

    let mut call_details_idx = 0;
    let mut call_details: [[i32;8usize]; NCALLS] = [[-1i32;8usize]; NCALLS];

    let mut travel_costs_idx = 0;
    let mut travel_costs: [[i32;4usize];TRAVEL_TIME_SIZE] = [[-1i32;4usize];TRAVEL_TIME_SIZE];

    let mut node_cost_idx = 0;
    let mut node_costs: [[i32;5usize];NCALLS*NVEHICLES] = [[-1i32;5usize];NCALLS*NVEHICLES];
    
    let line_buffer = match read_lines(path){
        Ok(v) => v,
        Err(e) => {
            println!("### {} ###",e);
            let current_dir =  env::current_dir().unwrap();
            println!("Current dir: ___| {:?}{:?}", current_dir,path);
            panic!("Could not read");
        },
    };
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
    return  AllData{vehicle_details, valid_calls, call_details, travel_costs, node_costs};
}