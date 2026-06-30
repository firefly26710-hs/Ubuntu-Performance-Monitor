use std::str::from_utf8;
use nvml_wrapper::Nvml;
use nvml_wrapper::error::NvmlError;
use crate::a_data_source::data::{DataSource, PADDING_SIZE};

const GPU_NAME_START:usize = 0;
const GPU_NAME_END:usize = GPU_NAME_START + PADDING_SIZE;
pub const VRAM_TOTAL_START:usize = PADDING_SIZE;
const VRAM_TOTAL_END:usize = VRAM_TOTAL_START + PADDING_SIZE;
pub const VRAM_AVAIL_START:usize = PADDING_SIZE*2;
const VRAM_AVAIL_END:usize = VRAM_AVAIL_START + PADDING_SIZE;



pub fn read_gpu_info(nvml: &Nvml,source: &mut DataSource)-> Result<(), NvmlError> {
    let data_source = &mut source.public_array;

    let device = nvml.device_by_index(0)?;
    let vram_info = device.memory_info()?;

    let name = device.name()?;
    let vram_total = vram_info.total;
    let vram_avail = vram_info.free;

    let name_byte_char = name.as_bytes();
    let total_byte_char = vram_total.to_be_bytes();
    let avail_byte_char = vram_avail.to_be_bytes();

    let name_length = name_byte_char.len();
    let total_length = total_byte_char.len();
    let avail_length = avail_byte_char.len();

    data_source[GPU_NAME_START..GPU_NAME_END].fill(0);
    data_source[GPU_NAME_START..GPU_NAME_START+name.len()].copy_from_slice(&name_byte_char);

    data_source[VRAM_TOTAL_START..VRAM_TOTAL_END].fill(0);
    data_source[VRAM_TOTAL_START..VRAM_TOTAL_START + total_length].copy_from_slice(&total_byte_char);

    data_source[VRAM_AVAIL_START..VRAM_AVAIL_END].fill(0);
    data_source[VRAM_AVAIL_START..VRAM_AVAIL_START + avail_length].copy_from_slice(&avail_byte_char);

    let check_gpu_name = from_utf8(&data_source[0..name_length]).unwrap();
    let check_vram_total = u64::from_be_bytes(data_source[PADDING_SIZE..PADDING_SIZE + 8].try_into().unwrap());
    let check_vram_avail = u64::from_be_bytes(data_source[PADDING_SIZE * 2..PADDING_SIZE * 2 + 8].try_into().unwrap());

    println!("-----------");
    println!("GPU NAME : {}\nVRAM TOTAL : {}\nVRAM AVAIL : {}", check_gpu_name, check_vram_total, check_vram_avail);
    println!("-----------");
    Ok(())
}
