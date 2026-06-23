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
const MAX_PUBLIC_ARRAY_SIZE:usize = PADDING_SIZE*13;
const THREAD_START:usize = 64;
const THREAD_NUMBER:usize = 12;
const TOTAL_MEMORY_INFO:usize = 0;
const AVAILABLE_MEMORY_INFO:usize = 2;


pub struct DataSource{
    pub public_array:[u8; MAX_PUBLIC_ARRAY_SIZE]
}

impl DataSource {
    fn new() -> Self{
        Self{public_array: [0; MAX_PUBLIC_ARRAY_SIZE]}
    }
    fn read_cpu_name(&mut self) {
        if let Ok(cpuinfo_file) = File::open("/proc/cpuinfo") {
            let cpuinfo_reader = BufReader::new(cpuinfo_file);
            if let Some(Ok(name_info)) = cpuinfo_reader.lines().nth(4) {
                let actual_length = name_info.len();
                let byte_char = name_info.as_bytes();

                let mut buffer_array = [0u8; PADDING_SIZE];
                buffer_array[0..actual_length].copy_from_slice(&byte_char[0..actual_length]);
                self.public_array[0..PADDING_SIZE].copy_from_slice(&buffer_array);

                println!("-----------");
                println!("{}", from_utf8(&self.public_array[0..PADDING_SIZE]).unwrap());
                println!("-----------");
            }
        }
    }

    fn read_thread(&mut self) {
        if let Ok(stat_file) = File::open("/proc/stat") {
            let stat_reader = BufReader::new(stat_file);
            for (number, thread) in stat_reader.lines().skip(1).take(THREAD_NUMBER).enumerate() {
                if let Ok(thread_info) = thread {
                    let actual_length = thread_info.len();
                    let byte_char = thread_info.as_bytes();

                    let offest = number * PADDING_SIZE;
                    let start = THREAD_START + offest;
                    let end = start + PADDING_SIZE;

                    let mut buffer_padding = [0u8; PADDING_SIZE];
                    buffer_padding[0..actual_length].copy_from_slice(&byte_char[0..actual_length]);
                    self.public_array[start..end].copy_from_slice(&buffer_padding);
                    println!("-----------");
                    println!("{}", from_utf8(&self.public_array[start..end]).unwrap());
                    println!("-----------");
                }
            }
        }
    }

    fn read_mem(&mut self) {
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


    fn read_disk(&mut self) {
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


#[test]
fn test_proc_reading() { // file reading exp
    let mut source = DataSource::new();
    source.read_cpu_name();
    source.read_thread();
    source.read_mem();
    source.read_disk();
}
