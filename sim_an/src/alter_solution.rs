use crate::validity_check::{deconstruct_vehicle, deconstruct_call, deconstruct_node, a_to_b_time, _correctness_check};
use crate::constants::{NVEHICLES,NCALLS, SOLUTION_SIZE, MAX_RUNTIME_IN_SECONDS};

use rand::prelude::Distribution;
use rand::{thread_rng, Rng};
use std::cmp::{max, Ordering};
use permutator::Permutation;
use std::time::Instant;
use std::collections::HashMap;


struct SolutionPermutation<'a>{
    current_solution: &'a [i32;SOLUTION_SIZE],
    current_cost : &'a i32,
    vehicle_details : &'a [[i32;3usize]; NVEHICLES],
    call_details : &'a [[i32;8usize]; NCALLS],
    travel_costs: &'a HashMap<(i32,i32,i32),(i32,i32)>,
    node_costs: &'a [[i32;5usize];NCALLS*NVEHICLES]
}

impl SolutionPermutation<'_> {
    fn improve(self) -> ([i32;SOLUTION_SIZE], i32){
        let start = Instant::now();
        const WEIGHTS_LEN: usize = 3usize; // Just here so we can match it later.
        let weights :[i32;WEIGHTS_LEN]= [10,8,1];
        let distribution = rand::distributions::WeightedIndex::new(weights).expect("Distributing weights failed. No weights provided?");
        let mut rng = thread_rng();
        let selected_idx = distribution.sample(&mut rng);


        let mut best_solution = *self.current_solution;
        let mut best_solution_cost = Some(*self.current_cost);
        #[allow(clippy::absurd_extreme_comparisons)] // This is here in case you set MAX_RUNTIME_IN_SECONDS to 0.
        while start.elapsed().as_secs() < MAX_RUNTIME_IN_SECONDS{
            let (new_solution, mut new_solution_cost) = match selected_idx{
                0 => (swap_two_random_calls(0usize, &best_solution), None),
                1 => (swap_three_random_calls(0usize, &best_solution), None),
                2 =>{let (sol, val) = swap_most_expensive_calls(&best_solution,
                                                                                &best_solution_cost.expect("Should not fail, as we never allow a None value to become the best value."),
                                                                                self.vehicle_details,
                                                                                self.call_details,
                                                                                self.travel_costs,
                                                                                self.node_costs);
                    (sol, Some(val))
                },// <- outputs this value to the "let" function.
                WEIGHTS_LEN.. => panic!("Not enough weights"),
                _ => panic!("This should not be possible to reach, as the last match point covers any remaining value...")
            };
            if new_solution_cost.is_none(){
                new_solution_cost = calculate_cost(&new_solution, self.vehicle_details, self.call_details, self.travel_costs, self.node_costs);
            }

            if new_solution_cost.is_some() && (best_solution_cost.is_none() || new_solution_cost.unwrap() < best_solution_cost.unwrap()){
                best_solution = new_solution;
                best_solution_cost = new_solution_cost;
            }
        }
        (best_solution, best_solution_cost.expect("Should not fail, as we never allow None value to become the best value."))
    }
}

pub fn semi_random_improve_solution(
        current_solution: &[i32;SOLUTION_SIZE],
        current_cost: &i32,
        vehicle_details : &[[i32;3usize]; NVEHICLES],
        call_details : &[[i32;8usize]; NCALLS],
        travel_costs: &HashMap<(i32,i32,i32),(i32,i32)>,
        node_costs: &[[i32;5usize];NCALLS*NVEHICLES]
    ) -> Option<([i32;SOLUTION_SIZE],i32)>{
    
    let sol_perm = SolutionPermutation{current_solution, current_cost, vehicle_details, call_details, travel_costs, node_costs};
    let (new_sol, new_cost) = sol_perm.improve();
    Some((new_sol, new_cost))
        
}

