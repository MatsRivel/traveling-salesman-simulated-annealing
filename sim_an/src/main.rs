mod constructors;
mod constants;
mod validity_check;

use validity_check::{correctness_check,CorrectnessError,a_to_b_time,deconstruct_call};
use constants::{NVEHICLES,NNODES,NCALLS};
use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;
use std::env;
use rand::thread_rng;
use rand::Rng;

fn swap_call_with_random(call_idx:usize,solution:&[i32;NCALLS*2]) -> [i32;NCALLS*2]{
    let mut output: [i32;NCALLS*2] = solution.clone();
    let other_idx = thread_rng().gen_range(0..(NCALLS*2));
    output.swap(call_idx,other_idx);
    return output;
}

fn swap_two_random_calls(solution:&[i32;NCALLS*2]) -> [i32;NCALLS*2]{
    let call_idx = thread_rng().gen_range(0..(NCALLS*2));
    swap_call_with_random(call_idx,solution)
}

fn swap_three_random_calls(solution:&[i32;NCALLS*2])  -> [i32;NCALLS*2]{
    let mut output: [i32;NCALLS*2] = solution.clone();
    let call_idx = thread_rng().gen_range(0..(NCALLS*2));
    swap_call_with_random(call_idx, solution);
    swap_call_with_random(call_idx, solution);
    return output;
}

fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where P: AsRef<Path>, {
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}

fn correct_overcalling(solution :&[i32;NCALLS*2]) -> [i32;NCALLS*2]{
    /*
    Takes a solution where some calls occur more than twice.
    Returns a solution where calls occur no more than twice. 
    */
    let mut counter = [0i32;NCALLS];
    let mut output = solution.clone();
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
    return output;

}


// TODO: Make func that goes through and tracks all cars, not just one. The earliest available car should get the first available call.
fn correct_pickup_too_late(vehicle_idx:usize, call:i32, arrival_time:i32, start_upper:i32, travel_costs: &[[i32;4usize];NNODES*NNODES*NVEHICLES],
                             call_details : &[[i32;8usize]; NCALLS], solution :&[i32;NCALLS*2]) -> [i32;NCALLS*2]{

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
            ..=1 | _  => { //TODO: Issue occurs here due to only considering *one* car, which can never be fast enough to deal with all calls.
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
        let call_time:i32 = a_to_b_time(vehicle_idx,from_node,to_node,travel_costs); // Get the cost of going from idx to idx+1.
        println!("Call_time: {}",call_time);
        current_time -= call_time;
    }
    // Move all calls from the desired position to the original position one step ahead, effective deleting the original call at its original position.
    let mut output:[i32;NCALLS*2] = solution.clone();
    for insert_idx in ((call_idx+1)..(initial_call_idx+1)).rev() { // The range [initial_call_idx, call_idx+1]
        output[insert_idx] = output[insert_idx -1];
    }
    // Then insert the desired call at the desired location. This overwrites the old call, which has been moved one step ahead.
    // Calls below this point should not be affected.
    output[call_idx] = call;

    assert_ne!(&output, solution);

    return output;

}

fn get_nth_occurrences(n_occurrence:usize, call:i32, solution:&[i32;NCALLS*2]) -> Option<usize>{
    solution.iter()
        .enumerate()
        .filter(|(_,val)| *val == &call)
        .map(|(idx,_)|idx)
        .nth(n_occurrence-1)
}

