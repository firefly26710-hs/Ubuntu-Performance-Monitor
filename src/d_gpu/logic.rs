use crate::a_data_source::data::{DataSource, HALF_PADDING_SIZE, U64_LEN};
use crate::d_gpu::collection::{VRAM_AVAIL_END, VRAM_AVAIL_START, VRAM_TOTAL_END, VRAM_TOTAL_START};

pub fn gpu_rating(source:&mut DataSource){
    let data_source = &mut source.data_array;
    let history = &mut source.history_array[0];

    let total_slice: &[u8; HALF_PADDING_SIZE] = (&data_source[VRAM_TOTAL_START..VRAM_TOTAL_END]).try_into().unwrap();
    let avail_slice: &[u8; U64_LEN] = (&data_source[VRAM_AVAIL_START..VRAM_AVAIL_END]).try_into().unwrap();

    let total_to_u64 = u64::from_be_bytes(*total_slice);
    let avail_to_u64 = u64::from_be_bytes(*avail_slice);

    let total_vram = total_to_u64 as f64;
    let used_vram = (total_to_u64 - avail_to_u64) as f64;

    if total_vram > 0.0{
        let rating = (used_vram / total_vram) * 100.0;

        history.copy_within(0..29, 1);
        history[0] = rating;
        println!("history[0]: {:.2}, history[1]: {:.2}, history[2]: {:.2}", history[0], history[1], history[2]);
    }


}