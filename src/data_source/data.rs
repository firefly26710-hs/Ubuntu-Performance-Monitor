use std::fs::File;
use std::io::{BufRead, BufReader};
use std::str::from_utf8;
use nix::libc::read;
use nix::sys::statvfs::statvfs;
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


const MAX_PUBLIC_ARRAY_SIZE:usize = 769;
const THREAD_NUMBER:usize = 12;
const THREAD_PADDING:usize = 50;
const MEMORY_INFO_PADDING:usize = 30;
const TOTAL_MEMORY_INFO:usize = 0;
const AVAILABLE_MEMORY_INFO:usize = 2;

const DISK_INFO_PADDING:usize = 8;
fn read_cpu(public_array:&mut[u8; MAX_PUBLIC_ARRAY_SIZE] ){
    let mut name_length = 0;
    if let Ok(cpuinfo_file) = File::open("/proc/cpuinfo") {
        let cpuinfo_reader = BufReader::new(cpuinfo_file);
        if let Some(Ok(name_info)) = cpuinfo_reader.lines().nth(4){
            let name_as_bytes = name_info.as_bytes();
            name_length = name_as_bytes.len();

            public_array[0..name_length].copy_from_slice(&name_as_bytes[0..name_length]);

            println!("-----------");
            println!("{}", from_utf8(&public_array[0..name_length]).unwrap());
            println!("-----------");
        }
    }

    if let Ok(stat_file) = File::open("/proc/stat"){
        let stat_reader = BufReader::new(stat_file);
        for(number, thread) in stat_reader.lines().skip(1).take(THREAD_NUMBER).enumerate(){
            if let Ok(thread_info) = thread{
                let thread_as_bytes = thread_info.as_bytes();
                let thread_length = thread_as_bytes.len();
                let offest = number*THREAD_PADDING;
                let start = name_length + offest;
                let end = start + THREAD_PADDING;

                let mut buffer_padding = [0u8; THREAD_PADDING];
                buffer_padding[0..thread_length].copy_from_slice(&thread_as_bytes[0..thread_length]);

                public_array[start..end].copy_from_slice(&buffer_padding);
                println!("-----------");
                println!("{}", from_utf8(&public_array[start..end]).unwrap());
                println!("-----------");
            }
        }
    }
}

fn read_mem(public_array:&mut[u8; MAX_PUBLIC_ARRAY_SIZE]){
    if let Ok(meminfo_file) = File::open("/proc/meminfo"){
        let meminfo_reader = BufReader::new(meminfo_file);
        for(number, info) in meminfo_reader.lines().take(3).enumerate(){
            if let Ok(this_info) = info{
                let info_as_bytes = this_info.as_bytes();
                let info_length = info_as_bytes.len();
                let mut buffer_padding = [0u8; MEMORY_INFO_PADDING];
                buffer_padding[0..info_length].copy_from_slice(&info_as_bytes[0..info_length]);
                match number {
                    TOTAL_MEMORY_INFO
                    => public_array[0..MEMORY_INFO_PADDING].copy_from_slice(&buffer_padding),

                    AVAILABLE_MEMORY_INFO
                    => public_array[MEMORY_INFO_PADDING..MEMORY_INFO_PADDING * 2].copy_from_slice(&buffer_padding),

                    _
                    => {}
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



fn read_disk(public_array:&mut[u8; MAX_PUBLIC_ARRAY_SIZE]){
    let path = "/";
    if let Ok( statvfs )= statvfs(path){
        let f_frsize = statvfs.fragment_size();
        let f_blocks = statvfs.blocks();
        let f_bavail = statvfs.blocks_available();

        let total = f_blocks * f_frsize;
        let avail = f_bavail * f_frsize;

        let total_as_bytes = total.to_be_bytes();
        let avail_as_bytes = avail.to_be_bytes();

        public_array[0..DISK_INFO_PADDING].copy_from_slice(&total_as_bytes);
        public_array[DISK_INFO_PADDING..DISK_INFO_PADDING*2].copy_from_slice(&avail_as_bytes);

        println!("-----------");
        println!("{}", u64::from_be_bytes(public_array[0..DISK_INFO_PADDING].try_into().unwrap()));
        println!("{}", u64::from_be_bytes(public_array[DISK_INFO_PADDING..DISK_INFO_PADDING*2].try_into().unwrap()));
        println!("-----------");

    }

}



#[test]
fn test_proc_reading() { // file reading exp
    let mut public_array:[u8; MAX_PUBLIC_ARRAY_SIZE] = [0; MAX_PUBLIC_ARRAY_SIZE];
    read_disk(&mut public_array);
    read_mem(&mut public_array);
    read_cpu(&mut public_array);
}
