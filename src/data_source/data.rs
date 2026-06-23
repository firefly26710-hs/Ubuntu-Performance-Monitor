use std::fs::File;
use std::io::{BufRead, BufReader};
use std::str::from_utf8;
use nix::sys::statvfs::statvfs;

//check table :
// CPU:
//------------------------------------------------
//  cpu_name |  thread0  | thread1 | thread2 | ...
//-----------------------------------------------
//PADDING:  0           1         2         3

// Memory:
//------------------------------------------------
//  Memory Total |  Memory avail  |
//-----------------------------------------------
//PADDING:       0                1

// Disk:
//------------------------------------------------
//  Disk Total |  Disk Avail  |
//-----------------------------------------------
//PADDING:     0              1



const PADDING_SIZE:usize = 64;
const HALF_SIZE:usize = PADDING_SIZE / 2;
const PADDING_NUMBER:usize = 13;
const MAX_PUBLIC_ARRAY_SIZE:usize = PADDING_NUMBER * PADDING_SIZE;


const TOTAL_MEMORY_INFO:usize = 0;
const AVAILABLE_MEMORY_INFO:usize = 2;

const THREAD_START:usize = 64;
const THREAD_NUMBER:usize = 12;

const NAME_INFO_START:usize = 0;
const NAME_INFO_END: usize = PADDING_SIZE - NAME_INFO_START;
const THREAD_SIZE:usize = THREAD_NUMBER * PADDING_SIZE;


pub struct DataSource{
    pub public_array:[u8; MAX_PUBLIC_ARRAY_SIZE]
}

impl DataSource {
    pub fn new() -> Self{
        Self{public_array: [0; MAX_PUBLIC_ARRAY_SIZE]}
    }
    pub fn read_cpu_name(&mut self) {
        if let Ok(cpuinfo_file) = File::open("/proc/cpuinfo") {
            let cpuinfo_reader = BufReader::new(cpuinfo_file);
            if let Some(Ok(name_info)) = cpuinfo_reader.lines().nth(4) {
                let name = match name_info.split(':').nth(1){
                    Some(name) => name.trim(),
                    None => name_info.trim(),
                };
                let name_len = name.len();
                let byte_char = name.as_bytes();
                let mut buffer_array = [0u8; PADDING_SIZE];
                buffer_array[0..name_len].copy_from_slice(&byte_char[0..name_len]);
                self.public_array[0..PADDING_SIZE].copy_from_slice(&buffer_array);

                println!("-----------");
                println!("{}", from_utf8(&self.public_array[0..PADDING_SIZE]).unwrap());
                println!("-----------");
            }
        }

        if let Ok(stat_file) = File::open("/proc/stat") {
            let stat_reader = BufReader::new(stat_file);
            for (number, thread) in stat_reader.lines().skip(1).take(THREAD_NUMBER).enumerate() {
                if let Ok(thread_info) = thread {
                    let mut parts = thread_info.split_whitespace();
                    parts.next();

                    let mut current_total: u64 = 0;
                    let mut current_idle: u64 = 0;
                    for(index, slice) in parts.enumerate(){
                        if let Ok(val) = slice.parse::<u64>(){
                            current_total+=val;
                            if(index == 3){
                                current_idle = val;
                            }
                        }
                    }

                    let total_byte_char = current_total.to_be_bytes();
                    let idle_byte_char = current_idle.to_be_bytes();

                    let len_total = total_byte_char.len();
                    let len_idle = idle_byte_char.len();

                    let offest = number * PADDING_SIZE;
                    let start = THREAD_START + offest;
                    let end = start + PADDING_SIZE;
                    let mid = (start + end) / 2;


                    let mut buffer_padding = [0u8; HALF_SIZE];

                    buffer_padding[0..len_idle].copy_from_slice(&idle_byte_char[0..len_idle]);
                    self.public_array[start..mid].copy_from_slice(&buffer_padding);

                    buffer_padding[0..len_total].copy_from_slice(&total_byte_char[0..len_total]);
                    self.public_array[mid..end].copy_from_slice(&buffer_padding);


                    let check_idle = u64::from_be_bytes(self.public_array[start..start+8].try_into().unwrap());
                    let check_total = u64::from_be_bytes(self.public_array[mid..mid+8].try_into().unwrap());
                    println!("-----------");
                    println!("Thread {} -> Idle(前半): {}, Total(後半): {}", number, check_idle, check_total);
                    println!("-----------");
                }
            }
        }
    }


    pub fn read_mem(&mut self) {
        if let Ok(meminfo_file) = File::open("/proc/meminfo") {
            let meminfo_reader = BufReader::new(meminfo_file);
            for (number, info) in meminfo_reader.lines().take(3).enumerate() {
                if let Ok(info) = info {
                    let actual_length = info.len();
                    let byte_char = info.as_bytes();
                    let mut buffer_padding = [0u8; PADDING_SIZE];
                    buffer_padding[0..actual_length].copy_from_slice(&byte_char[0..actual_length]);
                    match number {
                        TOTAL_MEMORY_INFO
                        => self.public_array[0..PADDING_SIZE].copy_from_slice(&buffer_padding),

                        AVAILABLE_MEMORY_INFO
                        => self.public_array[PADDING_SIZE..2 * PADDING_SIZE].copy_from_slice(&buffer_padding),

                        _
                        => {}
                    }

                    match number {
                        0 => {
                            println!("-----------");
                            println!("{}\n", from_utf8(&self.public_array[0..PADDING_SIZE]).unwrap());
                            println!("-----------");
                        },
                        2 => {
                            println!("-----------");
                            println!("{}\n", from_utf8(&self.public_array[PADDING_SIZE..2 * PADDING_SIZE]).unwrap());
                            println!("-----------");
                        },
                        _ => {}
                    }
                }
            }
        }
    }


    pub fn read_disk(&mut self) {
        let path = "/";
        if let Ok(statvfs) = statvfs(path) {
            let f_frsize = statvfs.fragment_size();
            let f_blocks = statvfs.blocks();
            let f_bavail = statvfs.blocks_available();

            let disk_total = f_blocks * f_frsize;
            let disk_avail = f_bavail * f_frsize;


            let total_byte_char = disk_total.to_be_bytes();
            let avail_byte_char = disk_avail.to_be_bytes();

            let len_total = total_byte_char.len();
            let len_avail = avail_byte_char.len();

            let mut buffer_array = [0u8; PADDING_SIZE];

            buffer_array[0..len_avail].copy_from_slice(&avail_byte_char[0..len_avail]);
            self.public_array[PADDING_SIZE..2 * PADDING_SIZE].copy_from_slice(&buffer_array[0..PADDING_SIZE]);

            buffer_array[0..len_total].copy_from_slice(&total_byte_char[0..len_total]);
            self.public_array[0..PADDING_SIZE].copy_from_slice(&buffer_array[0..PADDING_SIZE]);

            let total_restored = u64::from_be_bytes(self.public_array[0..8].try_into().unwrap());
            let avail_restored = u64::from_be_bytes(self.public_array[PADDING_SIZE..PADDING_SIZE + 8].try_into().unwrap());
            println!("-----------");
            println!("Total: {}", total_restored);
            println!("Avail: {}", avail_restored);
            println!("-----------");

        }

    }
}

