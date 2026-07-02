use crate::a_data_source::data::{DataSource, HALF_PADDING_SIZE};
use crate::e_disk::collection::{DISK_AVAIL_END, DISK_AVAIL_START, DISK_TOTAL_END, DISK_TOTAL_START};

pub fn disk_rating(source:&mut DataSource){
    let data_source = &mut source.data_array;
    let gauge_array = &mut source.gauge_array;

    let total_slice: &[u8; HALF_PADDING_SIZE] = (&data_source[DISK_TOTAL_START..DISK_TOTAL_END]).try_into().unwrap();
    let avail_slice: &[u8; HALF_PADDING_SIZE] = (&data_source[DISK_AVAIL_START..DISK_AVAIL_END]).try_into().unwrap();

    let total_to_u64 = u64::from_be_bytes(*total_slice);
    let avail_to_u64 = u64::from_be_bytes(*avail_slice);

    let total_disk = total_to_u64 as f64;
    let avail_disk = avail_to_u64 as f64;
    let used_disk = (total_disk - avail_disk).max(0.0);

    gauge_array[0] = total_disk;
    gauge_array[1] = avail_disk;
    gauge_array[2] = used_disk;


}