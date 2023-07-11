use crate::correctors::{correct_overcalling,correct_pickup_too_late,correct_deliver_too_late};
use crate::validity_check::{correctness_check,CorrectnessError, deconstruct_vehicle, deconstruct_call, deconstruct_node, a_to_b_time, a_to_b_cost};
use crate::constants::{NVEHICLES,NCALLS, TRAVEL_TIME_SIZE,SOLUTION_SIZE};
use rand::{thread_rng, Rng};
use std::cmp::{min, max};
use std::fmt::format;
use std::io;
use std::thread::current;
use itertools::{Itertools, enumerate};
use permutator::{Permutation, HeapPermutationIterator};
use std::time::{Duration, Instant};
use std::collections::HashMap;
use std;
fn swap_call_with_random(call_idx:usize,solution:&[i32;SOLUTION_SIZE]) -> [i32;SOLUTION_SIZE]{
    let mut output: [i32;SOLUTION_SIZE] = solution.clone();
    let other_idx = thread_rng().gen_range(0..(SOLUTION_SIZE));
    output.swap(call_idx,other_idx);
    return output;
}
pub fn swap_two_random_calls(solution:&[i32;SOLUTION_SIZE]) -> [i32;SOLUTION_SIZE]{
    let call_idx = thread_rng().gen_range(0..(SOLUTION_SIZE));
    swap_call_with_random(call_idx,solution)
}
fn swap_three_random_calls(solution:&[i32;SOLUTION_SIZE])  -> [i32;SOLUTION_SIZE]{
    let mut output: [i32;SOLUTION_SIZE] = solution.clone();
    let call_idx = thread_rng().gen_range(0..(SOLUTION_SIZE));
    swap_call_with_random(call_idx, solution);
    swap_call_with_random(call_idx, solution);
    return output;
}
pub fn brute_force_solve(
    vehicle_details : &[[i32;3usize]; NVEHICLES],
    call_details : &[[i32;8usize]; NCALLS],
    travel_costs: &[[i32;4usize];TRAVEL_TIME_SIZE],
    node_costs: &[[i32;5usize];NCALLS*NVEHICLES]
    ) -> [i32;SOLUTION_SIZE]{
        /*This solver ran for 17 hours and could still only finish about ~1% of the space. */
    println!("\tBruteFocre\t");
    let start = Instant::now();
    let mut solutions_tried:i64 = 0;
    let max_solutions = (1..=SOLUTION_SIZE).product::<usize>();
    println!("Trying {} solutions...",max_solutions);
    let mut one_percent_of_max = (max_solutions / 10000) as i64;
    if one_percent_of_max <= 0i64{
        one_percent_of_max = 1i64;
    } 
    let mut current_solution: [i32;SOLUTION_SIZE] = [0i32;SOLUTION_SIZE];
    for i in 0..NCALLS{
        current_solution[i*2] = i as i32 +1;
        current_solution[i*2+1] = i as i32 +1;
    }
    // Make custom iterator to make all combinations of [0,0, ..., NCALLS,NCALLS] to [NCALLS,NCALLS,..., 0,0], and everything in-between.
    let mut call_vec:Vec<usize> = Vec::with_capacity(SOLUTION_SIZE);
    for i in 1..=NCALLS{
        call_vec.push(i);
        call_vec.push(i);
    }
    let solution_iterator_main = call_vec.permutation();

    for possibility in solution_iterator_main{
        solutions_tried += 1;
        if solutions_tried % one_percent_of_max as i64 == 0{
            println!("{}% in {}sec", (solutions_tried as f64) / (one_percent_of_max as f64), start.elapsed().as_secs())
        }
        for (index, element) in possibility.iter().enumerate(){
            current_solution[index] = *element as i32;
        match correctness_check(&current_solution, vehicle_details, call_details, travel_costs, node_costs) {
            Ok(_val) =>{
                println!("\n\n Correct!\nIt took {}s \t\t", start.elapsed().as_secs());
                return current_solution;}
            Err(_err) => (),
            }
        }
    }
    println!("{} solutions tried, out of theoretical {} possible", solutions_tried, max_solutions);
    todo!("\nWhat if no possible solution exists?\n");
}

