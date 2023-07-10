pub const NCALLS: usize = 80; // Note: Should be derived from file.
pub const NVEHICLES: usize = 20; // Note: Should be derived from file.
//Note: NNODES is fixed at 39 for custom files. For "original" files it follows the formula below.
//pub const NNODES: usize = NCALLS*(NVEHICLES*2)-NVEHICLES; // Note: Should be derived from file. (== 7*(3*2)-3 = 39)?
pub const NNODES: usize = 39usize;
pub const TRAVEL_TIME_SIZE: usize = NNODES*NNODES*NVEHICLES;
pub const SOLUTION_SIZE: usize = NCALLS*2;