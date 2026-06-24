use std::fs::File;
use std::io::{BufRead, BufReader};
use crate::data_source::data::{DataSource, PADDING_SIZE};

const THIS_IS_TOTAL_MEMORY_INFO:usize = 0;
const THIS_IS_AVAILABLE_MEMORY_INFO:usize = 2;
const MEMORY_TOTAL_START:usize = 0;
const MEMORY_TOTAL_END:usize = MEMORY_TOTAL_START + PADDING_SIZE;
const MEMORY_AVAIL_START:usize = PADDING_SIZE;
const MEMORY_AVAIL_END:usize = MEMORY_AVAIL_START + PADDING_SIZE;
pub fn read_mem_info(source:&mut DataSource) {
    let data_source = &mut source.public_array;
    
    if let Ok(file) = File::open("/proc/meminfo") {
        let reader = BufReader::new(file);
        for (THIS_INFO, info) in reader.lines().take(3).enumerate() {
            if let Ok(info) = info {
                let mut data = info.split_whitespace();
                data.next();
                let mut number:u64 = 0;

                for(index, slice) in data.enumerate(){
                    if let Ok(val) = slice.parse::<u64>(){
                        if index == 0{ number = val }
                    }
                }
                
                let byte_char = number.to_be_bytes();
                let length = byte_char.len();
                match THIS_INFO {
                    THIS_IS_TOTAL_MEMORY_INFO
                    => {
                        data_source[MEMORY_TOTAL_START..MEMORY_TOTAL_END].fill(0);
                        data_source[MEMORY_TOTAL_START..MEMORY_TOTAL_START + length].copy_from_slice(&byte_char)
                    },

                    THIS_IS_AVAILABLE_MEMORY_INFO
                    => {
                        data_source[MEMORY_AVAIL_START..MEMORY_AVAIL_END].fill(0);
                        data_source[MEMORY_AVAIL_START..MEMORY_AVAIL_START + length].copy_from_slice(&byte_char)
                    },

                    _
                    => {}
                }


            }
        }
    }
    let check_total_memory = u64::from_be_bytes(data_source[MEMORY_TOTAL_START..MEMORY_TOTAL_START + 8].try_into().unwrap());
    let check_avail_memory = u64::from_be_bytes(data_source[MEMORY_AVAIL_START..MEMORY_AVAIL_START + 8].try_into().unwrap());
    println!("MEMORY TOTAL : {}, MEMORY AVAIL: {}", check_total_memory, &check_avail_memory)

}