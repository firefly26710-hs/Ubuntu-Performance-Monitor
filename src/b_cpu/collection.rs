use std::fs::File;
use std::io::{BufRead, BufReader};

use crate::a_data_source::data::{DataSource, HALF_PADDING_SIZE, NAME_ARRAY_SIZE, PADDING_SIZE};
use crate::a_data_source::hpl::thread_number;

const NAME_FILE:&str = "/proc/cpuinfo";
const THREAD_FILE: &str = "/proc/stat";
const IDLE_POSITION:usize = 3;

pub const THREAD_START:usize = 0;
pub const THREAD_NUMBER:usize = 12;


pub fn cpu_collection(source:&mut DataSource, thread_number:usize){
    let name_array = &mut source.name_array;
    
    let mut file = File::open(NAME_FILE).expect("Can't find /proc/cpuinfo");
    let mut reader = BufReader::new(file);

    let line_data = reader.lines().nth(4).expect("no line 4").expect("No File");
    let name =  line_data.split(": ").nth(1).expect("Can't find CPU Name");

    let name_length = name.len().min(NAME_ARRAY_SIZE);
    let name_slice = name.as_bytes();

    name_array.fill(0);
    name_array[0..name_length].copy_from_slice(&name_slice[0..name_length]);


    
    file = File::open(THREAD_FILE).expect("Can't find ");
    reader = BufReader::new(file);
    let data_array = &mut source.data_array;
    for (number, raw_data) in reader.lines().skip(1).take(thread_number).enumerate() {
        let raw_data = raw_data.expect("Can't find this line data");

        let data = raw_data.split_whitespace().skip(1);
        let mut current_total: u64 = 0;
        let mut current_idle: u64 = 0;
        for(this_position, slice) in data.enumerate(){
            let val = slice.parse::<u64>().expect("Can change to u64");
            if this_position == IDLE_POSITION{ current_idle = val; }
            current_total+=val;
        }

        let total_slice = current_total.to_be_bytes();
        let idle_slice = current_idle.to_be_bytes();

        let len_total = total_slice.len().min(HALF_PADDING_SIZE);
        let len_idle = idle_slice.len().min(HALF_PADDING_SIZE);

        let offest = number * PADDING_SIZE;
        let start = THREAD_START + offest;
        let end = start + PADDING_SIZE;
        let mid = (start + end) / 2;

        data_array[start..end].fill(0);
        data_array[start..start + len_total].copy_from_slice(&total_slice);
        data_array[mid..mid + len_idle].copy_from_slice(&idle_slice);
    }
}

#[test]
fn test_cpu_collection() {
    let mut source = DataSource::new();
    let thread_number = thread_number();
    cpu_collection(&mut source, thread_number);
    let raw_name = std::str::from_utf8(&source.name_array).unwrap_or("");
    let cpu_name = raw_name.trim_matches(char::from(0)).trim();
    eprintln!("CPU NAME : {}", cpu_name);


    let data_array = source.data_array;
    for (number, _) in data_array.chunks(PADDING_SIZE).take(thread_number).enumerate(){
        let offest = number * PADDING_SIZE;
        let start = THREAD_START + offest;
        let end = start + PADDING_SIZE;
        let mid = (start + end) / 2;
        let check_total= u64::from_be_bytes(data_array[start..mid].try_into().unwrap());
        let check_idle= u64::from_be_bytes(data_array[mid..end].try_into().unwrap());
        eprintln!("THREAD : {} , TOTAL : {} , IDLE : {}", number, check_total, check_idle)
    }

}




