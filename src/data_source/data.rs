use std::fs::File;
use std::io::{BufRead, BufReader};
use std::str::from_utf8;
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
    let mut name_length = 0;
    if let Ok(cpu_info) = File::open("/proc/cpuinfo") {
        let cpuinfo_reader = BufReader::new(cpu_info);
        if let Some(Ok(name_info)) = cpuinfo_reader.lines().nth(4){
            let name_as_bytes = name_info.as_bytes();
            name_length = name_as_bytes.len();

            public_array[0..name_length].copy_from_slice(&name_as_bytes[0..name_length]);

            println!("-----------");
            println!("{}", from_utf8(&public_array[0..name_length]).unwrap());
            println!("-----------");
        }
    }

    if let Ok(stat) = File::open("/proc/stat"){
       let stat_reader = BufReader::new(stat);
        for(number, thread) in stat_reader.lines().skip(1).take(12).enumerate(){
            if let Ok(thread_info) = thread{
                let mut thread_as_bytes = thread_info.as_bytes();
                let thread_length = thread_as_bytes.len();
                let padding = 50;
                let offest = number*padding;
                let start = name_length + offest;
                let end = start + padding;

                let mut buffer_padding = [0u8; 50];
                buffer_padding[0..thread_length].copy_from_slice(&thread_as_bytes[0..thread_length]);

                public_array[start..end].copy_from_slice(&buffer_padding);
                println!("-----------");
                println!("{}", std::str::from_utf8(&public_array[start..end]).unwrap());
                println!("-----------");
            }
        }
    }
}

fn read_mem(public_array:&mut[u8; 769]){
    if let Ok(meminfo) = File::open("/proc/meminfo"){
        let meminfo_reader = BufReader::new(meminfo);
        for(number, info) in meminfo_reader.lines().take(3).enumerate(){
            if let Ok(this_info) = info{
                let mut info_as_bytes = this_info.as_bytes();
                let info_length = info_as_bytes.len();
                let mut buffer_padding = [0u8; 30];
                buffer_padding[0..info_length].copy_from_slice(&info_as_bytes[0..info_length]);
                match number {
                    0 => public_array[0..30].copy_from_slice(&buffer_padding),
                    2 => public_array[30..60].copy_from_slice(&buffer_padding),
                    _ => {}
                }

                match number {
                    0 => {
                        println!("-----------");
                        println!("{}\n", from_utf8(&public_array[0..30]).unwrap());
                        println!("-----------");
                    },
                    2 => {
                        println!("-----------");
                        println!("{}\n", from_utf8(&public_array[30..60]).unwrap());
                        println!("-----------");
                    },
                    _ => {}
                }

            }
        }
    }
}

//fn read_gpu(){}

//fn read_disk(){}
    //直接看main部分



#[test]
fn test_proc_reading() { // file reading exp
    let mut public_array:[u8; 769] = [0; 769];
    read_mem(&mut public_array);
}
