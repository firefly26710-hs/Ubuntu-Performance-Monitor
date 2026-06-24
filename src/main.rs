use crate::cpu::collection::read_cpu_info;
use crate::disk::collection::read_disk_info;
use crate::data_source::data::DataSource;
use crate::mem::collection::read_mem_info;

mod data_source;
mod cpu;
mod disk;
mod mem;

fn main(){
    let mut source = DataSource::new();
    read_cpu_info(&mut source);
    read_mem_info(&mut source);
    read_disk_info(&mut source);

}