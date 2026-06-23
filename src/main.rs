use crate::data_source::data::DataSource;
mod data_source;
mod cpu;
mod disk;

fn main(){
    let mut source = DataSource::new();
    source.read_cpu_name();
    
}