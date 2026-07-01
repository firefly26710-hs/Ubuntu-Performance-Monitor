use crate::a_data_source::data::{DataSource, DATA_ARRAY_SIZE, HALF_PADDING_SIZE, U64_LEN};
use crate::c_mem::collection::{MEMORY_AVAIL_END, MEMORY_AVAIL_START, MEMORY_TOTAL_END, MEMORY_TOTAL_START};

pub fn mem_rating(source:&mut DataSource){
    let data_source:&mut[u8; DATA_ARRAY_SIZE] = &mut source.data_array;
    let history = &mut source.history_array[0];

    let total_slice: &[u8; HALF_PADDING_SIZE] = (&data_source[MEMORY_TOTAL_START..MEMORY_TOTAL_END]).try_into().unwrap();
    let avail_slice: &[u8; HALF_PADDING_SIZE] = (&data_source[MEMORY_AVAIL_START..MEMORY_AVAIL_END]).try_into().unwrap();

    let total_to_u64 = u64::from_be_bytes(*total_slice);
    let avail_to_u64 = u64::from_be_bytes(*avail_slice);

    let total_mem = total_to_u64 as f64;
    let used_mem = (total_to_u64 - avail_to_u64) as f64;


    if total_mem > 0.0 {
        let rating = ( used_mem / total_mem) * 100.0;

        history.copy_within(0..29, 1);
        history[0] = rating;

        println!("history[0]: {:.2}, history[1]: {:.2}, history[2]: {:.2}", history[0], history[1], history[2]);
    }



}