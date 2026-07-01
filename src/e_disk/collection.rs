use nix::libc::statvfs;
use crate::a_data_source::data::{DataSource, HALF_PADDING_SIZE, PADDING_SIZE};


pub const DISK_TOTAL_START:usize = 0;
pub const DISK_TOTAL_END:usize = HALF_PADDING_SIZE;
pub const DISK_AVAIL_START:usize = HALF_PADDING_SIZE;
pub const DISK_AVAIL_END:usize = PADDING_SIZE;

pub fn read_disk_info(read:&statvfs,source:&mut DataSource) -> Result<(), Box<dyn std::error::Error>> {
    let data_source = &mut source.data_array;

    
    let block_size = read.f_frsize;
    let total_block = read.f_blocks;
    let avail_block = read.f_bavail;

    let disk_total = total_block * block_size;
    let disk_avail = avail_block * block_size;

    let total_slice = disk_total.to_be_bytes();
    let avail_slice = disk_avail.to_be_bytes();
    
    data_source[DISK_TOTAL_START..DISK_TOTAL_END].fill(0);
    data_source[DISK_TOTAL_START..DISK_TOTAL_END].copy_from_slice(&total_slice);

    data_source[DISK_AVAIL_START..DISK_AVAIL_END].fill(0);
    data_source[DISK_AVAIL_START..DISK_AVAIL_END].copy_from_slice(&avail_slice);

    let check_total_disk = u64::from_be_bytes(data_source[DISK_TOTAL_START..DISK_TOTAL_END].try_into()?);
    let check_avail_disk = u64::from_be_bytes(data_source[DISK_AVAIL_START..DISK_AVAIL_END].try_into()?);
    println!("-----------");
    println!("DISK TOTAL : {}, DISK AVAIL: {}", check_total_disk, check_avail_disk);
    println!("-----------");


    Ok(())
}