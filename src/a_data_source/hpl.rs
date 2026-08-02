use std::fs::File;
use std::io::{BufRead, BufReader};
use crate::a_data_source::data::DataSource;
use crate::b_cpu::collection::cpu_collection;
use crate::b_cpu::logic::cpu_logic;

pub fn thread_number() -> usize {
    let file = File::open("/proc/stat").expect("Can't find file /proc/stat ");
    let reader = BufReader::new(file);
    let res = reader.lines().map(|l| l.unwrap()).
        filter(|is| is.contains("cpu")).count();

    res - 1
}

#[test]
fn test_hpl(){
    let thread_number = thread_number();
    let source = &mut DataSource::new();
    cpu_collection(source, thread_number);
    cpu_logic(source, thread_number);
    eprintln!("Is Thread Number is true? {}", thread_number == 12);
}
