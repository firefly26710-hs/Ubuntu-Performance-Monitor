use crate::a_data_source::data::{DataSource, HALF_PADDING_SIZE};
use crate::d_gpu::collection::{VRAM_AVAIL_END, VRAM_AVAIL_START, VRAM_TOTAL_END, VRAM_TOTAL_START};

pub fn gpu_logic(source:&mut DataSource){
    let data_source = &mut source.data_array;
    let gauge_array = &mut source.gauge_array;

    let total_slice: &[u8; HALF_PADDING_SIZE] = (&data_source[VRAM_TOTAL_START..VRAM_TOTAL_END]).try_into().unwrap();
    let avail_slice: &[u8; HALF_PADDING_SIZE] = (&data_source[VRAM_AVAIL_START..VRAM_AVAIL_END]).try_into().unwrap();

    let total_to_u64 = u64::from_be_bytes(*total_slice);
    let avail_to_u64 = u64::from_be_bytes(*avail_slice);

    let total_vram = total_to_u64 as f64;
    let avail_vram = avail_to_u64 as f64;
    let used_vram = (total_vram - avail_vram).max(0.0) as f64 ;

    gauge_array[0] = total_vram;
    gauge_array[1] = avail_vram;
    gauge_array[2] = used_vram;

    
    //eprintln!("DEBUG: Total: {}, Avail: {}, Used: {}", gauge_array[0], gauge_array[1], gauge_array[2]);


}