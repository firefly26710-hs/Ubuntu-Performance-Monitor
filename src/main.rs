use nvml_wrapper::Nvml;
use crate::a_data_source::data::DataSource;
use std::thread;
use std::time::Duration;
use nix::libc::statvfs;

mod a_data_source;
mod b_cpu;
mod c_mem;
mod d_gpu;
mod e_disk;


use crate::b_cpu::collection::read_cpu_info;
use crate::b_cpu::logic::cpu_rating;

use crate::c_mem::collection::read_mem_info;
use crate::c_mem::logic::mem_rating;

use crate::d_gpu::collection::read_gpu_info;
use crate::d_gpu::logic::gpu_rating;

use crate::e_disk::collection::read_disk_info;
use crate::e_disk::logic::disk_rating;


fn main() -> Result<(), Box<dyn std::error::Error>>{
    let mut source = DataSource::new();
    let nvml = Nvml::init()?;
    let mut read: statvfs = unsafe { std::mem::zeroed() };
    if unsafe { statvfs(c"/".as_ptr(), &mut read) } != 0 {
        return Err("statvfs syscall failed".into());
    }
    loop {
        read_cpu_info(&mut source);
        cpu_rating(&mut source);
        thread::sleep(Duration::new(1,0));
    }

    Ok(())
}
