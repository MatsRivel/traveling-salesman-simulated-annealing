use std::cmp::{min,max};
use crate::constants::{NVEHICLES,NNODES,NCALLS};
// Note to self: either a or b is zero here... So conversion breaks completely...

/*
TODO!
*/

fn a_to_b_time(vehicle_idx:usize,a:i32,b:i32,travel_costs: &[[i32;4usize];NNODES*NNODES*NVEHICLES]) -> i32{
    let A = min(a-1,b-1)as usize;
    let B = max((a-1),(b-1)) as usize;
    let index:usize = vehicle_idx%NVEHICLES + (B + A*NNODES)*NVEHICLES;
    travel_costs[index][2]
}
fn a_to_b_cost(vehicle_idx:usize,a:i32,b:i32,travel_costs: &[[i32;4usize];NNODES*NNODES*NVEHICLES]) -> i32{
    let A = min(a-1,b-1)as usize;
    let B = max((a-1),(b-1)) as usize;
    let index:usize = vehicle_idx%NVEHICLES + (B + A*NNODES)*NVEHICLES;
    travel_costs[index][3]
}
fn deconstruct_vehicle(vehicle_idx:usize,vehicle_details : &[[i32;3usize]; NVEHICLES]) -> (i32,i32,i32){
    (vehicle_details[vehicle_idx][0],vehicle_details[vehicle_idx][1],vehicle_details[vehicle_idx][2])
}
fn deconstruct_call(call:i32,call_details : &[[i32;8usize]; NCALLS]) -> (i32,i32,i32,i32,i32,i32,i32,i32){
    (call_details[(call-1) as usize][0],
    call_details[(call-1) as usize][1],
    call_details[(call-1) as usize][2],
    call_details[(call-1) as usize][3],
    call_details[(call-1) as usize][4],
    call_details[(call-1) as usize][5],
    call_details[(call-1) as usize][6],
    call_details[(call-1) as usize][7])
}
fn deconstruct_node(vehicle_idx:usize,call:i32,node_costs:&[[i32;5usize];NCALLS*NVEHICLES])->(i32,i32,i32,i32){
    let index = (call-1) as usize + vehicle_idx*NCALLS;
    (node_costs[index][1],node_costs[index][2],node_costs[index][3],node_costs[index][4])
}

pub fn correctness_check(
        solution : &[i32;NCALLS*2],
        vehicle_details : &[[i32;3usize]; NVEHICLES],
        call_details : &[[i32;8usize]; NCALLS],
        travel_costs: &[[i32;4usize];NNODES*NNODES*NVEHICLES],
        node_costs: &[[i32;5usize];NCALLS*NVEHICLES]
        ) -> Result<i32,String> {
    
    // All vehicles start at 0 weight.
    let mut vehicle_weights = vec![0i32;NVEHICLES];
    // All vehicles start at 0 costs.
    let mut vehicle_costs = vec![0i32;NVEHICLES];
    // All vehicles start at different times.
    let mut vehicle_times = vec![0i32;NVEHICLES];
    // All vehicles start at home.
    let mut vehicle_positions = vec![0i32;NVEHICLES];
    // Updating:
    for vehicle_idx in 0..NVEHICLES {
        let (vehicle_start, vehicle_home, _) = deconstruct_vehicle(vehicle_idx, vehicle_details);
        vehicle_times[vehicle_idx] = vehicle_start;
        vehicle_positions[vehicle_idx] = vehicle_home;
    }

    // Each call can be performed 0 or 2 times. (Either finish it or don't even start!)
    let mut call_counter = [0i32;NCALLS];
    let car_idx = 0; // == Car number -1
    
    for call in solution.iter() {
        call_counter[(call-1) as usize] += 1;
        let car_time = vehicle_times[car_idx];
        let car_pos = vehicle_positions[car_idx];
        let car_weight = vehicle_weights[car_idx];
        let car_capacity = vehicle_details[car_idx][2];

        // Note: Origin and destination is from the perspective of the call, not the vehicle.
        let (origin,
            destination,
            call_size,
            _,
            start_lower,
            start_upper,
            end_lower,
            end_upper) = deconstruct_call(*call,call_details);
        
        let (origin_time,
            origin_cost,
            destination_time,
            destination_cost) = deconstruct_node(car_idx, *call, node_costs);

        let moving_time = a_to_b_time(car_idx,car_pos,origin,travel_costs);
        let moving_cost = a_to_b_cost(car_idx,car_pos,origin,travel_costs);
        match call_counter[(call-1) as usize]{
            1 => {
                let arrival_time = car_time + moving_time + origin_time;
                let loaded_weight = car_weight+call_size;
                if arrival_time >start_upper{
                    return Err(format!("Could not pick up call {} in time.\nCurrent time: {}, Arrival time{}, Upper PickUp: {}",
                                call,car_time,moving_time+origin_time,start_upper));
                }else if loaded_weight > car_capacity{
                    return Err(format!("Overloaded at call {}.\nCurrent weight: {}, Loaded weight{}, Capacity: {}",
                    call,car_weight,car_weight+call_size,car_capacity));
                }else{
                    vehicle_times[car_idx]  = max(arrival_time, start_lower);
                    vehicle_weights[car_idx] = loaded_weight;
                    vehicle_positions[car_idx] = origin;
                    vehicle_costs[car_idx] += moving_cost + origin_cost
                }
            },
            2 =>  {
                let arrival_time = car_time + moving_time + destination_time;
                if arrival_time > end_upper{
                    return Err(format!("Could not deliver call {} in time.\nCurrent time: {}, Arrival time{}, Upper Deliver: {}",
                                call,car_time,moving_time+origin_time,end_upper));
                }else{
                    let unloaded_weight = car_weight-call_size;
                    vehicle_times[car_idx]  = max(arrival_time, end_lower);
                    vehicle_weights[car_idx] = unloaded_weight;
                    vehicle_positions[car_idx] = destination;
                    vehicle_costs[car_idx] += moving_cost + destination_cost;
                }
            },
            _ => return Err(format!("Call {} occurred more than 2 times.\nCurrent solution: {:?}",call,solution)),
        }
    }
    let total_cost = vehicle_costs.iter().sum();
    return Ok(total_cost);
}

