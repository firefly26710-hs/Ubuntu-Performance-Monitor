//Check Table :

// CPU:
//------------------------------------------------
//  cpu_name |  thread0  | thread1 | thread2 | ...
//-----------------------------------------------
//PADDING:  0           1         2         3

// Memory:
//------------------------------------------------
// Total | avail  |
//-----------------------------------------------
//PADDING:        0              

// GPU:
//------------------------------------------------
// Total | avail  |
//-----------------------------------------------
//PADDING:        0           

// DISK:
//------------------------------------------------
// Total | avail  |
//-----------------------------------------------
//PADDING:        0      




pub const NAME_ARRAY_SIZE:usize = 64;
pub const PADDING_NUMBER:usize = 12;
pub const PADDING_SIZE:usize = 16;
pub const HALF_PADDING_SIZE:usize = PADDING_SIZE / 2;
pub const DATA_ARRAY_SIZE:usize = PADDING_NUMBER * PADDING_SIZE;
pub struct DataSource{
    pub name_array:[u8; NAME_ARRAY_SIZE],
    pub data_array:[u8; DATA_ARRAY_SIZE],
    pub prev_data_array:[u8; DATA_ARRAY_SIZE],
    pub chart_array:[f64;PADDING_NUMBER],
    pub gauge_array:[f64;3]
}

impl DataSource {
    pub fn new() -> Self {
        Self{
            name_array:[0u8; NAME_ARRAY_SIZE],
            data_array: [0u8; DATA_ARRAY_SIZE],
            prev_data_array:[0u8; DATA_ARRAY_SIZE],
            chart_array:[0f64;PADDING_NUMBER],
            gauge_array:[0f64;3]
        }
    }

}


