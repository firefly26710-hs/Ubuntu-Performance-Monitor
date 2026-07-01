use std::fs::File;
use std::io::{BufRead, BufReader};
use crate::a_data_source::data::{DataSource, HALF_PADDING_SIZE, PADDING_SIZE};
const MEMORY_FILE : &str = "/proc/meminfo";
const THIS_INFO_IS_TOTAL_MEMORY:usize = 0;
const THIS_INFO_IS_AVAILABLE_MEMORY:usize = 2;
pub const MEMORY_TOTAL_START:usize = 0;
pub const MEMORY_TOTAL_END:usize = HALF_PADDING_SIZE;
pub const MEMORY_AVAIL_START:usize = HALF_PADDING_SIZE;
pub const MEMORY_AVAIL_END:usize = PADDING_SIZE;
const U64_LEN:usize = 8;
pub fn read_mem_info(source:&mut DataSource) -> Result<(), Box<dyn std::error::Error>>{
    let data_array = &mut source.data_array;
    
    let file = File::open(MEMORY_FILE)?;
    let reader = BufReader::new(file);
    for (this_info, line) in reader.lines().take(3).enumerate() {
        let info = line?;
        
        let mut data = info.split_whitespace().skip(1);
        let mut number:u64 = 0;
        if let Some(raw_str) = data.next() {
            if let Ok(num) = raw_str.parse::<u64>() {
                number = num;
            }
        }
        
        let slice:[u8; HALF_PADDING_SIZE] = number.to_be_bytes();
        match this_info {
            THIS_INFO_IS_TOTAL_MEMORY
            => {
                data_array[MEMORY_TOTAL_START..MEMORY_TOTAL_END].fill(0);
                data_array[MEMORY_TOTAL_START..MEMORY_TOTAL_END].copy_from_slice(&slice)
            },

            THIS_INFO_IS_AVAILABLE_MEMORY
            => {
                data_array[MEMORY_AVAIL_START..MEMORY_AVAIL_END].fill(0);
                data_array[MEMORY_AVAIL_START..MEMORY_AVAIL_END].copy_from_slice(&slice)
            },

            _
            => {}
        }

    }

    let check_total_memory:u64 = u64::from_be_bytes(data_array[MEMORY_TOTAL_START..MEMORY_TOTAL_END].try_into()?);
    let check_avail_memory:u64 = u64::from_be_bytes(data_array[MEMORY_AVAIL_START..MEMORY_AVAIL_END].try_into()?);
    println!("MEMORY TOTAL : {}, MEMORY AVAIL: {}", check_total_memory, &check_avail_memory);
    Ok(())

}