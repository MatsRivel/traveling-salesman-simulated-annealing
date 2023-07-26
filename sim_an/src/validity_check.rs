use std::{cmp::{min,max}, collections::HashMap};
use crate::constants::{NVEHICLES,NCALLS,TRAVEL_TIME_SIZE,SOLUTION_SIZE};

#[derive(Debug)]
pub enum CorrectnessError<'a>{ // TODO: Don't need to return the solution here... It is already owned by whatever calls the function returning these errors.
    CallNumberTooLow{call_number:i32, solution:&'a [i32;SOLUTION_SIZE]},
    CallNumberTooHigh{call_number:i32, solution:&'a [i32;SOLUTION_SIZE]},
    VehicleNumberTooLow{vehicle_number:i32, solution:&'a [i32;SOLUTION_SIZE]}, // Not needed here.
    VehicleNumberTooHigh{vehicle_number:i32, solution:&'a [i32;SOLUTION_SIZE]}, // Not needed here.
    NegativeLoad{ call:i32, unloaded_weight:i32, solution:&'a [i32;SOLUTION_SIZE]},
    Overloaded{call:i32,car_weight:i32,loaded_weight:i32,car_capacity:i32, solution:&'a [i32;SOLUTION_SIZE]},
    PickupNotDelivered{idx:usize,solution:&'a [i32;SOLUTION_SIZE]},
    CallTooManyTimes{call:i32, solution:&'a [i32;SOLUTION_SIZE]},
    PickUpTooLate{vehicle_idx:usize, call:i32,car_time:i32,arrival_time:i32,start_upper:i32, solution:&'a [i32;SOLUTION_SIZE]},
    DeliverTooLate{vehicle_idx:usize,call:i32,car_time:i32,arrival_time:i32,end_upper:i32, solution:&'a [i32;SOLUTION_SIZE]},
    NoPath{},
}


fn a_to_b_info(vehicle_idx:usize,a:i32,b:i32,travel_costs: &HashMap<(i32,i32,i32),(i32,i32)>) -> Option<&(i32,i32)>{
    let a_prime = min(a,b);
    let b_prime = max(a,b);
    let vehicle = vehicle_idx as i32 +1;
    travel_costs.get(&(vehicle,a_prime,b_prime))
}
pub fn a_to_b_time(vehicle_idx:usize,a:i32,b:i32,travel_costs: &HashMap<(i32,i32,i32),(i32,i32)>) -> Option<i32>{
    a_to_b_info(vehicle_idx, a, b, travel_costs).map(|v| v.0)
}
pub fn a_to_b_cost(vehicle_idx:usize,a:i32,b:i32,travel_costs: &HashMap<(i32,i32,i32),(i32,i32)>) -> Option<i32>{
    a_to_b_info(vehicle_idx, a, b, travel_costs).map(|v| v.1)
}
pub fn deconstruct_vehicle(vehicle_idx:usize,vehicle_details : &[[i32;3usize]; NVEHICLES]) -> (i32,i32,i32){
    // Home, start_time, capacity.
    (vehicle_details[vehicle_idx][0],vehicle_details[vehicle_idx][1],vehicle_details[vehicle_idx][2])
}
pub fn deconstruct_call(call:i32,call_details : &[[i32;8usize]; NCALLS]) -> (i32,i32,i32,i32,i32,i32,i32,i32){
    /*  origin,
        destination,
        call_size,
        cost_of_not_deliver,
        start_lower,
        start_upper,
        end_lower,
        end_upper */
    /*s
    println!("{:?}",call_details[(call-1) as usize]);
    println!("
    origin: {}
    destination: {}
    call_size: {}
    cost_of_not_deliver: {}
    start_lower: {}
    start_upper: {}
    end_lower: {}
    end_upper: {}",    call_details[(call-1) as usize][0],
    call_details[(call-1) as usize][1],
    0,//TODO: Weight is temporarily 0 | call_details[(call-1) as usize][2],
    call_details[(call-1) as usize][3],
    call_details[(call-1) as usize][4],
    call_details[(call-1) as usize][5],
    call_details[(call-1) as usize][6],
    call_details[(call-1) as usize][7]);
    */
    (call_details[(call-1) as usize][0],
    call_details[(call-1) as usize][1],
    call_details[(call-1) as usize][2],
    call_details[(call-1) as usize][3],
    call_details[(call-1) as usize][4],
    call_details[(call-1) as usize][5],
    call_details[(call-1) as usize][6],
    call_details[(call-1) as usize][7])
}
pub fn deconstruct_node(vehicle_idx:usize,call:i32,node_costs:&[[i32;5usize];NCALLS*NVEHICLES])->(i32,i32,i32,i32){
    /*  origin_time,
        origin_cost,
        destination_time,
        destination_cost*/
    let index = (call-1) as usize + vehicle_idx*NCALLS;
    (node_costs[index][1],node_costs[index][2],node_costs[index][3],node_costs[index][4])
}
fn get_idx_of_min(arr_in: [i32;NVEHICLES]) -> Option<usize>{
    if NVEHICLES == 0{
        return None;
    }

    let mut min = &arr_in[0];
    let mut index: usize = 0;
    for (idx,element) in arr_in.iter().enumerate().skip(1){
        if element <= min{
            min = element;
            index = idx;
        }
    }
    return Some(index);
}
pub fn _correctness_check<'a>(
        solution : &'a [i32;SOLUTION_SIZE],
        vehicle_details : &'a [[i32;3usize]; NVEHICLES],
        call_details : &'a [[i32;8usize]; NCALLS],
        travel_costs: &'a HashMap<(i32,i32,i32),(i32,i32)>,
        node_costs: &'a [[i32;5usize];NCALLS*NVEHICLES]
    ) -> Result<i32,CorrectnessError<'a>> {
    
    // All vehicles start at 0 weight.
    let mut vehicle_weights = [0i32;NVEHICLES];
    // All vehicles start at 0 costs.
    let mut vehicle_costs = [0i32;NVEHICLES];
    // All vehicles start at different times.
    let mut vehicle_times = [0i32;NVEHICLES];
    // All vehicles start at home.
    let mut vehicle_positions = [0i32;NVEHICLES];
    
    // Updating:
    for vehicle_idx in 0..NVEHICLES {
        let (vehicle_home, vehicle_start, _) = deconstruct_vehicle(vehicle_idx, vehicle_details); //(Home, start, capacity)
        vehicle_times[vehicle_idx] = vehicle_start;
        vehicle_positions[vehicle_idx] = vehicle_home;
    }

    // Each call can be performed 0 or 2 times. (Either finish it or don't even start!)
    let mut call_counter = [0i32;NCALLS];
    for call in solution.iter() {
        // Check that calls are in the range of (0,NNCALLS]
        if *call <= 0{
            // println!("{}: {:?}","CallNumberTooLow",solution);
            return Err(CorrectnessError::CallNumberTooLow{call_number:*call, solution});
        }if *call > (NCALLS as i32){
            // println!("{}: {:?}","CallNumberTooHigh",solution);
            return Err(CorrectnessError::CallNumberTooHigh{call_number:*call, solution})
        }
        let car_idx = get_idx_of_min(vehicle_times).expect("Program will not get to this point without at least ONE vehicle...");
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


        match (car_pos, origin) {
            (_,0) | (0, _) => panic!("|| Node-numbers can not be zero! Will cause overflow to INTMAX"),
            _ => ()
        }

        let moving_time = match a_to_b_time(car_idx,car_pos,origin,travel_costs){
            Some(v) => v,
            None => return Err(CorrectnessError::NoPath{}),
        };
        let moving_cost = match a_to_b_cost(car_idx,car_pos,origin,travel_costs){
            Some(v) => v,
            None => return Err(CorrectnessError::NoPath{}),
        };

        // Check if pick-up or deliver. If already delivered, can not be called again.
        match call_counter[(call-1) as usize]{
            1 => {
                let arrival_time = car_time + moving_time + origin_time;
                let loaded_weight = car_weight+call_size;
                if arrival_time > start_upper{
                    // println!("{}: {:?}","PickUpTooLate",solution);
                    return Err(CorrectnessError::PickUpTooLate{vehicle_idx:car_idx,call:*call,car_time,arrival_time, start_upper, solution});
                }if loaded_weight > car_capacity{
                    // println!("{}: {:?}","Overloaded",solution);
                    return Err(CorrectnessError::Overloaded {call:*call, car_weight, loaded_weight, car_capacity, solution});
                }
                vehicle_times[car_idx]  = max(arrival_time, start_lower);
                vehicle_weights[car_idx] = loaded_weight;
                vehicle_positions[car_idx] = origin;
                vehicle_costs[car_idx] += moving_cost + origin_cost;
            },
            2 =>  {
                let arrival_time = car_time + moving_time + destination_time;
                if arrival_time > end_upper{
                    // println!("{}: {:?}","DeliverTooLate",solution);
                    return Err(CorrectnessError::DeliverTooLate{vehicle_idx:car_idx, call:*call, car_time, arrival_time, end_upper, solution});
                }
                let unloaded_weight = car_weight-call_size;
                if unloaded_weight < 0{
                    // println!("{}: {:?}","NegativeLoad",solution);
                    return Err(CorrectnessError::NegativeLoad{ call:*call, unloaded_weight, solution})
                }
                vehicle_times[car_idx]  = max(arrival_time, end_lower);
                vehicle_weights[car_idx] = unloaded_weight;
                vehicle_positions[car_idx] = destination;
                vehicle_costs[car_idx] += moving_cost + destination_cost;
                
            },
            _ =>{// println!("{}: {:?}","CallTooManyTimes",solution);
                 return Err(CorrectnessError::CallTooManyTimes { call:*call,solution})},
        }
    }
    for (idx, count) in call_counter.iter().enumerate(){
        match count {
            1 => return Err(CorrectnessError::PickupNotDelivered { idx, solution}),
            _ => continue
        }
    }

    let total_cost = vehicle_costs.iter().sum();
    //println!("{}: {:?}","\t ### Solution OK ###",solution);
    return Ok(total_cost);
}