fn get_move_costs(
    solution: &[i32;SOLUTION_SIZE],
    vehicle_details : &[[i32;3usize]; NVEHICLES],
    call_details : &[[i32;8usize]; NCALLS],
    travel_costs: &HashMap<(i32,i32,i32),(i32,i32)>,
    node_costs: &[[i32;5usize];NCALLS*NVEHICLES]
    ) -> Option<[Option<(i32,usize)>;SOLUTION_SIZE]>{

    let mut car_pos = [0i32;NVEHICLES];
    let mut car_times = [0i32;NVEHICLES];
    let mut cost_order: [Option<(i32,usize)>; SOLUTION_SIZE] = [None;SOLUTION_SIZE];
    for vehicle_idx in 0..NVEHICLES{
        let (vehicle_home, vehicle_start, _ ) = deconstruct_vehicle(vehicle_idx, vehicle_details);
        car_pos[vehicle_idx] = vehicle_home;
        car_times[vehicle_idx] = vehicle_start;
    }
    let mut call_counter = [0;NCALLS];
    for (call_idx, &call) in solution.iter().enumerate(){
        let mut cheapest_vehicle:Option<usize> = None;
        let mut cheapest_call:Option<usize> = None;
        let mut cheapest_cost:Option<i32> = None;
        let mut cheapest_time:Option<i32> = None;
        let mut cheapest_to_node:Option<i32> = None;
        let mut cheapest_time_delta:Option<i32> = None;
        let mut car_info = CarInfo {vehicle_idx:0, car_pos, car_times};
        for vehicle_idx in 0..NVEHICLES{
            car_info.vehicle_idx = vehicle_idx;
            let possible_informant:Option<Info> = Info::new(call_counter[call as usize-1], &car_info, call,node_costs, call_details,travel_costs);
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
                cost_order[call_idx] = Some((cost_value, call_idx));
            }
            _ => return None //No valid solution!
        }
    }
    Some(cost_order)
}

fn sort_call_by_cost(
    solution: &[i32;SOLUTION_SIZE],
    vehicle_details : &[[i32;3usize]; NVEHICLES],
    call_details : &[[i32;8usize]; NCALLS],
    travel_costs: &HashMap<(i32,i32,i32),(i32,i32)>,
    node_costs: &[[i32;5usize];NCALLS*NVEHICLES]
    ) -> [Option<(i32, usize)>; SOLUTION_SIZE] {
    
    let mut call_costs = get_move_costs(solution, vehicle_details, call_details, travel_costs, node_costs).expect("No solution to unwrap");
    call_costs.sort_by(|a,b| {
        match (a,b){
            (None,None) => Ordering::Equal,
            (_,None) => Ordering::Greater,
            (None,_) => Ordering::Less,
            (Some((x1,_)),Some((y1,_))) => x1.cmp(y1)
        }
    });
    call_costs
}



pub fn swap_most_expensive_calls(    
    current_solution: &[i32;SOLUTION_SIZE],
    current_cost: &i32,
    vehicle_details : &[[i32;3usize]; NVEHICLES],
    call_details : &[[i32;8usize]; NCALLS],
    travel_costs: &HashMap<(i32,i32,i32),(i32,i32)>,
    node_costs: &[[i32;5usize];NCALLS*NVEHICLES]
    ) -> ([i32;SOLUTION_SIZE],i32){
    let mut new_solution = *current_solution;//current_solution.clone();
    let sorted_call_costs = sort_call_by_cost(current_solution, vehicle_details, call_details, travel_costs, node_costs);
    let mut total_cost = *current_cost;

    if sorted_call_costs[0].is_none() || sorted_call_costs[1].is_none(){
        return (new_solution, total_cost);
    }
    let mut temp_solution = new_solution; //new_solution.clone();
    'outer: for i in 0..(SOLUTION_SIZE-1) {
        for j in (i+1)..SOLUTION_SIZE {
            match (sorted_call_costs[i], sorted_call_costs[j]) {
                (None, _) | (_, None)=> break 'outer, // If "None", no more valid pairs exist, so break the outer loop. 
                (Some(pair_a), Some(pair_b)) => (pair_a.1, pair_b.1),
            };
            temp_solution.swap(i, j);
            let temp_cost = match calculate_cost(&new_solution, vehicle_details, call_details, travel_costs, node_costs){
                None => continue, // If solution is not valid, try next permutation.
                Some(val) => val
            };
            if temp_cost <= total_cost {
                total_cost = temp_cost;
                new_solution = temp_solution;
            }
        }
    }
    (new_solution, total_cost)

}

