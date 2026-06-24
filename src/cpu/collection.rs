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
        if let Some(Ok(info)) = reader.lines().nth(4) {
            let name = match info.split(':').nth(1){
                Some(name) => name.trim(),
                None => info.trim(),
            };
            let name_len = name.len();
            let byte_char = name.as_bytes();
            let mut buffer_array = [0u8; PADDING_SIZE];
            buffer_array[0..name_len].copy_from_slice(&byte_char[0..name_len]);
            data_source[0..PADDING_SIZE].copy_from_slice(&buffer_array);

            println!("-----------");
            println!("{}", from_utf8(&data_source[0..PADDING_SIZE]).unwrap());
            println!("-----------");
        }
    }



    if let Ok(file) = File::open("/proc/stat") {
        let reader = BufReader::new(file);
        for (NUMBER, infos) in reader.lines().skip(1).take(THREAD_NUMBER).enumerate() {
            if let Ok(info) = infos {
                let mut datas = info.split_whitespace();
                datas.next();

                let mut current_total: u64 = 0;
                let mut current_idle: u64 = 0;
                for(index, slice) in datas.enumerate(){
                    if let Ok(val) = slice.parse::<u64>(){
                        current_total+=val;
                        if index == 3{
                            current_idle = val;
                        }
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


                let mut buffer_padding = [0u8; HALF_SIZE];

                buffer_padding[0..len_idle].copy_from_slice(&idle_byte_char[0..len_idle]);
                data_source[start..mid].copy_from_slice(&buffer_padding);

                buffer_padding[0..len_total].copy_from_slice(&total_byte_char[0..len_total]);
                data_source[mid..end].copy_from_slice(&buffer_padding);


                let check_idle = u64::from_be_bytes(data_source[start..start+8].try_into().unwrap());
                let check_total = u64::from_be_bytes(data_source[mid..mid+8].try_into().unwrap());
                println!("-----------");
                println!("Thread {} -> Idle(前半): {}, Total(後半): {}", NUMBER, check_idle, check_total);
                println!("-----------");
            }
        }
    }
}



