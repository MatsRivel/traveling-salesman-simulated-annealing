use crate::constants::NCALLS;
use std::convert::TryInto;

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
