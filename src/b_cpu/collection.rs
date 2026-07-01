use std::fs::File;
use std::io::{BufRead, BufReader};
use std::str::from_utf8;

use crate::a_data_source::data::{DataSource, HALF_PADDING_SIZE, NAME_ARRAY_SIZE, PADDING_SIZE};

const NAME_FILE:&str = "/proc/cpuinfo";
const THREAD_FILE: &str = "/proc/stat";
const IDLE_POSITION:usize = 3;

pub const THREAD_START:usize = 0;
pub const THREAD_NUMBER:usize = 12;


pub fn read_cpu_info(source:&mut DataSource) -> Result<(), Box<dyn std::error::Error>>{
    let name_array = &mut source.name_array;
    
    let file = File::open(NAME_FILE)?;
    let reader = BufReader::new(file);
    let raw_data = reader.lines().nth(4).ok_or("找不到 cpuinfo 的第 5 行")?.map_err(|e| e.to_string())?;
    
    let name =  raw_data.split(":").nth(1).ok_or("找不到Name")?;
    let name_length = name.len().min(NAME_ARRAY_SIZE);
    let name_slice = name.as_bytes();

    name_array.fill(0);
    name_array[0..name_length].copy_from_slice(&name_slice[0..name_length]);

    println!("-----------");
    println!("{}", from_utf8(&name_array[0..name_length])?);
    println!("-----------");



    
    let file = File::open(THREAD_FILE)?;
    let reader = BufReader::new(file);
    let data_array = &mut source.data_array;
    for (NUMBER, raw_datas) in reader.lines().skip(1).take(THREAD_NUMBER).enumerate() {
        let raw_data = raw_datas?;

        let data = raw_data.split_whitespace().skip(1);

        let mut current_total: u64 = 0;
        let mut current_idle: u64 = 0;

        for(THIS_POSITION, slice) in data.enumerate(){
            let val = slice.parse::<u64>()?;
            if THIS_POSITION == IDLE_POSITION{ current_idle = val; }
            current_total+=val;
        }

        let total_slice = current_total.to_be_bytes();
        let idle_slice = current_idle.to_be_bytes();

        let len_total = total_slice.len().min(HALF_PADDING_SIZE);
        let len_idle = idle_slice.len().min(HALF_PADDING_SIZE);

        let offest = NUMBER * PADDING_SIZE;
        let start = THREAD_START + offest;
        let end = start + PADDING_SIZE;
        let mid = (start + end) / 2;

        data_array[start..end].fill(0);
        data_array[start..start + len_total].copy_from_slice(&total_slice);
        data_array[mid..mid + len_idle].copy_from_slice(&idle_slice);


        let check_total= u64::from_be_bytes(data_array[start..start + len_total].try_into()?);
        let check_idle= u64::from_be_bytes(data_array[mid..mid+len_idle].try_into()?);
        println!("-----------");
        println!("Thread {} -> Total(前半): {}, Idle(後半): {}", NUMBER, check_total, check_idle);
        println!("-----------");
    }
    Ok(())
}



