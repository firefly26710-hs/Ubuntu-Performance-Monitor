use nix::sys::statvfs::statvfs;
use crate::data_source::data::{DataSource, PADDING_SIZE};

pub fn read_disk_info(source:&mut DataSource) {
    let data_source = &mut source.public_array;


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
        data_source[PADDING_SIZE..2 * PADDING_SIZE].copy_from_slice(&buffer_array[0..PADDING_SIZE]);

        buffer_array[0..len_total].copy_from_slice(&total_byte_char[0..len_total]);
        data_source[0..PADDING_SIZE].copy_from_slice(&buffer_array[0..PADDING_SIZE]);

        let total_restored = u64::from_be_bytes(data_source[0..8].try_into().unwrap());
        let avail_restored = u64::from_be_bytes(data_source[PADDING_SIZE..PADDING_SIZE + 8].try_into().unwrap());
        println!("-----------");
        println!("Total: {}", total_restored);
        println!("Avail: {}", avail_restored);
        println!("-----------");

    }

}