fn correct_deliver_too_late(vehicle_idx:usize, call:i32, arrival_time:i32, end_upper:i32, travel_costs: &[[i32;4usize];NNODES*NNODES*NVEHICLES], solution :&[i32;NCALLS*2]) -> [i32;NCALLS*2]{
    // Get the 2nd occurence of the call (the delivery)
    let initial_call_idx = get_nth_occurrences(2usize,call,solution).expect("\"correct_deliver_too_late(...)\": This unwrap should be unfalable, as this func is only called when the given call exists."); 
    let mut call_idx = initial_call_idx;
    let mut current_time = arrival_time;
    // Find the latest possible spot where picking up the call is possible.
    while current_time > end_upper{
        call_idx -= 1;
        let call_time:i32 = a_to_b_time(vehicle_idx,solution[call_idx],solution[call_idx+1],travel_costs); // Get the cost of going from idx to idx+1.
        current_time -= call_time;
    }
    // Move all calls from the desired position to the original position one step ahead, effective deleting the original call at its original position.
    let mut output:[i32;NCALLS*2] = solution.clone();
    for insert_idx in ((call_idx+1)..(initial_call_idx+1)).rev() { // The range [initial_call_idx, call_idx+1]
        output[insert_idx] = output[insert_idx -1];
    }
    // Then insert the desired call at the desired location. This overwrites the old call, which has been moved one step ahead.
    // Calls below this point should not be affected.
    output[call_idx] = call;
    return output;
}
fn correct_overloaded(solution :&[i32;NCALLS*2]) -> [i32;NCALLS*2]{
    todo!()
}
fn correct_pickup_not_delivered(solution :&[i32;NCALLS*2]) -> [i32;NCALLS*2]{
    todo!()
}


fn generate_any_valid_solution(
    vehicle_details : &[[i32;3usize]; NVEHICLES],
    call_details : &[[i32;8usize]; NCALLS],
    travel_costs: &[[i32;4usize];NNODES*NNODES*NVEHICLES],
    node_costs: &[[i32;5usize];NCALLS*NVEHICLES]
    ) -> [i32;NCALLS*2]{

    let mut current_solution:[i32;NCALLS*2] = [0i32;NCALLS*2];
    for i in 0..NCALLS{
        current_solution[i*2] = i as i32 +1;
        current_solution[i*2+1] = i as i32 +1;
    }
    let mut counter = 0;
    loop{
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
fn main(){

    //let path = Path::new(r"C:\Users\Mats\Rust Projects\sim_an\Call_7_Vehicle_3.txt");
    let path = Path::new(r"C:\Users\rivelandm\OneDrive - NOV Inc\Documents\Other\simulated_annealing_rust\sim_an\src\Call_7_Vehicle_3.txt");
    let mut vehicle_details_idx = 0;
    let mut vehicle_details: [[i32;3usize]; NVEHICLES] = [[-1i32;3usize]; NVEHICLES];

    let mut valid_calls_idx = 0;
    let mut valid_calls: [[i32;NCALLS]; NVEHICLES] = [[-1i32;NCALLS]; NVEHICLES];

    let mut call_details_idx = 0;
    let mut call_details: [[i32;8usize]; NCALLS] = [[-1i32;8usize]; NCALLS];

    let mut travel_costs_idx = 0;
    let mut travel_costs: [[i32;4usize];NNODES*NNODES*NVEHICLES] = [[-1i32;4usize];NNODES*NNODES*NVEHICLES];

    let mut node_cost_idx = 0;
    let mut node_costs: [[i32;5usize];NCALLS*NVEHICLES] = [[-1i32;5usize];NCALLS*NVEHICLES];
    
    let line_buffer = match read_lines(path){
        Ok(v) => v,
        Err(e) => {
            println!("### {} ###",e);
            let current_dir =  env::current_dir().unwrap();
            println!("Current dir: ___| {:?}{:?}", current_dir,path);
            return ();
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

    // Generate any valid solution.
    let valid_solution = generate_any_valid_solution(
        &vehicle_details,
        &call_details,
        &travel_costs,
        &node_costs
        );
        println!("\n_____ {:?} _____\n", valid_solution);
    ()
    
}

/*
In my project sim_an I have a folder called src. In scr is main.rs and doc.txt. I want to give the path for doc.txt to main.rs so that main.rs can read it. Currently I am getting the error: 

 """thread 'main' panicked at 'Could not unwrap LineBuffer: Os { code: 3, kind: NotFound, message: "The system cannot find the path specified." }', main.rs:28:22
note: run with `RUST_BACKTRACE=1` environment variable to display a backtrace"""

and my code is:

let filename = r"Call_7_Vehicle_3.txt";
fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where P: AsRef<Path>, {
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}
 */