use std::fs::File;
use std::io::{BufRead, BufReader};
use std::str::from_utf8;
use nix::sys::statvfs::statvfs;

// check table :
//       cpu   : cpuinfo, stat
//       memory: meminfo  TotalMem, AvailMem

// public_array allocation:
// cpuinfo:49, cpu0-12 : 50
// MemTotal : 28, MemAvl : 28


const MAX_PUBLIC_ARRAY_SIZE:usize = 832;
const PADDING:usize = 64;
const THREAD_START:usize = 64;
const THREAD_NUMBER:usize = 12;
const TOTAL_MEMORY_INFO:usize = 0;
const AVAILABLE_MEMORY_INFO:usize = 2;



fn read_cpu_name(public_array:&mut[u8; MAX_PUBLIC_ARRAY_SIZE] ){
    if let Ok(cpuinfo_file) = File::open("/proc/cpuinfo") {
        let cpuinfo_reader = BufReader::new(cpuinfo_file);
        if let Some(Ok(name_info)) = cpuinfo_reader.lines().nth(4){
            let actual_length = name_info.len();
            let byte_char = name_info.as_bytes();

            let mut buffer_array = [0u8; PADDING];
            buffer_array[0..actual_length].copy_from_slice(&byte_char[0..actual_length]);
            public_array[0..PADDING].copy_from_slice(&buffer_array);

            println!("-----------");
            println!("{}", from_utf8(&public_array[0..PADDING]).unwrap());
            println!("-----------");
        }
    }
}

fn read_thread(public_array:&mut[u8; MAX_PUBLIC_ARRAY_SIZE] ){
    if let Ok(stat_file) = File::open("/proc/stat"){
        let stat_reader = BufReader::new(stat_file);
        for(number, thread) in stat_reader.lines().skip(1).take(THREAD_NUMBER).enumerate(){
            if let Ok(thread_info) = thread{
                let actual_length = thread_info.len();
                let byte_char = thread_info.as_bytes();

                let offest = number*PADDING;
                let start = THREAD_START + offest;
                let end = start + PADDING;

                let mut buffer_padding = [0u8; PADDING];
                buffer_padding[0..actual_length].copy_from_slice(&byte_char[0..actual_length]);
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
            if let Ok(info) = info{
                let actual_length = info.len();
                let byte_char = info.as_bytes();
                let mut buffer_padding = [0u8; PADDING];
                buffer_padding[0..actual_length].copy_from_slice(&byte_char[0..actual_length]);
                match number {
                    TOTAL_MEMORY_INFO
                    => public_array[0..PADDING].copy_from_slice(&buffer_padding),

                    AVAILABLE_MEMORY_INFO
                    => public_array[PADDING..PADDING+PADDING].copy_from_slice(&buffer_padding),

                    _
                    => {}
                }

                match number {
                    0 => {
                        println!("-----------");
                        println!("{}\n", from_utf8(&public_array[0..PADDING]).unwrap());
                        println!("-----------");
                    },
                    2 => {
                        println!("-----------");
                        println!("{}\n", from_utf8(&public_array[PADDING..PADDING+PADDING]).unwrap());
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

        let disk_total = f_blocks * f_frsize;
        let disk_avail = f_bavail * f_frsize;


        let total_byte_char = disk_total.to_be_bytes();
        let avail_byte_char = disk_avail.to_be_bytes();

        let len_total = total_byte_char.len();
        let len_avail = avail_byte_char.len();

        let mut buffer_array = [0u8 ; PADDING];

        buffer_array[0..len_avail].copy_from_slice(&avail_byte_char[0..len_avail]);
        public_array[PADDING..PADDING+PADDING].copy_from_slice(&buffer_array[0..PADDING]);

        buffer_array[0..len_total].copy_from_slice(&total_byte_char[0..len_total]);
        public_array[0..PADDING].copy_from_slice(&buffer_array[0..PADDING]);

        let total_restored = u64::from_be_bytes(public_array[0..8].try_into().unwrap());
        let avail_restored = u64::from_be_bytes(public_array[PADDING..PADDING + 8].try_into().unwrap());
        println!("-----------");
        println!("Total: {}", total_restored);
        println!("Avail: {}", avail_restored);
        println!("-----------");

    }

}



#[test]
fn test_proc_reading() { // file reading exp
    let mut public_array:[u8; MAX_PUBLIC_ARRAY_SIZE] = [0; MAX_PUBLIC_ARRAY_SIZE];

    read_cpu_name(&mut public_array);
    read_thread(&mut public_array);
    read_mem(&mut public_array);
    read_disk(&mut public_array);
}
