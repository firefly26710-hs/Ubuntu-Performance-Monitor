use nvml_wrapper::Nvml;
use crate::a_data_source::data;
use crate::a_data_source::data::DataSource;
use crate::b_cpu::collection::read_cpu_info;
use crate::c_mem::collection::read_mem_info;
use crate::e_disk::collection::read_disk_info;
use crate::d_gpu::collection::read_gpu_info;

mod a_data_source;
mod b_cpu;
mod c_mem;
mod d_gpu;
mod e_disk;



fn main() -> Result<(), Box<dyn std::error::Error>>{
    let mut source = DataSource::new();
    let nvml = Nvml::init()?;
    read_cpu_info(&mut source);
    read_mem_info(&mut source);
    read_disk_info(&mut source);
    read_gpu_info(&nvml, &mut source);

    Ok(())
}