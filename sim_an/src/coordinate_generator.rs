use std::collections::HashMap;
use crate::{constants::{SOLUTION_SIZE, NVEHICLES, NCALLS}, validity_check::{deconstruct_vehicle, a_to_b_time}, alter_solution::{CarInfo, Info}};

fn coord_options_from_two_points(coord_a:[i32;2], coord_b:[i32;2], dist_a_b:i32, dist_a_c:i32, dist_b_c:i32) -> [Option<[i32;2]>;2]{
    if coord_a == coord_b{
        return [None,None];
    }
    let [a0, a1] = coord_a;
    let [b0, b1] = coord_b;
    let h_squared = dist_a_c.pow(2) + dist_b_c.pow(2);
    let h = (h_squared as f64).sqrt() as i32;
    let dist_a_q = ((dist_a_c.pow(2) - dist_b_c.pow(2) + dist_a_b.pow(2))) / (2*dist_a_b);
    let [q0, q1] = [a0 + dist_a_q*(b0-a0)/dist_a_b, a1 + dist_a_q*(b1-a1)/dist_a_b]; 
    let c0 = q0 + h *(b1-a1)/dist_a_b;
    let c1 = q1 - h * (b0-a0)/dist_a_b;
    let d0 = q0 - h *(b1-a1)/dist_a_b;
    let d1 = q1 + h * (b0-a0)/dist_a_b;
    if (c0,c1) == (d0,d1) {
        return [Some([c0,c1]),None::<[i32;2]>];
    }
    [Some([c0,c1]),Some([d0,d1])]

}pub fn get_node_coords(
    node_order: &Vec<i32>,
    travel_costs: &HashMap<(i32,i32,i32),(i32,i32)>,
    map_size: &[i32;2]
) -> HashMap<i32,[i32;2]>{
    const DISTANCE_MULTIPLIER: i32 = 1;
    let mut node_coords: HashMap<i32,[i32;2]> = HashMap::new();
    let coord_a = [0,0];
    node_coords.insert(1, coord_a);
    match node_order.len(){ // Dealing with special cases.
        0 | 1 => {  node_coords.insert(1, coord_a);; // Note: Car will allways have at least one node.
                    return node_coords;
                },
        2 => {
                let node_a = node_order[0];
                let node_b = node_order[1];
                let dist_a_b = a_to_b_time(0, node_a, node_b, travel_costs).expect("We know this connection exis");
                node_coords.insert(1, coord_a);
                node_coords.insert(2, [0,dist_a_b]);
                return node_coords;
            },
        _ => () // If no special case, continue as normal.
    }
    let node_a = node_order[0];
    let node_b = node_order[1];
    let node_c = node_order[2];
    let dist_a_b = match a_to_b_time(0, node_a, node_b, travel_costs){
        Some(dist) => DISTANCE_MULTIPLIER * dist,
        None => panic!("First 3 nodes should be guaranteed to work. Here a->b failed.")
    };
    let dist_a_c = match a_to_b_time(0, node_a, node_c, travel_costs){
        Some(dist) => DISTANCE_MULTIPLIER * dist,
        None => panic!("First 3 nodes should be guaranteed to work. Here a->c failed.")
    };
    let dist_b_c = match a_to_b_time(0, node_b, node_c, travel_costs){
        Some(dist) => DISTANCE_MULTIPLIER * dist,
        None => panic!("First 3 nodes should be guaranteed to work. Here b->c failed.")
    };

    let coord_b = [0,dist_a_b];
    node_coords.insert(2, coord_b);
    let coord_abc_options = coord_options_from_two_points(coord_a, coord_b, dist_a_b, dist_a_c, dist_b_c);
    let coord_c  = match coord_abc_options{
        [Some(coord_c), _] => coord_c,
        [_, Some(coord_d)] => coord_d,
        [None,None] => {return node_coords}
    };
    node_coords.insert(3, coord_c);
    let mut used_coords = Vec::<[i32;2]>::with_capacity(node_order.len());
    used_coords.push(coord_a);
    used_coords.push(coord_b);
    used_coords.push(coord_c);
    // Go through nodes and use 3 determined coordinates to find the nth coordinate.
    for &node_n in node_order.iter(){
        let dist_a_n = match a_to_b_time(0, node_a, node_n, travel_costs){
            Some(dist) => DISTANCE_MULTIPLIER * dist,
            None => continue
        };
        let dist_b_n = match a_to_b_time(0, node_b, node_n, travel_costs){
            Some(dist) => DISTANCE_MULTIPLIER * dist,
            None => continue
        };
        let dist_c_n = match a_to_b_time(0, node_c, node_n, travel_costs){
            Some(dist) => DISTANCE_MULTIPLIER * dist,
            None => continue
        };
        // Make function to calculate coords.
        let coord_abn_options = coord_options_from_two_points(coord_a, coord_b, dist_a_b, dist_a_n, dist_b_n);
        let coord_acn_options = coord_options_from_two_points(coord_a, coord_c, dist_a_c, dist_a_n, dist_c_n);
        let coord_options = [coord_abn_options[0], coord_abn_options[1], coord_acn_options[0], coord_acn_options[1]];
        let mut best_choice : Option<[i32;2]> = None;
        for &coord_option in coord_options.iter() {
            let mut coord = match coord_option{
                Some(v) => v,
                None => continue,
            };
            for i in 0..=1{
                if coord[i] < 0{
                    coord[i] += map_size[i];
                }
            }
            if best_choice.is_none() || used_coords.contains(&best_choice.unwrap()) && !used_coords.contains(&coord){
                best_choice = Some(coord);
            }
        }
        match best_choice{
            Some(v) => {
                used_coords.push(v);
                node_coords.insert(node_n, v);
            },
            None => ()
        }
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