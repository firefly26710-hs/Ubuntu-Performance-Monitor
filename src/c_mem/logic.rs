use crate::a_data_source::data::{DataSource, DATA_ARRAY_SIZE, HALF_PADDING_SIZE};
use crate::c_mem::collection::{mem_collection, MEMORY_AVAIL_END, MEMORY_AVAIL_START, MEMORY_TOTAL_END, MEMORY_TOTAL_START};

pub fn mem_logic(source:&mut DataSource){
    let data_source:&mut[u8; DATA_ARRAY_SIZE] = &mut source.data_array;
    let gauge_array = &mut source.gauge_array;

    let total_slice: &[u8; HALF_PADDING_SIZE] = (&data_source[MEMORY_TOTAL_START..MEMORY_TOTAL_END]).try_into().unwrap();
    let avail_slice: &[u8; HALF_PADDING_SIZE] = (&data_source[MEMORY_AVAIL_START..MEMORY_AVAIL_END]).try_into().unwrap();

    let total_to_u64 = u64::from_be_bytes(*total_slice);
    let avail_to_u64 = u64::from_be_bytes(*avail_slice);

    let total_mem = total_to_u64 as f64;
    let avail_mem = avail_to_u64 as f64;
    let used_mem = (total_mem - avail_mem).max(0.0);
    
    let total_mem = total_mem / 1024.0 / 1024.0;
    let avail_mem = avail_mem / 1024.0 / 1024.0;
    let used_mem = used_mem / 1024.0 / 1024.0;

    gauge_array[0] = total_mem;
    gauge_array[1] = avail_mem;
    gauge_array[2] = used_mem;

}

#[test]
fn test_mem_logic(){
    let mut source = DataSource::new();
    mem_collection(&mut source);
    mem_logic(&mut source);
    let gauge_array = &mut source.gauge_array;
    let check_total_memory = gauge_array[0];
    let check_avail_memory = gauge_array[1];
    let check_used_memory = gauge_array[2];
    eprintln!(" Memory Total : {:.2} , Memory Avail : {:.2}, Memory Used : {:.2} "
              , check_total_memory, check_avail_memory, check_used_memory);
}