use crate::a_data_source::data::{DataSource, MAX_PUBLIC_ARRAY_SIZE, PADDING_SIZE};
use crate::c_mem::collection::{MEMORY_AVAIL_START, MEMORY_TOTAL_START};


const OLD_MEMORY_TOTAL_START:usize = PADDING_SIZE*2;
const OLD_MEMORY_AVAIL_START:usize = PADDING_SIZE*3;
const U64_SIZE: usize = 8;
pub fn cal(source:&mut DataSource){
    let data_source:&mut[u8; MAX_PUBLIC_ARRAY_SIZE] = &mut source.public_array;


    let n_total_slice: &[u8; U64_SIZE] = (&data_source[MEMORY_TOTAL_START..MEMORY_TOTAL_START + U64_SIZE]).try_into().unwrap();
    let n_avail_slice: &[u8; U64_SIZE] = (&data_source[MEMORY_AVAIL_START..MEMORY_AVAIL_START + U64_SIZE]).try_into().unwrap();


    let n_total = u64::from_be_bytes(*n_total_slice);
    let n_avail = u64::from_be_bytes(*n_avail_slice);

    let used = (n_total - n_avail) as f64;
    let total = n_total as f64;

    if total > 0.0 {
        let rating = ( used / total ) * 100.0;

        source.history_array.copy_within(0..29, 1);

        source.history_array[0] = rating;

        println!("history1 : {}, history2 : {}", &source.history_array[0], &source.history_array[1]);
    }



}