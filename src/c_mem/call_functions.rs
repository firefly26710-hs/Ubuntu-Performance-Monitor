use ratatui::Frame;
use crate::a_data_source::data::DataSource;
use crate::c_mem::collection::mem_collection;
use crate::c_mem::logic::mem_logic;
use crate::c_mem::present::mem_present;

pub fn mem_call(f: &mut Frame, source: &mut DataSource){
    mem_collection(source);
    mem_logic(source);
    mem_present(f, source);
}