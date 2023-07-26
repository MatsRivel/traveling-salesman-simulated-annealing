use std::collections::HashMap;

use crate::constants::{NCALLS, SOLUTION_SIZE};
use crate::validity_check::{a_to_b_time, deconstruct_call};

pub fn _correct_overcalling(solution :&[i32;SOLUTION_SIZE]) -> [i32;SOLUTION_SIZE]{
    /*
    Takes a solution where some calls occur more than twice.
    Returns a solution where calls occur no more than twice. 
    */
    let mut counter = [0i32;NCALLS];
    let mut output = *solution;
    let mut open_indices:Vec<usize> = Vec::with_capacity(20); // Arbitrary capacity, but better than 1.
    // Find calls occuring more than twice. 
    for (idx, call) in solution.iter().enumerate(){
        match counter[*call as usize]{
            ..=1 => {   
                    counter[*call as usize] += 1;
                    },
            2.. => {
                    open_indices.push(idx);
                    }
        }
    };
    // Replace over-represented calls with under-represented calls.
    for c in counter.iter(){
        let mut count = *c;
        while count < 2{
            count -= 1;
            match open_indices.pop(){
                Some(idx) => output[idx] = count,
                None => break,
            }
        } 
    }
    output

}
// TODO: Make func that goes through and tracks all cars, not just one. The earliest available car should get the first available call.
pub fn _correct_pickup_too_late(vehicle_idx:usize, call:i32, arrival_time:i32, start_upper:i32, travel_costs: &HashMap<(i32,i32,i32),(i32,i32)>,
                             call_details : &[[i32;8usize]; NCALLS], solution :&[i32;SOLUTION_SIZE]) -> [i32;SOLUTION_SIZE]{

    // Get the 1st occurence of the call (the pick-up)
    let initial_call_idx = solution.clone()
                                        .iter()
                                        .position(|&x| x==call)
                                        .expect("\"correct_pickup_too_late(...)\": This unwrap should be unfalable, as this func is only called when the given call exists."); 
    let mut call_idx = initial_call_idx;
    let mut current_time = arrival_time;
    // Find the latest possible spot where picking up the call is possible.
    while current_time > start_upper && call_idx > 0{
        match call_idx{
            2.. => {}, // Continue as normal.
            _  => { //TODO: Issue occurs here due to only considering *one* car, which can never be fast enough to deal with all calls.
                println!("\nVehicle idx: {}, call: {}, arrival_time: {}, start_upper {}\nsolution: {:?}\n",vehicle_idx, call, arrival_time, start_upper, solution);
                panic!("First call is \"picked up too late\". Issues with cars?")
            } // First call called cant possible be picked up *too late*???
        }
        call_idx -= 1;
        let (_, from_node, _, _, _, _, _, _) = deconstruct_call(solution[call_idx],call_details);
        let (to_node, _, _, _, _, _, _, _) = deconstruct_call(solution[call_idx+1],call_details);

        println!("\nFrom node: {} To node:{}\n",solution[call_idx],solution[call_idx+1]);
        match (from_node, to_node) {
            (_,0) | (0, _) => panic!("Node-numbers can not be zero! Will cause overflow to INTMAX"),
            _ => ()
        }
        println!("________________ correct_pickup_too_late ________________");
        let call_time:i32 = a_to_b_time(vehicle_idx,from_node,to_node,travel_costs).expect("Can not fail, as only legal moves are considered."); // Get the cost of going from idx to idx+1.
        println!("Call_time: {}",call_time);
        current_time -= call_time;
    }
    // Move all calls from the desired position to the original position one step ahead, effective deleting the original call at its original position.
    let mut output:[i32;SOLUTION_SIZE] = *solution;
    for insert_idx in ((call_idx+1)..(initial_call_idx+1)).rev() { // The range [initial_call_idx, call_idx+1]
        output[insert_idx] = output[insert_idx -1];
    }
    // Then insert the desired call at the desired location. This overwrites the old call, which has been moved one step ahead.
    // Calls below this point should not be affected.
    output[call_idx] = call;

    assert_ne!(&output, solution);

    output

}
fn _get_nth_occurrences(n_occurrence:usize, call:i32, solution:&[i32;SOLUTION_SIZE]) -> Option<usize>{
    solution.iter()
        .enumerate()
        .filter(|(_,val)| *val == &call)
        .map(|(idx,_)|idx)
        .nth(n_occurrence-1)
}
pub fn _correct_deliver_too_late(vehicle_idx:usize, call:i32, arrival_time:i32, end_upper:i32, travel_costs: &HashMap<(i32,i32,i32),(i32,i32)>, solution :&[i32;SOLUTION_SIZE]) -> [i32;SOLUTION_SIZE]{
    // Get the 2nd occurence of the call (the delivery)
    let initial_call_idx = _get_nth_occurrences(2usize,call,solution).expect("\"correct_deliver_too_late(...)\": This unwrap should be unfalable, as this func is only called when the given call exists."); 
    let mut call_idx = initial_call_idx;
    let mut current_time = arrival_time;
    // Find the latest possible spot where picking up the call is possible.
    while current_time > end_upper{
        call_idx -= 1;
        let call_time:i32 = a_to_b_time(vehicle_idx,solution[call_idx],solution[call_idx+1],travel_costs).expect("Unwrap can not fail, as all moves havea legal start and end."); // Get the cost of going from idx to idx+1.
        current_time -= call_time;
    }
    // Move all calls from the desired position to the original position one step ahead, effective deleting the original call at its original position.
    let mut output:[i32;SOLUTION_SIZE] = *solution;
    for insert_idx in ((call_idx+1)..(initial_call_idx+1)).rev() { // The range [initial_call_idx, call_idx+1]
        output[insert_idx] = output[insert_idx -1];
    }
    // Then insert the desired call at the desired location. This overwrites the old call, which has been moved one step ahead.
    // Calls below this point should not be affected.
    output[call_idx] = call;
    output
}
