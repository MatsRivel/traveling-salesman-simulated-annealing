mod constructors;
mod constants;
mod validity_check;
mod correctors;
mod alter_solution;
mod coordinate_generator;
mod images_functions;
mod visualizations;

use constructors::{AllData, get_all_data};
use constants::{NVEHICLES, NCALLS, SOLUTION_SIZE, MAX_X, MAX_Y};
use image::RgbImage;
use std::path::Path;
use crate::{alter_solution::{naive_solve, semi_random_improve_solution}, constants::{N_THREADS, NNODES}, coordinate_generator::{find_node_orders, get_node_coords}, images_functions::make_gif, visualizations::{coords_to_points, lines_between_points}};
use std::time::Instant;
use rayon::prelude::{IntoParallelIterator, ParallelIterator};
use images_functions::{make_image};

fn _get_predefined_solution(predef:Vec<i32>) -> [i32;SOLUTION_SIZE] {
    let mut sol:[i32;SOLUTION_SIZE] = [0i32;SOLUTION_SIZE];
    for (idx,element) in predef.iter().enumerate(){
        sol[idx] = *element;
    }
    sol
}
fn prepare_data()->AllData{
    let base_path = r"C:\Users\rivelandm\OneDrive - NOV Inc\Documents\Other\traveling-salesman-simulated-annealing\sim_an\src\";
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
    full_path.push_str(base_path);
    full_path.push_str(file_name);
    let path = Path::new(full_path.as_str());
    let data_struct: AllData = get_all_data(path);
    data_struct
}
// Note to self: Would be more fair to read reduntant file but only keep non-redundant info.
// Currently reading a different file seems a bit like a cheat.
fn main(){
    //println!(" ################## Recursive size: {}\n ################## Static Size: {}", TRAVEL_TIME_SIZE, NNODES*NNODES*NVEHICLES);
    let start = Instant::now();
    let data_struct: AllData = prepare_data();
    let (vehicle_details, _valid_calls, call_details, travel_costs, node_costs) = data_struct.deconstruct();
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
        .map(|_| semi_random_improve_solution(&best_solution,&lowest_cost, &vehicle_details, &call_details, &travel_costs, &node_costs)).collect();
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

    // Find what order nodes occur in:
    let car_node_order = find_node_orders(&best_solution, &vehicle_details,&call_details,&travel_costs,&node_costs);
    let mut flat_node_vec = Vec::<i32>::with_capacity(car_node_order[0].len() * car_node_order.len());
    for sub_vec in car_node_order.iter() {
        for element in sub_vec.iter(){
            flat_node_vec.push(*element);
        }
    }

    let node_coords = get_node_coords(&flat_node_vec, &travel_costs, &[MAX_X, MAX_Y]);
    //let mut img = RgbImage::new(MAX_X as u32, MAX_Y as u32);
    // Making sure to place the used nodes first so that their relative positions ar the same.
    let other_nodes: Vec<i32> = (1i32..=(NNODES as i32)).filter(|i| !flat_node_vec.contains(i)).collect();
    let mut all_nodes= Vec::<i32>::with_capacity(NNODES); 
    for element in flat_node_vec{
        all_nodes.push(element);
    }
    for element in other_nodes{
        all_nodes.push(element);
    }
    let all_node_coords = get_node_coords(&all_nodes, &travel_costs, &[MAX_Y, MAX_X]);
    for point in node_coords.iter(){
        println!("{:?}", point);
    }
    //img = make_image(img, all_node_coords,[255u8,100u8,255u8], false);
    //img = make_image(img, node_coords,[100u8,255u8,255u8], true);
    //img = make_gif(img, node_coords,[100u8,255u8,255u8], true);

    make_visualization(&node_coords);


}
