use crate::constants::{NVEHICLES,NNODES,NCALLS,TRAVEL_TIME_SIZE,SOLUTION_SIZE};

fn prepare_data(){
    let base_path = r"C:\Users\rivelandm\OneDrive - NOV Inc\Documents\Other\simulated_annealing_rust\sim_an\src\";
    let file_name = match (NVEHICLES,NCALLS){
        (1,2) => r"Call_2_Vehicle_1_Custom.txt",
        (2,4) => r"Call_4_Vehicle_2_Custom.txt",
        (3,7) => r"Call_7_Vehicle_3.txt",
        (5,18) => r"Call_18_Vehicle_5.txt",
        (7,35) => r"Call_035_Vehicle_07.txt",
        (20,80) => r"Call_080_Vehicle_20.txt",
        (40,130) => r"Call_130_Vehicle_40.txt",
        _ => "",
    };
    let mut full_path = "".to_string();
    full_path.push_str(&base_path);
    full_path.push_str(&file_name);
    let path = Path::new(full_path.as_str());
    let data_struct: AllData = get_all_data(path);
    return data_struct.deconstruct();
}

#[cfg(test)]
mod tests {
    #[test]
    fn correct_adaptive_input_validity_check() {
        println("correct_adaptive_input_validity_check()");
        let (vehicle_details, valid_calls, call_details, travel_costs, node_costs) = data_struct.deconstruct();
        let solution:[i32;SOLUTION_SIZE] = match (NVEHICLES,NCALLS){
            (1,2) => [1,1,2,2],
            (2,4) => todo!(),
            (3,7) => todo!(),
            (5,18) => todo!(),
            (7,35) => todo!(),
            (20,80) => todo!(),
            (40,130) => todo!(),
            _ => "",
        };
        assert_eq!(correctness_check(solution, vehicle_details, call_details, travel_costs,node_costs ), Ok());
    }
}