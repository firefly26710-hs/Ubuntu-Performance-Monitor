use crate::a_data_source::data::{DataSource, MAX_PUBLIC_ARRAY_SIZE, PADDING_SIZE};
use crate::c_mem::collection::{MEMORY_AVAIL_START, MEMORY_TOTAL_START};

const U64_SIZE: usize = 8;
pub fn cal(source:&mut DataSource){
    let data_source:&mut[u8; MAX_PUBLIC_ARRAY_SIZE] = &mut source.public_array;
    let history = &mut source.history_array[0];

    let n_total_slice: &[u8; U64_SIZE] = (&data_source[MEMORY_TOTAL_START..MEMORY_TOTAL_START + U64_SIZE]).try_into().unwrap();
    let n_avail_slice: &[u8; U64_SIZE] = (&data_source[MEMORY_AVAIL_START..MEMORY_AVAIL_START + U64_SIZE]).try_into().unwrap();

    let n_total = u64::from_be_bytes(*n_total_slice);
    let n_avail = u64::from_be_bytes(*n_avail_slice);

    let used = (n_total - n_avail) as f64;
    let total = n_total as f64;

    if total > 0.0 {
        let rating = ( used / total ) * 100.0;

        history.copy_within(0..29, 1);
        history[0] = rating;

        println!("history1 : {:.2}, history2 : {:.2}", history[0], history[1]);
    }



}