#[allow(dead_code)]
type SwapperFunction = fn(usize, &[i32; SOLUTION_SIZE]) -> [i32; SOLUTION_SIZE];
pub fn _randomly_improve_solution(
    current_solution: &[i32;SOLUTION_SIZE],
    _current_cost: &i32,
    vehicle_details : &[[i32;3usize]; NVEHICLES],
    call_details : &[[i32;8usize]; NCALLS],
    travel_costs: &HashMap<(i32,i32,i32),(i32,i32)>,
    node_costs: &[[i32;5usize];NCALLS*NVEHICLES]
    ) -> Option<([i32;SOLUTION_SIZE],i32)>{
    let start = Instant::now();
    let function_list: [SwapperFunction; 3] = [swap_call_with_random, swap_two_random_calls, swap_three_random_calls];
    let weights = [3,2,1];
    let distribution = rand::distributions::WeightedIndex::new(weights).expect("Distributing weights failed. No weights provided?");
    let mut rng = thread_rng();
    let selected_function = function_list[distribution.sample(&mut rng)];
    let mut best_solution = selected_function(0usize,current_solution);
    let mut best_solution_cost: Option<i32> = calculate_cost(&best_solution, vehicle_details, call_details, travel_costs, node_costs);

    #[allow(clippy::absurd_extreme_comparisons)] // This is here in case you set MAX_RUNTIME_IN_SECONDS to 0.
    while start.elapsed().as_secs() < MAX_RUNTIME_IN_SECONDS{
        let new_solution = selected_function(0usize,current_solution);
        let new_solution_cost= calculate_cost(&best_solution, vehicle_details, call_details, travel_costs, node_costs);
        if new_solution_cost.is_some() && (new_solution_cost < best_solution_cost || best_solution_cost.is_none()){
            best_solution = new_solution;
            best_solution_cost = new_solution_cost;
        }
    }
    best_solution_cost.map(|cost| (best_solution, cost))
}

fn swap_call_with_random(call_idx:usize,solution:&[i32;SOLUTION_SIZE]) -> [i32;SOLUTION_SIZE]{
    let mut output: [i32;SOLUTION_SIZE] = *solution;//solution.clone();
    let other_idx = thread_rng().gen_range(0..(SOLUTION_SIZE));
    output.swap(call_idx,other_idx);
    output
}

