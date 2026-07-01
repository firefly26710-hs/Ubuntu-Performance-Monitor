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



pub const PADDING_SIZE:usize = 64;
pub const PADDING_NUMBER:usize = 13;
pub const MAX_PUBLIC_ARRAY_SIZE:usize = PADDING_NUMBER * PADDING_SIZE;
pub const TIMESTAMP:usize = 30;
pub const THREAD_NUMBER:usize = 12;
pub const U64_SIZE:usize = 8;
pub struct DataSource{
    pub public_array:[u8; MAX_PUBLIC_ARRAY_SIZE],
    pub history_array:[[f64;TIMESTAMP];THREAD_NUMBER]
}

impl DataSource {
    pub fn new() -> Self {
        Self{
            public_array: [0; MAX_PUBLIC_ARRAY_SIZE],
            history_array:[[0f64;TIMESTAMP];THREAD_NUMBER]
        }
    }

}