pub fn generate_any_valid_solution(
    vehicle_details : &[[i32;3usize]; NVEHICLES],
    call_details : &[[i32;8usize]; NCALLS],
    travel_costs: &[[i32;4usize];TRAVEL_TIME_SIZE],
    node_costs: &[[i32;5usize];NCALLS*NVEHICLES]
    ) -> [i32;SOLUTION_SIZE]{

    let mut current_solution:[i32;SOLUTION_SIZE] = [0i32;SOLUTION_SIZE];
    for i in 0..NCALLS{
        current_solution[i*2] = i as i32 +1;
        current_solution[i*2+1] = i as i32 +1;
    }
    let mut counter = 0;
    loop{
        let mut dummy_input = "".to_string();
        match io::stdin().read_line(&mut dummy_input) {
            Ok(_) => (),
            Err(_) => break
        }
        if counter % 100000 == 0{ // Make a dot every 10k and line every 100k loop  to track actual progress.
            println!("| {}",counter);
        }else if counter % 10000 == 0{
            print!(".");
        }
        counter += 1;
        let correctness_result = correctness_check(&current_solution, vehicle_details, call_details, travel_costs, node_costs);
        match correctness_result{
            Ok(_) => return current_solution,
            Err(_) => {//println!("{:?}",&current_solution);

            current_solution = swap_two_random_calls(&current_solution)}
        };
    }

    loop{
        let correctness_result = correctness_check(&current_solution, vehicle_details, call_details, travel_costs, node_costs);
        let current_solution = match correctness_result{
            Ok(_total_cost) => return current_solution,
            Err(CorrectnessError::CallTooManyTimes { .. }) => correct_overcalling(&current_solution),                                    // Severe error: A function is not making balances changes. Recoverable, but there is a bug.
            Err(CorrectnessError::DeliverTooLate { vehicle_idx, call, car_time, arrival_time, end_upper, .. }) =>{ 
                correct_deliver_too_late(
                    vehicle_idx,
                    call,
                    arrival_time,
                    end_upper,
                    travel_costs,
                    &current_solution
                )
            },                  // Bad solution. Expected until the first valid solution occurs. Recoverable. 
            Err(CorrectnessError::Overloaded { call, car_weight, loaded_weight, car_capacity, .. }) => todo!(),                 // Bad solution. Expected until the first valid solution occurs. Recoverable. 
            Err(CorrectnessError::PickUpTooLate { vehicle_idx, call, car_time, arrival_time, start_upper, .. }) =>{ 
                correct_pickup_too_late(
                    vehicle_idx,
                    call,
                    arrival_time,
                    start_upper,
                    travel_costs,
                    call_details,
                    &current_solution
                )
            },                  // Bad solution. Expected until the first valid solution occurs. Recoverable. 
            Err(CorrectnessError::PickupNotDelivered { idx, .. }) => todo!(),                                                                // Bad solution. Expected until the first valid solution occurs. Recoverable.
            Err(e) => panic!("Unrecoverable error. Input is likely fundamentally flawed.\n{:?}",e)                                   // Fatal error: Input is flawed. No recovery possible.
        };
    }
}

struct Info{
    to_node:i32,
    total_time:i32,
    time_delta:i32,
    upper_bound:i32,
    total_cost:i32

}
impl Info{
    fn new(call_count: i32, vehicle_idx:usize, car_pos:[i32;NVEHICLES], car_times:[i32;NVEHICLES], call:i32,node_costs:&[[i32;5usize];NCALLS*NVEHICLES], call_details:&[[i32;8usize]; NCALLS],travel_costs:&[[i32;4usize];TRAVEL_TIME_SIZE]) -> Option<Self>{
        let from_node = car_pos[vehicle_idx];
        let current_time = car_times[vehicle_idx];
        let (mut node_time, mut node_cost, mut to_node, mut _call_size, mut cost_of_not_deliver, mut lower_bound, mut upper_bound) = (0i32,0i32,0i32,0i32, 0i32, 0i32, 0i32); 
        match call_count{
            0 => {
                    (node_time, node_cost, _, _) = deconstruct_node(vehicle_idx, (call) as i32, node_costs);
                    (to_node, _, _call_size, cost_of_not_deliver, lower_bound, upper_bound, _, _) = deconstruct_call(call as i32, call_details);
                },
            1 => {
                    (_, _, node_time, node_cost) = deconstruct_node(vehicle_idx, (call) as i32, node_costs);
                    (_, to_node, _call_size, cost_of_not_deliver, _, _, lower_bound, upper_bound) = deconstruct_call(call as i32, call_details);
                },
            _ => return None
        }
        let total_cost = node_cost + cost_of_not_deliver;
        let potential_time = current_time + node_time + a_to_b_time(vehicle_idx, from_node, to_node, travel_costs);
        let total_time = max(lower_bound, potential_time);
        let time_delta = (total_time - lower_bound).abs();
        return Some(Info{to_node, total_time, time_delta, upper_bound, total_cost});
    }
    fn get_info(&self) -> (Option<i32>,Option<i32>,Option<i32>, Option<i32>){
        return (Some(self.to_node), Some(self.total_time), Some(self.time_delta), Some(self.total_cost));
    }
    fn is_faster(&self, cheapest_time_delta:Option<i32>) -> bool {
        if self.total_time <= self.upper_bound{
            match cheapest_time_delta{
                None => return true, // If not value, assign it.
                Some(some_cheapest_time_delta)=> { // If value, compare. If cost < previous cost, assign.
                    if self.time_delta <= some_cheapest_time_delta{
                        return true
                    }
                }
            }
        }
        return false
    }

}

