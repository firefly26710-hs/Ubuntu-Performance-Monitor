use std::fs::File;
use std::io::{BufRead, BufReader, Write};
use crate::cpu::logic as cpu_logic;
use crate::mem::logic as mem_logic;
use crate::gpu::logic as gpu_logic;
use crate::disk::logic as disk_logic;


#[test]
fn test_proc_reading() {
    let opener = File::open("/proc/stat");
    let reader = BufReader::new(opener.unwrap());

    for (index, line) in reader.lines().enumerate() {
        let line_content = line.unwrap();
        println!("{}\n", line_content);

    }
}





//pub fn hardware_type()

