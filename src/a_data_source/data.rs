//Check Table :
// CPU:
//------------------------------------------------
//  cpu_name |  thread0  | thread1 | thread2 | ...
//-----------------------------------------------
//PADDING:  0           1         2         3

// Memory:
//------------------------------------------------
//  Memory Total |  Memory avail  |
//-----------------------------------------------
//PADDING:       0                1

// Disk:
//------------------------------------------------
//  Disk Total |  Disk Avail  |
//-----------------------------------------------
//PADDING:     0              1



pub const NAME_ARRAY_SIZE:usize = 64;
pub const PADDING_NUMBER:usize = 12;
pub const PADDING_SIZE:usize = 16;
pub const HALF_PADDING_SIZE:usize = PADDING_SIZE / 2;
pub const DATA_ARRAY_SIZE:usize = PADDING_NUMBER * PADDING_SIZE;
pub const TIMESTAMP:usize = 30;
pub const U64_LEN:usize = 8;
pub struct DataSource{
    pub name_array:[u8; NAME_ARRAY_SIZE],
    pub data_array:[u8; DATA_ARRAY_SIZE],
    pub history_array:[[f64;TIMESTAMP];PADDING_NUMBER]
}

impl DataSource {
    pub fn new() -> Self {
        Self{
            name_array:[0u8; NAME_ARRAY_SIZE],
            data_array: [0u8; DATA_ARRAY_SIZE],
            history_array:[[0f64;TIMESTAMP];PADDING_NUMBER]
        }
    }

}


