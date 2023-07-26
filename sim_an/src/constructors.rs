use crate::constants::{NVEHICLES,NCALLS, NNODES, TRAVEL_TIME_SIZE};
use crate::constructors;
use std::collections::HashMap;
use std::convert::TryInto;
use std::env;
use std::path::Path;
use std::fs::File;
use std::io::{self, BufRead};

pub struct AllDataReference<'a>{
    vehicle_details: &'a [[i32; 3]; NVEHICLES],
    valid_calls: &'a [[i32; NCALLS]; NVEHICLES],
    call_details: &'a [[i32; 8]; NCALLS],
    travel_costs: &'a HashMap<(i32,i32,i32),(i32,i32)>,
    node_costs: &'a [[i32; 5]; NCALLS*NVEHICLES]
}
#[allow(clippy::type_complexity)]
impl AllDataReference<'_> {
    fn deconstruct(&self) -> (&[[i32; 3]; NVEHICLES], &[[i32; NCALLS]; NVEHICLES], &[[i32; 8]; NCALLS], &HashMap<(i32,i32,i32),(i32,i32)>, &[[i32; 5]; NCALLS*NVEHICLES] ){
        (self.vehicle_details, self.valid_calls, self.call_details, self.travel_costs, self.node_costs) 
    }
}
trait DeconstructArray{
    type Output;
    fn deconstruct_array(&self) -> Self::Output;
}
impl DeconstructArray for [i32;5]{
    type Output = (i32,i32,i32,i32,i32);
    fn deconstruct_array(&self) -> Self::Output {
        (self[0],self[1],self[2],self[3],self[4])
    }
}

pub struct AllData{
    vehicle_details: [[i32; 3]; NVEHICLES],
    valid_calls: [[i32; NCALLS]; NVEHICLES],
    call_details: [[i32; 8]; NCALLS],
    travel_costs: HashMap<(i32,i32,i32),(i32,i32)>,
    node_costs: [[i32; 5]; NCALLS*NVEHICLES]
}
#[allow(clippy::type_complexity)]
impl AllData{
    pub fn deconstruct(&self) -> ([[i32; 3]; NVEHICLES], [[i32; NCALLS]; NVEHICLES], [[i32; 8]; NCALLS], HashMap<(i32,i32,i32),(i32,i32)>, [[i32; 5]; NCALLS*NVEHICLES] ){
        (self.vehicle_details, self.valid_calls, self.call_details, self.travel_costs.clone(), self.node_costs) //Note: Find a way to avoid cloning?
    }
}
fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where P: AsRef<Path>, {
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}
pub fn construct_vehicle_details(s:String) -> [i32; 3usize] {
    s.split(',')
    .map(|s|
        s.to_string()
        .parse::<i32>()
        .expect("Could not convert str to i32 for VehicleDetails")
    ).skip(1)
    .collect::<Vec<i32>>().try_into().expect("Failed to convert Vec to [i32;4usize]")
}
pub fn construct_valid_calls(s:String)-> [i32;NCALLS] {
    let mut output = [-1i32;NCALLS];
    let iterator = s.split(',')
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
    s.split(',')
        .map(|s|
            s.to_string()
            .parse::<i32>()
            .expect("Could not convert str to i32 for CallDetails")
        ).skip(1)
        .collect::<Vec<i32>>().try_into().expect("Failed to convert Vec to [i32;8usize]")
}
pub fn construct_travel_costs(s:String, used_nodes:[bool;NNODES]) -> Option<[i32;5usize]>{
    // arr == [vehicle, origin node, destination node, travel time (in hours), travel cost (in $)]
    let arr:[i32;5usize] = s.split(',')
    .map(|s|
        s.to_string()
        .parse::<i32>()
        .expect("Could not convert str to i32 for TravelCost")
    )
    .collect::<Vec<i32>>().try_into().expect("Failed to convert Vec to [i32;5usize]");

    if !used_nodes[arr[1usize] as usize-1] || !used_nodes[arr[2usize] as usize-1]{
        return None;
    } 
    return Some(arr);
}
pub fn construct_node_costs(s:String) -> [i32;5usize] {
    s.split(',')
        .map(|s|
            s.to_string()
            .parse::<i32>()
            .expect("Could not convert str to i32 for NodeCost")
        ).skip(1)
        .collect::<Vec<i32>>().try_into().expect("Failed to convert Vec to [i32;6usize]")
}


pub fn get_all_data(path:&Path) -> AllData{
    // Vehicle details = [home node, starting time, capacity]
    // Note: Vehicle is implicitly also considered.
    let mut vehicle_details: [[i32;3usize]; NVEHICLES] = [[-1i32;3usize]; NVEHICLES];
    let mut vehicle_details_idx = 0;

    //Valid calls = ["list of calls that can be transported using this vehicle"]
    // Note: Vehicle is implicitly also considered.
    let mut valid_calls: [[i32;NCALLS]; NVEHICLES] = [[-1i32;NCALLS]; NVEHICLES];
    let mut valid_calls_idx = 0;

    // Call details = [origin node, destination node, size, cost of not transporting, lowerbound timewindow for pickup, upper_timewindow for pickup, lowerbound timewindow for delivery, upper_timewindow for delivery]
    // Note: Call is implicitly also considered.
    let mut call_details: [[i32;8usize]; NCALLS] = [[-1i32;8usize]; NCALLS];
    let mut call_details_idx = 0;

    // Travel costs: Key:(vehicle, origin node, destination node) =  Value: (travel time (in hours), travel cost (in $))
    let mut travel_costs:HashMap<(i32,i32,i32),(i32,i32)> = HashMap::new();

    // Node costs: [call, origin node time (in hours), origin node costs (in �), destination node time (in hours), destination node costs (in $)]
    // Note: Vehicle is implicitly also considered.
    let mut node_costs: [[i32;5usize];NCALLS*NVEHICLES] = [[-1i32;5usize];NCALLS*NVEHICLES];
    let mut node_cost_idx = 0;

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
    let mut used_nodes: [bool;NNODES] = [false;NNODES];
    for result_line in line_buffer{
        let line = result_line.expect("Failed to unwrap result_line");
        if line.starts_with('%'){
            info_idx += 1;
            continue;
        }
        match info_idx {
            2 => {  vehicle_details[vehicle_details_idx] = constructors::construct_vehicle_details(line);
                let home_node_idx = vehicle_details[vehicle_details_idx][0] as usize -1;
                    used_nodes[home_node_idx] = true;
                    vehicle_details_idx+=1;
                },
            4 => {  valid_calls[valid_calls_idx] = constructors::construct_valid_calls(line);
                    valid_calls_idx += 1;
                },
            5 => {  call_details[call_details_idx] = constructors::construct_call_details(line);
                    let (origin_node_index, destination_node_index) = (call_details[call_details_idx][0] as usize-1, call_details[call_details_idx][1] as usize-1);
                    used_nodes[origin_node_index] = true;
                    used_nodes[destination_node_index] = true;
                    call_details_idx += 1;
                },
            6 => {  if let Some(v) = constructors::construct_travel_costs(line,used_nodes){
                        let (vehicle, origin,destination,time,cost) = v.deconstruct_array();
                        travel_costs.insert((vehicle, origin, destination),(time,cost));
                    }
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