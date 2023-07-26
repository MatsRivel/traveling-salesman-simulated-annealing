pub const NCALLS: usize = 130; // Note: Should be derived from file.
pub const NVEHICLES: usize = 40; // Note: Should be derived from file.
pub const NNODES: usize = 39usize; // There are always 39 nodes.
pub const MAX_RUNTIME_IN_SECONDS: u64 = 0; // How long should the program run to find a better solution?
pub const N_THREADS: usize = 8;
// This macro creates a constant from a sum, based on "NNODES" and "NVEHICLES".
macro_rules! rec_func {
    ($constant_name:ident, $func_name:ident, $n:expr) => {
        pub const fn $func_name(x: usize) -> usize {
            match x {
                0 => 0,
                _ => 1 + $n/2 + $func_name(x - 1),
            }
        }
        pub const $constant_name:usize = NVEHICLES*2*$func_name($n);
    };
}

rec_func!(_TRAVEL_TIME_SIZE, _recursive_sum, NNODES);
//pub const TRAVEL_TIME_SIZE_REDUNDANT: usize = NNODES*NNODES*NVEHICLES;
pub const SOLUTION_SIZE: usize = NCALLS*2;