pub fn naive_solve(
    vehicle_details : &[[i32;3usize]; NVEHICLES],
    call_details : &[[i32;8usize]; NCALLS],
    travel_costs: &[[i32;4usize];TRAVEL_TIME_SIZE],
    node_costs: &[[i32;5usize];NCALLS*NVEHICLES]
    ) -> [i32;SOLUTION_SIZE]{
    let mut total_cost:i32 = 0;
    let mut current_solution:[i32;SOLUTION_SIZE] = [0i32;SOLUTION_SIZE];
    // (Call, IsPickUp): (vehicle_index, cost)
    let mut car_pos = [0i32;NVEHICLES];
    let mut car_times = [0i32;NVEHICLES];
    for vehicle_idx in 0..NVEHICLES{
        let (vehicle_home, vehicle_start, _ ) = deconstruct_vehicle(vehicle_idx, vehicle_details);
        car_pos[vehicle_idx] = vehicle_home;
        car_times[vehicle_idx] = vehicle_start;
    }

    let mut call_counter = [0;NCALLS];
    for solution_idx in 0..SOLUTION_SIZE{
        let mut cheapest_vehicle:Option<usize> = None;
        let mut cheapest_call:Option<usize> = None;
        let mut cheapest_cost:Option<i32> = None;
        let mut cheapest_time:Option<i32> = None;
        let mut cheapest_to_node:Option<i32> = None;
        let mut cheapest_time_delta:Option<i32> = None;
        for vehicle_idx in 0..NVEHICLES{
            //println!("Vehicle: {}, Pos: {}",vehicle_idx+1, car_pos[vehicle_idx]);
            for call in 1..=NCALLS{// We're picking up the call:
                //println!("Call count: {:?}, call: {}", call_counter, call);
                let possible_informant:Option<Info> = Info::new(call_counter[call-1], vehicle_idx, car_pos, car_times, call as i32,node_costs, call_details,travel_costs);
                let informant = match possible_informant {
                    Some(v) => v,
                    None => continue // If no valid data, leave
                };
                if informant.is_faster(cheapest_time_delta) || cheapest_call.is_none(){
                    cheapest_vehicle = Some(vehicle_idx);
                    if cheapest_call.is_some(){
                        call_counter[cheapest_call.unwrap()-1] -= 1;
                    }
                    cheapest_call = Some(call);
                    call_counter[call-1] += 1;
                    cheapest_cost = Some(0); // TODO: Temporary 0. Should be removed later, when we care about cost.
                    (cheapest_to_node, cheapest_time, cheapest_time_delta, cheapest_cost) = informant.get_info();
                }
            }
        }
        //println!("{:?}",(cheapest_call,cheapest_vehicle,cheapest_to_node,cheapest_time));
        match (cheapest_call,cheapest_vehicle,cheapest_to_node,cheapest_time, cheapest_cost) {
            (Some(call_value),Some(vehicle_value),Some(to_node_value),Some(time_value), Some(cost_value)) => {
                current_solution[solution_idx] = call_value as i32;
                car_times[vehicle_value] = time_value;
                car_pos[vehicle_value] = to_node_value;
                total_cost += cost_value;
            }
            _ => panic!("\n ### No valid solution! ###")
        }
        total_cost += cheapest_cost.unwrap(); // Already checked that this valie is Some(v), so .unwrap() is safe.
        //println!("Current solution: {:?}\nCXurrent total cost: {}",current_solution, total_cost );
    }
    println!("\nnFinal solution: {:?}\n### Total Cost: {} ###",current_solution, total_cost );
    return current_solution;

}