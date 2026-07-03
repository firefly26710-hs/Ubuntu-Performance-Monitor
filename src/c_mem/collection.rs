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

pub fn mem_collection(source:&mut DataSource) -> Result<(), Box<dyn std::error::Error>>{
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


    Ok(())

}

#[test]
fn test_mem_collection(){
    let mut source = DataSource::new();
    mem_collection(&mut source);
    let data_array = &mut source.data_array;
    let check_total_memory:u64 = u64::from_be_bytes(data_array[MEMORY_TOTAL_START..MEMORY_TOTAL_END].try_into().unwrap());
    let check_avail_memory:u64 = u64::from_be_bytes(data_array[MEMORY_AVAIL_START..MEMORY_AVAIL_END].try_into().unwrap());
    eprintln!("Memory Total : {} , Memory Avail : {}", check_total_memory, check_avail_memory);
}