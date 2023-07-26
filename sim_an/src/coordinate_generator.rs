use std::collections::HashMap;

use crate::{constants::{SOLUTION_SIZE, NVEHICLES, NCALLS}, validity_check::deconstruct_vehicle, alter_solution::{CarInfo, Info}};

pub fn get_node_coords(
    car_node_order: &Vec<Vec<i32>>,
    vehicle_details : &[[i32;3usize]; NVEHICLES],
    call_details : &[[i32;8usize]; NCALLS],
    travel_costs: &HashMap<(i32,i32,i32),(i32,i32)>,
    node_costs: &[[i32;5usize];NCALLS*NVEHICLES]
) -> Vec<Vec<(i32,i32)>>{
    let mut node_coords = Vec::new();
    for _ in 0..NVEHICLES {
        node_coords.push(vec![(0i32,0i32)]);
        // Make function to calculate coords.
    }

    node_coords
}

pub fn find_node_orders( // Note: Might be too complex, but currently works the same as the solving function, so at least the functionality matches.
    solution: &[i32;SOLUTION_SIZE],
    vehicle_details : &[[i32;3usize]; NVEHICLES],
    call_details : &[[i32;8usize]; NCALLS],
    travel_costs: &HashMap<(i32,i32,i32),(i32,i32)>,
    node_costs: &[[i32;5usize];NCALLS*NVEHICLES]
)-> Vec<Vec<i32>> {
        let mut vehicle_position_history:Vec<Vec<i32>> = Vec::<Vec<i32>>::new();
        let mut car_pos = [0i32;NVEHICLES];
        let mut car_times = [0i32;NVEHICLES];
        for vehicle_idx in 0..NVEHICLES{
            let (vehicle_home, vehicle_start, _ ) = deconstruct_vehicle(vehicle_idx, vehicle_details);
            car_pos[vehicle_idx] = vehicle_home;
            vehicle_position_history.push(vec![vehicle_home]);
            car_times[vehicle_idx] = vehicle_start;
        }
        let mut call_counter = [0;NCALLS];
        for &call in solution.iter(){
            let mut cheapest_vehicle:Option<usize> = None;
            let mut cheapest_call:Option<usize> = None;
            let mut cheapest_cost:Option<i32> = None;
            let mut cheapest_time:Option<i32> = None;
            let mut cheapest_to_node:Option<i32> = None;
            let mut cheapest_time_delta:Option<i32> = None;
            let mut car_info = CarInfo{vehicle_idx:0, car_pos, car_times};
            for vehicle_idx in 0..NVEHICLES{
                car_info.vehicle_idx = vehicle_idx;
                let possible_informant:Option<Info> = Info::new(call_counter[call as usize-1], &car_info, call, node_costs, call_details,travel_costs);
                let informant = match possible_informant {
                    Some(v) => v,
                    None => continue // If no valid data, leave
                };
                if informant.is_faster(cheapest_time_delta) || cheapest_call.is_none(){
                    cheapest_vehicle = Some(vehicle_idx);
                    if let Some(i) = cheapest_call{
                        call_counter[i-1] -= 1;
                    }
                    cheapest_call = Some(call as usize);
                    call_counter[call as usize-1] += 1;
                    (cheapest_to_node, cheapest_time, cheapest_time_delta, cheapest_cost) = informant.get_info();
                }
            }
            match (cheapest_call, cheapest_vehicle, cheapest_to_node, cheapest_time, cheapest_cost) {
                (Some(_call_value),Some(vehicle_value),Some(to_node_value),Some(time_value), Some(cost_value)) => {
                    car_times[vehicle_value] = time_value;
                    car_pos[vehicle_value] = to_node_value;
                    vehicle_position_history[vehicle_value].push(to_node_value);
                }
                _ => ()
            }
        }
        vehicle_position_history
    
    }