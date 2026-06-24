use std::fs::File;
use std::io::{BufRead, BufReader};
use std::str::from_utf8;

use crate::data_source::data::{DataSource, HALF_SIZE, PADDING_SIZE};

const THREAD_START:usize = 64;
const THREAD_NUMBER:usize = 12;

const NAME_INFO_START:usize = 0;
const NAME_INFO_END: usize = PADDING_SIZE - NAME_INFO_START;
const THREAD_SIZE:usize = THREAD_NUMBER * PADDING_SIZE;

pub fn read_cpu_info(source:&mut DataSource) {
    let data_source = &mut source.public_array;

    if let Ok(file) = File::open("/proc/cpuinfo") {
        let reader = BufReader::new(file);
        if let Some(Ok(raw_data)) = reader.lines().nth(4) {
            let name = match raw_data.split(':').nth(1){
                Some(name) => name.trim(),
                None => raw_data.trim(),
            };
            let name_len = name.len();
            let byte_char = name.as_bytes();
            data_source[NAME_INFO_START..NAME_INFO_END].fill(0);
            data_source[NAME_INFO_START..name_len].copy_from_slice(&byte_char);

            println!("-----------");
            println!("{}", from_utf8(&data_source[0..PADDING_SIZE]).unwrap());
            println!("-----------");
        }
    }



    if let Ok(file) = File::open("/proc/stat") {
        let reader = BufReader::new(file);
        for (NUMBER, raw_datas) in reader.lines().skip(1).take(THREAD_NUMBER).enumerate() {
            if let Ok(raw_data) = raw_datas {
                let mut data = raw_data.split_whitespace();
                data.next();

                let mut current_total: u64 = 0;
                let mut current_idle: u64 = 0;
                for(index, slice) in data.enumerate(){
                    if let Ok(val) = slice.parse::<u64>(){
                        current_total+=val;
                        if index == 3{ current_idle = val; }
                    }
                }

                let total_byte_char = current_total.to_be_bytes();
                let idle_byte_char = current_idle.to_be_bytes();

                let len_total = total_byte_char.len();
                let len_idle = idle_byte_char.len();

                let offest = NUMBER * PADDING_SIZE;
                let start =THREAD_START + offest;
                let end = start + PADDING_SIZE;
                let mid = (start + end) / 2;

                data_source[start..start + len_idle].copy_from_slice(&idle_byte_char);
                data_source[mid..mid + len_total].copy_from_slice(&total_byte_char);


                let check_idle = u64::from_be_bytes(data_source[start..start + len_total].try_into().unwrap());
                let check_total = u64::from_be_bytes(data_source[mid..mid+len_idle].try_into().unwrap());
                println!("-----------");
                println!("Thread {} -> Idle(前半): {}, Total(後半): {}", NUMBER, check_idle, check_total);
                println!("-----------");
            }
        }
    }
}



