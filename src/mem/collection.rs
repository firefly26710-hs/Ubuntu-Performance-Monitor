use std::fs::File;
use std::io::{BufRead, BufReader};
use std::str::from_utf8;
use crate::data_source::data::{DataSource, PADDING_SIZE};

const TOTAL_MEMORY_INFO:usize = 0;
const AVAILABLE_MEMORY_INFO:usize = 2;
pub fn read_mem_info(source:&mut DataSource) {
    let data_source = &mut source.public_array;
    
    if let Ok(file) = File::open("/proc/meminfo") {
        let reader = BufReader::new(file);
        for (number, info) in reader.lines().take(3).enumerate() {
            if let Ok(info) = info {
                let actual_length = info.len();
                let byte_char = info.as_bytes();
                let mut buffer_padding = [0u8; PADDING_SIZE];
                buffer_padding[0..actual_length].copy_from_slice(&byte_char[0..actual_length]);
                match number {
                    TOTAL_MEMORY_INFO
                    => data_source[0..PADDING_SIZE].copy_from_slice(&buffer_padding),

                    AVAILABLE_MEMORY_INFO
                    => data_source[PADDING_SIZE..2 * PADDING_SIZE].copy_from_slice(&buffer_padding),

                    _
                    => {}
                }

                match number {
                    TOTAL_MEMORY_INFO
                    => {
                        println!("-----------");
                        println!("{}", from_utf8(&data_source[0..PADDING_SIZE]).unwrap());
                        println!("-----------");
                    },
                    AVAILABLE_MEMORY_INFO
                    => {
                        println!("-----------");
                        println!("{}", from_utf8(&data_source[PADDING_SIZE..2 * PADDING_SIZE]).unwrap());
                        println!("-----------");
                    },
                    _ => {}
                }
            }
        }
    }
}