pub fn swap_two_random_calls(_call_idx:usize, solution:&[i32;SOLUTION_SIZE]) -> [i32;SOLUTION_SIZE]{
    let call_idx = thread_rng().gen_range(0..(SOLUTION_SIZE));
    swap_call_with_random(call_idx,solution)
}
fn swap_three_random_calls(_call_idx:usize, solution:&[i32;SOLUTION_SIZE])  -> [i32;SOLUTION_SIZE]{
    let output: [i32;SOLUTION_SIZE] = *solution;
    let call_idx = thread_rng().gen_range(0..(SOLUTION_SIZE));
    swap_call_with_random(call_idx, solution);
    swap_call_with_random(call_idx, solution);
    output
}
pub fn _brute_force_solve(
    vehicle_details : &[[i32;3usize]; NVEHICLES],
    call_details : &[[i32;8usize]; NCALLS],
    travel_costs: &HashMap<(i32,i32,i32),(i32,i32)>,
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
        if solutions_tried % one_percent_of_max== 0{
            println!("{}% in {}sec", (solutions_tried as f64) / (one_percent_of_max as f64), start.elapsed().as_secs())
        }
        for (index, element) in possibility.iter().enumerate(){
            current_solution[index] = *element as i32;
        match _correctness_check(&current_solution, vehicle_details, call_details, travel_costs, node_costs) {
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

pub struct CarInfo{
    pub vehicle_idx:usize,
    pub car_pos:[i32;NVEHICLES],
    pub car_times:[i32;NVEHICLES]
}
pub struct Info{
    to_node:i32,
    total_time:i32,
    time_delta:i32,
    upper_bound:i32,
    total_cost:i32

}
impl Info{
    pub fn new(call_count: i32, car_info:&CarInfo, call:i32, node_costs:&[[i32;5usize];NCALLS*NVEHICLES], call_details:&[[i32;8usize]; NCALLS],travel_costs:&HashMap<(i32,i32,i32),(i32,i32)>) -> Option<Self>{
        let car_pos = car_info.car_pos;
        let car_times = car_info.car_times;
        let vehicle_idx = car_info.vehicle_idx;

        let from_node = car_pos[vehicle_idx];
        let current_time = car_times[vehicle_idx];
        let (node_time, node_cost, to_node, _call_size, cost_of_not_deliver, lower_bound, upper_bound);
        match call_count{
            0 => {
                    (node_time, node_cost, _, _) = deconstruct_node(vehicle_idx, call, node_costs);
                    (to_node, _, _call_size, cost_of_not_deliver, lower_bound, upper_bound, _, _) = deconstruct_call(call, call_details);
                },
            1 => {
                    (_, _, node_time, node_cost) = deconstruct_node(vehicle_idx, call, node_costs);
                    (_, to_node, _call_size, cost_of_not_deliver, _, _, lower_bound, upper_bound) = deconstruct_call(call, call_details);
                },
            _ => return None
        }
        let total_cost = node_cost + cost_of_not_deliver;
        let ab_time =  match a_to_b_time(vehicle_idx, from_node, to_node, travel_costs){
            Some(v) => v,
            None => return None
        };
        let potential_time = current_time + node_time + ab_time; 
        let total_time = max(lower_bound, potential_time);
        let time_delta = (total_time - lower_bound).abs();
        Some(Info{to_node, total_time, time_delta, upper_bound, total_cost})

    }
    pub fn get_info(&self) -> (Option<i32>,Option<i32>,Option<i32>, Option<i32>){
        (Some(self.to_node), Some(self.total_time), Some(self.time_delta), Some(self.total_cost))
    }
    pub fn is_faster(&self, cheapest_time_delta:Option<i32>) -> bool {
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
        false
    }

}
fn calculate_cost(
    solution: &[i32;SOLUTION_SIZE],
    vehicle_details : &[[i32;3usize]; NVEHICLES],
    call_details : &[[i32;8usize]; NCALLS],
    travel_costs: &HashMap<(i32,i32,i32),(i32,i32)>,
    node_costs: &[[i32;5usize];NCALLS*NVEHICLES]
) -> Option<i32>{
    let mut total_cost:i32 = 0;
    let mut car_pos = [0i32;NVEHICLES];
    let mut car_times = [0i32;NVEHICLES];
    for vehicle_idx in 0..NVEHICLES{
        let (vehicle_home, vehicle_start, _ ) = deconstruct_vehicle(vehicle_idx, vehicle_details);
        car_pos[vehicle_idx] = vehicle_home;
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
                total_cost += cost_value;
            }
            _ => return None //No valid solution!
        }
        total_cost += cheapest_cost.unwrap(); // Already checked that this valie is Some(v), so .unwrap() is safe.
    }
    Some(total_cost)

}
pub fn naive_solve(
    vehicle_details : &[[i32;3usize]; NVEHICLES],
    call_details : &[[i32;8usize]; NCALLS],
    travel_costs: &HashMap<(i32,i32,i32),(i32,i32)>,
    node_costs: &[[i32;5usize];NCALLS*NVEHICLES]
    ) -> ([i32;SOLUTION_SIZE],i32){
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
    #[allow(clippy::needless_range_loop)]
    for solution_idx in 0..SOLUTION_SIZE{
        let mut cheapest_vehicle:Option<usize> = None;
        let mut cheapest_call:Option<usize> = None;
        let mut cheapest_cost:Option<i32> = Some(i32::MAX);
        let mut cheapest_time:Option<i32> = Some(i32::MAX);
        let mut cheapest_to_node:Option<i32> = None;
        let mut cheapest_time_delta:Option<i32> = Some(i32::MAX);
        let mut car_info = CarInfo{vehicle_idx:0, car_pos, car_times};
        for vehicle_idx in 0..NVEHICLES{
            car_info.vehicle_idx = vehicle_idx;
            for call in 1..=NCALLS{// We're picking up the call:
                let possible_informant:Option<Info> = Info::new(call_counter[call-1], &car_info, call as i32,node_costs, call_details,travel_costs);
                let informant = match possible_informant {
                    Some(v) => v,
                    None => continue // If no valid data, leave
                };
                if informant.is_faster(cheapest_time_delta) || cheapest_call.is_none(){
                    cheapest_vehicle = Some(vehicle_idx);
                    
                    if let Some(i) = cheapest_call{
                        call_counter[i-1] -= 1;
                    }

                    cheapest_call = Some(call);
                    call_counter[call-1] += 1;
                    (cheapest_to_node, cheapest_time, cheapest_time_delta, cheapest_cost) = informant.get_info();
                }
            }
        }
        match (cheapest_call, cheapest_vehicle, cheapest_to_node, cheapest_time, cheapest_cost) {
            (Some(call_value),Some(vehicle_value),Some(to_node_value),Some(time_value), Some(cost_value)) => {
                current_solution[solution_idx] = call_value as i32;
                car_times[vehicle_value] = time_value;
                car_pos[vehicle_value] = to_node_value;
                total_cost += cost_value;
            }
            _ => panic!("\n ### No valid solution! ###") // One of the "cheapest" variables is "None", so no valid call-vehicle combo was found. Note: Likely bug, as the data is made to be possible.
        }
        total_cost += cheapest_cost.unwrap(); // Already checked that this valie is Some(v), so .unwrap() is safe.
    }
    (current_solution, total_cost)

}