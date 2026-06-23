use crate::cpu::classifier::read_cpu_info;
use crate::data_source::data::DataSource;
mod data_source;
mod cpu;
mod disk;

fn main(){
    let mut source = DataSource::new();
    read_cpu_info(&mut source);

}