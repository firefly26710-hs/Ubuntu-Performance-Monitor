use std::thread;
use std::time::Duration;
use ratatui::Frame;
use crate::a_data_source::data::DataSource;
use crate::a_data_source::hpl::thread_number;
use crate::b_cpu::collection::cpu_collection;
use crate::b_cpu::logic::cpu_logic;
use crate::b_cpu::present::cpu_present;

pub fn cpu_call(f: &mut Frame, source: &mut DataSource){
    cpu_collection(source, thread_number());
    cpu_logic(source, thread_number());
    cpu_present(f, source);
}