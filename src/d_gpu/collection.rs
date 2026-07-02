use std::str::from_utf8;
use nvml_wrapper::Nvml;
use nvml_wrapper::error::NvmlError;
use crate::a_data_source::data::{DataSource, HALF_PADDING_SIZE, NAME_ARRAY_SIZE, PADDING_SIZE};

pub const VRAM_TOTAL_START:usize = 0;
pub const VRAM_TOTAL_END:usize = HALF_PADDING_SIZE;
pub const VRAM_AVAIL_START:usize = HALF_PADDING_SIZE;
pub const VRAM_AVAIL_END:usize = PADDING_SIZE;


pub fn gpu_collection(nvml: &Nvml, source: &mut DataSource) -> Result<(), NvmlError> {
    let name_array = &mut source.name_array;
    let data_array = &mut source.data_array;

    let device = nvml.device_by_index(0)?;
    let vram_info = device.memory_info()?;

    let name = device.name()?;
    let vram_total = vram_info.total;
    let vram_avail = vram_info.free;

    let name_slice = name.as_bytes();
    let name_length = name_slice.len().min(NAME_ARRAY_SIZE);


    let total_slice = vram_total.to_be_bytes();
    let avail_slice = vram_avail.to_be_bytes();



    name_array.fill(0);
    name_array[0..name_length].copy_from_slice(&name_slice);

    data_array[VRAM_TOTAL_START..VRAM_TOTAL_END].fill(0);
    data_array[VRAM_TOTAL_START..VRAM_TOTAL_END].copy_from_slice(&total_slice);

    data_array[VRAM_AVAIL_START..VRAM_AVAIL_END].fill(0);
    data_array[VRAM_AVAIL_START..VRAM_AVAIL_END].copy_from_slice(&avail_slice);

    let check_gpu_name = from_utf8(&name_array[0..name_length]).unwrap();
    let check_vram_total = u64::from_be_bytes(data_array[VRAM_TOTAL_START..VRAM_TOTAL_END].try_into().unwrap());
    let check_vram_avail = u64::from_be_bytes(data_array[VRAM_AVAIL_START..VRAM_AVAIL_END].try_into().unwrap());

    //println!("-----------");
    //println!("GPU NAME : {}, VRAM TOTAL : {}, VRAM AVAIL : {}", check_gpu_name, check_vram_total, check_vram_avail);
    //println!("-----------");
    Ok(())
}
