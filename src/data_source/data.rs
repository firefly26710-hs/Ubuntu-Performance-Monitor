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
    let mut cpu_info_len = 0;
    if let Ok(cpu_info) = File::open("/proc/cpuinfo") {
        let info_reader1 = BufReader::new(cpu_info);
        if let Some(Ok(model_info)) = info_reader1.lines().nth(4){
            let info_by_bytes = model_info.as_bytes();
            cpu_info_len = info_by_bytes.len();
            public_array[0..cpu_info_len].copy_from_slice(&info_by_bytes[0..cpu_info_len]);
            println!("-----------");
            println!("{}", std::str::from_utf8(&public_array[0..cpu_info_len]).unwrap());
            println!("-----------");
        }
    }

    if let Ok(stat) = File::open("/proc/stat"){
       let info_reader2 = BufReader::new(stat);
        for(i, thread) in info_reader2.lines().skip(1).take(12).enumerate(){
            if let Ok(thread_info) = thread{
                let mut info_by_bytes = thread_info.as_bytes();
                let thread_info_len = info_by_bytes.len();
                let mut offest = 50*(i + 1);
                let start = cpu_info_len + offest;

            }
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
