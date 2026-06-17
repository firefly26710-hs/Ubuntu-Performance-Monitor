use std::fs::File;
use std::io::{BufRead, BufReader};
//use crate::cpu::logic as cpu_logic;
//use crate::mem::logic as mem_logic;
//use crate::gpu::logic as gpu_logic;
//use crate::disk::logic as disk_logic;


// check table :
//       cpu   : cpuinfo, stat
//       memory: meminfo  TotalMem, AvailMem

// public_array allocation:
// cpuinfo:49, cpu0-12 : 50
// MemTotal : 28, MemAvl : 28


fn read_cpu(public_array:&mut[u8; 769] ){
    if let Ok(mut file) = File::open("/proc/cpuinfo") {
        let reader = BufReader::new(file);
        if let Some(Ok( mut line)) = reader.lines().nth(4){
            let line_bytes = line.as_bytes();
            let len = line_bytes.len();
            public_array[0..len].copy_from_slice(&line_bytes[0..len]);
            println!("-----------");
            println!("{}", std::str::from_utf8(&public_array[0..len]).unwrap());
            println!("-----------");
        }
    }
}

//fn read_mem(){}

//fn read_gpu(){}

//fn read_disk(){}
    //直接看main部分



#[test]
fn test_proc_reading() { // file reading exp
    let mut public_array:[u8; 769] = [0; 769];
    read_cpu(&mut public_array);
}
