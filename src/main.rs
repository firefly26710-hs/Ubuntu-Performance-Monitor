use nvml_wrapper::Nvml;
use crate::a_data_source::data::DataSource;
use nix::libc::{statvfs};

use crossterm::{
    terminal::{enable_raw_mode, EnterAlternateScreen},
    ExecutableCommand,
};
use ratatui::{backend::CrosstermBackend, Terminal};
use std::io::{stdout, Stdout};


mod a_data_source;
mod b_cpu;
mod c_mem;
mod d_gpu;
mod e_disk;

use crate::b_cpu::call_functions::cpu_call;
use crate::c_mem::call_functions::mem_call;
use crate::d_gpu::call_functions::gpu_call;
use crate::e_disk::call_functions::disk_call;

fn init()->(DataSource, Nvml, statvfs, Terminal<CrosstermBackend<Stdout>>) {
    enable_raw_mode().expect("Failed to enable raw mode");
    let mut stdout = stdout();
    stdout.execute(EnterAlternateScreen).expect("Failed to enter alt screen");

    let source = DataSource::new();
    let nvml = Nvml::init().expect("Failed to init NVML");
    let mut read: statvfs = unsafe { std::mem::zeroed() };
    unsafe { statvfs("/\0".as_ptr() as *const i8, &mut read); }
    let terminal = Terminal::new(CrosstermBackend::new(stdout)).expect("Failed to init Terminal");

    ( source, nvml, read, terminal )

}
fn main() -> Result<(), Box<dyn std::error::Error>>{
    let (mut source, nvml, read, mut terminal) = init();


    loop {
        terminal.draw(|f| {
            gpu_call(f, &nvml, &mut source);
       })?;
    }


}