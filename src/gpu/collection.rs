use std::str::from_utf8;
use nvml_wrapper::Nvml;
use nvml_wrapper::error::NvmlError;
use crate::data_source::data::{DataSource, PADDING_SIZE};

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

    data_source[0..PADDING_SIZE].fill(0);
    data_source[0..name.len()].copy_from_slice(&name_byte_char);

    data_source[PADDING_SIZE..PADDING_SIZE * 2].fill(0);
    data_source[PADDING_SIZE..PADDING_SIZE + total_length].copy_from_slice(&total_byte_char);

    data_source[PADDING_SIZE * 2..PADDING_SIZE * 3].fill(0);
    data_source[PADDING_SIZE * 2..PADDING_SIZE * 2 + avail_length].copy_from_slice(&avail_byte_char);

    let check_gpu_name = from_utf8(&data_source[0..name_length]).unwrap();
    let check_vram_total = u64::from_be_bytes(data_source[PADDING_SIZE..PADDING_SIZE + 8].try_into().unwrap());
    let check_vram_avail = u64::from_be_bytes(data_source[PADDING_SIZE * 2..PADDING_SIZE * 2 + 8].try_into().unwrap());

    println!("-----------");
    println!("GPU NAME : {}\nVRAM TOTAL : {}\nVRAM AVAIL : {}", check_gpu_name, check_vram_total, check_vram_avail);
    println!("-----------");
    Ok(())
}
