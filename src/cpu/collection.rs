use std::fs::File;
use std::io::{BufRead, BufReader};
use std::str::from_utf8;

use crate::data_source::data::{DataSource, PADDING_SIZE};

const NAME_FILE:&str = "/proc/cpuinfo";
const THREAD_FILE: &str = "/proc/stat";
const IDLE_POSITION:usize = 3;



const NAME_INFO_START:usize = 0;
const NAME_INFO_END: usize = NAME_INFO_START + PADDING_SIZE;


const THREAD_START:usize = PADDING_SIZE;
const THREAD_NUMBER:usize = 12;

pub fn read_cpu_info(source:&mut DataSource) -> Result<(), Box<dyn std::error::Error>>{
    let data_source = &mut source.public_array;


    let file = File::open(NAME_FILE)?;
    let reader = BufReader::new(file);
    let raw_data = reader.lines().nth(4).ok_or("找不到 cpuinfo 的第 5 行")?.map_err(|e| e.to_string())?;



    let name =  raw_data.split(":").nth(1).ok_or("找不到Name")?;
    let length = name.len();
    let byte_char = name.as_bytes();

    data_source[NAME_INFO_START..NAME_INFO_END].fill(0);
    data_source[NAME_INFO_START..NAME_INFO_START + length].copy_from_slice(&byte_char);

    println!("-----------");
    println!("{}", from_utf8(&data_source[0..PADDING_SIZE])?);
    println!("-----------");



    let file = File::open(THREAD_FILE)?;
    let reader = BufReader::new(file);
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

        let total_byte_char = current_total.to_be_bytes();
        let idle_byte_char = current_idle.to_be_bytes();

        let len_total = total_byte_char.len();
        let len_idle = idle_byte_char.len();

        let offest = NUMBER * PADDING_SIZE;
        let start =THREAD_START + offest;
        let end = start + PADDING_SIZE;
        let mid = (start + end) / 2;

        data_source[start..end].fill(0);
        data_source[start..start + len_total].copy_from_slice(&total_byte_char);
        data_source[mid..mid + len_idle].copy_from_slice(&idle_byte_char);


        let check_total= u64::from_be_bytes(data_source[start..start + len_total].try_into()?);
        let check_idle= u64::from_be_bytes(data_source[mid..mid+len_idle].try_into()?);
        println!("-----------");
        println!("Thread {} -> Total(前半): {}, Idle(前半): {}", NUMBER, check_total, check_idle);
        println!("-----------");
    }

    Ok(())
}



