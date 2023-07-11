pub const NCALLS: usize = 80; // Note: Should be derived from file.
pub const NVEHICLES: usize = 20; // Note: Should be derived from file.
pub const NNODES: usize = 39usize; // There are always 39 nodes.

// This macro creates a constant from a sum, based on "NNODES"

macro_rules! rec_func {
    ($constant_name:ident, $func_name:ident, $n:expr) => {
        pub const fn $func_name(x: usize) -> usize {
            match x {
                0 => 0,
                _ => x + $func_name(x - 1),
            }
        }
        pub const $constant_name:usize = 3*2*$func_name($n);
    };
}

macro_rules! rec_func_v2 {
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


rec_func_v2!(TRAVEL_TIME_SIZE, recursive_sum, NNODES);
pub const TRAVEL_TIME_SIZE_REDUNDANT: usize = NNODES*NNODES*NVEHICLES;
pub const SOLUTION_SIZE: usize = NCALLS*2;

