use crate::a_data_source::data::{DataSource, DATA_ARRAY_SIZE, HALF_PADDING_SIZE};
use crate::c_mem::collection::{MEMORY_AVAIL_END, MEMORY_AVAIL_START, MEMORY_TOTAL_END, MEMORY_TOTAL_START};

pub fn mem_rating(source:&mut DataSource){
    let data_source:&mut[u8; DATA_ARRAY_SIZE] = &mut source.data_array;
    let gauge_array = &mut source.gauge_array;

    let total_slice: &[u8; HALF_PADDING_SIZE] = (&data_source[MEMORY_TOTAL_START..MEMORY_TOTAL_END]).try_into().unwrap();
    let avail_slice: &[u8; HALF_PADDING_SIZE] = (&data_source[MEMORY_AVAIL_START..MEMORY_AVAIL_END]).try_into().unwrap();

    let total_to_u64 = u64::from_be_bytes(*total_slice);
    let avail_to_u64 = u64::from_be_bytes(*avail_slice);

    let total_mem = total_to_u64 as f64;
    let avail_mem = avail_to_u64 as f64;
    let used_mem = (total_mem - avail_mem).max(0.0);

    gauge_array[0] = total_mem;
    gauge_array[1] = avail_mem;
    gauge_array[2] = used_mem;

}