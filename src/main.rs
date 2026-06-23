use crate::data_source::data::DataSource;
use crate::cpu::classifier::information_cpu;
mod data_source;
mod cpu;
mod disk;

fn main(){
    let mut source = DataSource::new();
    source.read_cpu_name();
    information_cpu::catch_cpu_name(&mut source);
}