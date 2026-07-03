use nvml_wrapper::error::NvmlError;
use nvml_wrapper::Nvml;
use crate::a_data_source::data::{DataSource, HALF_PADDING_SIZE};
use crate::c_mem::logic::mem_logic;
use crate::d_gpu::collection::{gpu_collection, VRAM_AVAIL_END, VRAM_AVAIL_START, VRAM_TOTAL_END, VRAM_TOTAL_START};

pub fn gpu_logic(source:&mut DataSource){
    let data_source = &mut source.data_array;
    let gauge_array = &mut source.gauge_array;

    let total_slice: &[u8; HALF_PADDING_SIZE] = (&data_source[VRAM_TOTAL_START..VRAM_TOTAL_END]).try_into().unwrap();
    let avail_slice: &[u8; HALF_PADDING_SIZE] = (&data_source[VRAM_AVAIL_START..VRAM_AVAIL_END]).try_into().unwrap();

    let total_to_u64 = u64::from_be_bytes(*total_slice);
    let avail_to_u64 = u64::from_be_bytes(*avail_slice);

    let total_vram = total_to_u64 as f64;
    let avail_vram = avail_to_u64 as f64;
    let used_vram = (total_vram - avail_vram).max(0.0);
    
    let total_vram = total_vram / 1024.0 / 1024.0 / 1024.0;
    let avail_vram = avail_vram / 1024.0 / 1024.0 / 1024.0;
    let used_vram = used_vram / 1024.0 / 1024.0 / 1024.0;

    gauge_array[0] = total_vram;
    gauge_array[1] = avail_vram;
    gauge_array[2] = used_vram;

    
    //eprintln!("DEBUG: Total: {}, Avail: {}, Used: {}", gauge_array[0], gauge_array[1], gauge_array[2]);


}

#[test]
fn test_gpu_logic() -> Result<(), NvmlError>{
    let mut source = DataSource::new();
    let nvml = Nvml::init()?;
    gpu_collection(&nvml, &mut source);
    gpu_logic(&mut source);
    let gauge_array = &mut source.gauge_array;
    let check_total_vram = gauge_array[0];
    let check_avail_vram = gauge_array[1];
    let check_used_vram = gauge_array[2];
    eprintln!(" Vram Total : {:.2} , Vram Avail : {:.2}, Vram Used : {:.2} "
              , check_total_vram, check_avail_vram, check_used_vram);

    Ok(())
}