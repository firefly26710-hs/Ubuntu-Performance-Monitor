use nvml_wrapper::Nvml;
use crate::data_source::data::DataSource;
use crate::cpu::collection::read_cpu_info;
use crate::mem::collection::read_mem_info;
use crate::disk::collection::read_disk_info;
use crate::gpu::collection::read_gpu_info;

mod data_source;
mod cpu;
mod disk;
mod mem;
mod gpu;

fn main() -> Result<(), Box<dyn std::error::Error>>{
    let mut source = DataSource::new();
    let nvml = Nvml::init()?;
    read_cpu_info(&mut source);
    read_mem_info(&mut source);
    read_disk_info(&mut source);
    read_gpu_info(&nvml, &mut source);

    Ok(())
}