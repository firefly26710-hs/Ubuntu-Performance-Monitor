use nix::libc::statvfs;
use crate::a_data_source::data::{DataSource, PADDING_SIZE};


const QUERY_RANGE:&str = "/";
const DISK_TOTAL_START:usize = 0;
const DISK_TOTAL_END:usize = DISK_TOTAL_START + PADDING_SIZE;
const DISK_IDLE_START:usize = PADDING_SIZE;
const DISK_IDLE_END:usize = DISK_IDLE_START + PADDING_SIZE;
const U64_LEN:usize = 8;

pub fn read_disk_info(source:&mut DataSource) -> Result<(), Box<dyn std::error::Error>> {
    let data_source = &mut source.public_array;

    let mut read: statvfs = unsafe { std::mem::zeroed() };
    if unsafe { statvfs(c"/".as_ptr(), &mut read) } != 0 {
        return Err("statvfs syscall failed".into());
    }


    let block_size = read.f_frsize;
    let total_block = read.f_blocks;
    let avail_block = read.f_bavail;

    let disk_total = total_block * block_size;
    let disk_avail = avail_block * block_size;

    let total_byte_char = disk_total.to_be_bytes();
    let avail_byte_char = disk_avail.to_be_bytes();


    data_source[DISK_TOTAL_START..DISK_TOTAL_END].fill(0);
    data_source[DISK_TOTAL_START..DISK_TOTAL_START+U64_LEN].copy_from_slice(&total_byte_char);

    data_source[DISK_IDLE_START..DISK_IDLE_END].fill(0);
    data_source[DISK_IDLE_START..DISK_IDLE_START+U64_LEN].copy_from_slice(&avail_byte_char);

    let check_total_disk = u64::from_be_bytes(data_source[0..U64_LEN].try_into().unwrap());
    let check_idle_disk = u64::from_be_bytes(data_source[PADDING_SIZE..PADDING_SIZE + U64_LEN].try_into().unwrap());
    println!("-----------");
    println!("DISK TOTAL : {}, DISK AVAIL: {}", check_total_disk, check_idle_disk);
    println!("-----------");


    Ok(())
}