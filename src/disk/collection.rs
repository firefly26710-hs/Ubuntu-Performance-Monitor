use nix::sys::statvfs::statvfs;
use crate::data_source::data::{DataSource, PADDING_SIZE};


const QUERY_RANGE:&str = "/";
const DISK_TOTAL_START:usize = 0;
const DISK_TOTAL_END:usize = DISK_TOTAL_START + PADDING_SIZE;
const DISK_IDLE_START:usize = PADDING_SIZE;
const DISK_IDLE_END:usize = DISK_IDLE_START + PADDING_SIZE;
pub fn read_disk_info(source:&mut DataSource) {
    let data_source = &mut source.public_array;

    let path = QUERY_RANGE;
    if let Ok(read) = statvfs(path) {
        let block_size = read.fragment_size();
        let total_block = read.blocks();
        let avail_block = read.blocks_available();

        let disk_total = total_block * block_size;
        let disk_avail = avail_block * block_size;


        let total_byte_char = disk_total.to_be_bytes();
        let avail_byte_char = disk_avail.to_be_bytes();

        let len_total = total_byte_char.len();
        let len_avail = avail_byte_char.len();


        data_source[DISK_TOTAL_START..DISK_TOTAL_END].fill(0);
        data_source[DISK_TOTAL_START..DISK_TOTAL_START+len_total].copy_from_slice(&total_byte_char);

        data_source[DISK_IDLE_START..DISK_IDLE_END].fill(0);
        data_source[DISK_IDLE_START..DISK_IDLE_START+len_avail].copy_from_slice(&avail_byte_char);

        let check_total_disk = u64::from_be_bytes(data_source[0..8].try_into().unwrap());
        let check_idle_disk = u64::from_be_bytes(data_source[PADDING_SIZE..PADDING_SIZE + 8].try_into().unwrap());
        println!("-----------");
        println!("DISK TOTAL : {}, DISK AVAIL: {}", check_total_disk, check_idle_disk);
        println!("-----------");

    }

}