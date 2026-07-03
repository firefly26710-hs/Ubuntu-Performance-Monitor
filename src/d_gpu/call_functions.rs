
use nvml_wrapper::Nvml;
use ratatui::Frame;
use crate::a_data_source::data::DataSource;
use crate::d_gpu::collection::gpu_collection;
use crate::d_gpu::logic::gpu_logic;
use crate::d_gpu::present::gpu_present;
pub fn gpu_call(f: &mut Frame,nvml: &Nvml, source: &mut DataSource){ 
    gpu_collection(nvml, source);
    gpu_logic(source);
    gpu_present(f, source);

}