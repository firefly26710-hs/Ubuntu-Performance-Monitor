use nix::libc::statvfs;
use crate::a_data_source::data::{DataSource, PADDING_SIZE};


const QUERY_RANGE:&str = "/";
pub const DISK_TOTAL_START:usize = 0;
const DISK_TOTAL_END:usize = DISK_TOTAL_START + PADDING_SIZE;
pub const DISK_AVAIL_START:usize = PADDING_SIZE;
const DISK_AVAIL_END:usize = DISK_AVAIL_START + PADDING_SIZE;
const U64_LEN:usize = 8;

pub fn read_disk_info(read:&statvfs,source:&mut DataSource) -> Result<(), Box<dyn std::error::Error>> {
    let data_source = &mut source.data_array;

    
    let block_size = read.f_frsize;
    let total_block = read.f_blocks;
    let avail_block = read.f_bavail;

    let disk_total = total_block * block_size;
    let disk_avail = avail_block * block_size;

    let total_slice = disk_total.to_be_bytes();
    let avail_slice = disk_avail.to_be_bytes();
    
    let total_length = U64_LEN;
    let avail_length = U64_LEN;
    
    data_source[DISK_TOTAL_START..DISK_TOTAL_END].fill(0);
    data_source[DISK_TOTAL_START..DISK_TOTAL_START+total_length].copy_from_slice(&total_slice);

    data_source[DISK_AVAIL_START..DISK_AVAIL_END].fill(0);
    data_source[DISK_AVAIL_START..DISK_AVAIL_START +avail_length].copy_from_slice(&avail_slice);

    let check_total_disk = u64::from_be_bytes(data_source[0..total_length].try_into()?);
    let check_avail_disk = u64::from_be_bytes(data_source[PADDING_SIZE..PADDING_SIZE + avail_length].try_into()?);
    println!("-----------");
    println!("DISK TOTAL : {}, DISK AVAIL: {}", check_total_disk, check_avail_disk);
    println!("-----------");


    Ok(())
}