mod constructors;
mod constants;
mod validity_check;
mod correctors;
mod alter_solution;

use validity_check::correctness_check;
use constructors::{AllData, get_all_data};
use constants::{NVEHICLES, NCALLS,NNODES, SOLUTION_SIZE, TRAVEL_TIME_SIZE,MAX_RUNTIME_IN_SECONDS};
use alter_solution::{generate_any_valid_solution};
use std::{path::Path, panic};
use crate::{alter_solution::{brute_force_solve, naive_solve, randomly_improve_solution}, constants::N_THREADS};
use std::time::{Duration,Instant};
use rayon::prelude::{self, IntoParallelIterator, ParallelIterator};


fn get_predefined_solution(predef:Vec<i32>) -> [i32;SOLUTION_SIZE] {
    let mut sol:[i32;SOLUTION_SIZE] = [0i32;SOLUTION_SIZE];
    for (idx,element) in predef.iter().enumerate(){
        sol[idx] = *element;
    }
    return sol;
}
fn prepare_data()->AllData{
    let base_path = r"C:\Users\Mats\OneDrive - University of Bergen\Documents\Rust Stuff\traveling-salesman-simulated-annealing\traveling-salesman-simulated-annealing\sim_an\src\";
    let file_name = match (NVEHICLES,NCALLS){
        (1,2) => r"Call_2_Vehicle_1_Custom.txt",
        (2,4) => r"Call_4_Vehicle_2_Custom.txt",
        (3,7) => r"Call_7_Vehicle_3.txt",
        (5,18) => r"Call_18_Vehicle_5.txt",
        (7,35) => r"Call_35_Vehicle_7.txt",
        (20,80) => r"Call_80_Vehicle_20.txt",
        (40,130) => r"Call_130_Vehicle_40.txt",
        _ => "",
    };
    let mut full_path = "".to_string();
    full_path.push_str(&base_path);
    full_path.push_str(&file_name);
    let path = Path::new(full_path.as_str());
    let data_struct: AllData = get_all_data(path);
    return data_struct;
}
fn correct_adaptive_input_validity_test(data_struct: AllData){
    println!("correct_adaptive_input_validity_check()");
    let (vehicle_details, valid_calls, call_details, travel_costs, node_costs) = data_struct.deconstruct();
    let solution:[i32;SOLUTION_SIZE] = match (NVEHICLES,NCALLS){
        (1,2) => get_predefined_solution(vec![2,1,1,2]),
        (2,4) => get_predefined_solution(vec![3, 3, 1, 1, 2, 2, 4, 4]),
        (3,7) => get_predefined_solution(vec![7, 6, 3, 7, 3, 1, 1, 6, 2, 2, 4, 4, 5, 5]),
        (5,18) => get_predefined_solution(vec![17, 14, 9, 2, 18, 17, 10, 9, 2, 16, 14, 10, 7, 16, 8, 3, 11, 8, 7, 3, 1, 1, 4, 4, 5, 5, 6, 6, 11, 12, 12, 13, 13, 15, 15, 18]),
        (7,35) => get_predefined_solution(vec![35, 33, 31, 26, 4, 35, 33, 31, 26, 32, 9, 4, 32, 9, 30, 29, 27, 30, 27, 15, 29, 24, 15, 1, 1, 2, 2, 24, 3, 3, 5, 5, 6, 6, 7, 7, 8, 8, 10, 10, 11, 11, 12, 12, 13, 13, 14, 14, 16, 16, 17, 17, 18, 18, 19, 19, 20, 20, 21, 21, 22, 22, 23, 23, 25, 25, 28, 28, 34, 34] ),
        (20,80) => todo!(), // get_predefined_solution(vec!),
        (40,130) => todo!(), // get_predefined_solution(vec!),
        (_,_) => todo!(),
    };
    assert!(correctness_check(&solution, &vehicle_details, &call_details, &travel_costs, &node_costs ).is_ok());
}
// Note to self: Would be more fair to read reduntant file but only keep non-redundant info.
// Currently reading a different file seems a bit like a cheat.
fn main(){
    //println!(" ################## Recursive size: {}\n ################## Static Size: {}", TRAVEL_TIME_SIZE, NNODES*NNODES*NVEHICLES);
    let start = Instant::now();
    let data_struct: AllData = prepare_data();
    let (vehicle_details, valid_calls, call_details, travel_costs, node_costs) = data_struct.deconstruct();
    // Note to self: travel_costs now only considers combinations that COULD occur, as well as only reading through non-redundant data.
    // This allows for much larger solution spaces to be considerer quickly.
    // Currently still only runs naive solutions.
    //correct_adaptive_input_validity_test(data_struct);
    //println!("Current array capacity: {TRAVEL_TIME_SIZE}");
    // Generate any valid solution.
    println!("travel_costs size: {}", travel_costs.len());
    let (mut best_solution, mut lowest_cost) = naive_solve(
                                                            &vehicle_details,
                                                            &call_details,
                                                            &travel_costs,
                                                            &node_costs
                                                        );
    let runtime = (start.elapsed().as_nanos() as f32) / 10f32.powf(9f32);
    println!("Total runtime: {:?}sec\nTotal Cost: ${}\nSolution:\n{:?}", runtime, lowest_cost, best_solution);
    let old_cost = lowest_cost;
    // Time to improve the solution:
    // randomly_improve_solution(&best_solution,&lowest_cost, &vehicle_details, &call_details, &travel_costs, &node_costs)
    let changed_solutions: Vec<Option<([i32;SOLUTION_SIZE], i32)>> = (0..N_THREADS)
        .into_par_iter()
        .map(|_| randomly_improve_solution(&best_solution,&lowest_cost, &vehicle_details, &call_details, &travel_costs, &node_costs)).collect();
    for potential_solution in changed_solutions.iter() {
        let (sol,val) = match potential_solution{
            None => continue,
            Some((sol,val)) => (*sol,*val)
        };
        if val < lowest_cost {
            best_solution = sol;
            lowest_cost = val;
        }
    }
    println!("\nImproving:\n");
    let runtime2 = (start.elapsed().as_nanos() as f32) / 10f32.powf(9f32);
    println!("Total runtime: {:?}sec\nTotal Cost: ${}\nSolution:\n{:?}", runtime2, lowest_cost, best_solution);
    println!("Cost improvement: ${:?} cheaper after running for {:?}s longer", old_cost - lowest_cost, runtime2-runtime);
}
