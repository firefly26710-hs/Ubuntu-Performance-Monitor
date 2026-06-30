use crate::a_data_source::data::DataSource;
use crate::e_disk::collection::{DISK_AVAIL_START, DISK_TOTAL_START};

const U64_SIZE: usize = 8;
pub fn disk_rating(source:&mut DataSource){
    let data_source = &mut source.public_array;
    let history = &mut source.history_array[0];

    let total_slice: &[u8; U64_SIZE] = (&data_source[DISK_TOTAL_START..DISK_TOTAL_START + U64_SIZE]).try_into().unwrap();
    let avail_slice: &[u8; U64_SIZE] = (&data_source[DISK_AVAIL_START..DISK_AVAIL_START + U64_SIZE]).try_into().unwrap();

    let total_to_u64 = u64::from_be_bytes(*total_slice);
    let avail_to_u64 = u64::from_be_bytes(*avail_slice);

    let total_disk = total_to_u64 as f64;
    let avail_disk = avail_to_u64 as f64;

    if total_disk > 0.0{
        let rating = (avail_disk / total_disk) * 100.0;

        history.copy_within(0..29, 1);
        history[0] = rating;
        println!("history[0]: {:.2}, history[1]: {:.2}, history[2]: {:.2}", history[0], history[1], history[2]);
    }


}