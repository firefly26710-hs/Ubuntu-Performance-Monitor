extern crate core;

use nvml_wrapper::Nvml;
use crate::a_data_source::data::DataSource;
use nix::libc::statvfs;
use crossterm::event::{self, Event, KeyCode};
use std::time::Duration;

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
enum MonitorPage{
    Cpu,
    Memory,
    Gpu,
    Disk
}

impl MonitorPage{
    fn next(self) -> Self {
        match self {
            MonitorPage::Cpu => MonitorPage::Memory,
            MonitorPage::Memory => MonitorPage::Gpu,
            MonitorPage::Gpu => MonitorPage::Disk,
            MonitorPage::Disk => MonitorPage::Cpu
        }
    }

    fn prev(self) -> Self {
        match self {
            MonitorPage::Cpu => MonitorPage::Disk,
            MonitorPage::Disk => MonitorPage::Gpu,
            MonitorPage::Gpu => MonitorPage::Memory,
            MonitorPage::Memory => MonitorPage::Cpu
        }
    }
}



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
    let (mut source, nvml, read, mut ter) = init();
    let mut current_page = MonitorPage::Cpu;
    ter.draw(|f| {
        cpu_call(f,&mut source);
    })?;



    loop {
        ter.draw(|f| {
            match current_page {
                MonitorPage::Cpu => { cpu_call(f, &mut source); }
                MonitorPage::Memory => { mem_call(f, &mut source); }
                MonitorPage::Gpu => { gpu_call(f, &nvml, &mut source); }
                MonitorPage::Disk => { disk_call(f, &read, &mut source); }
            }
        })?;

        if event::poll(Duration::from_secs(1))? {
            if let Event::Key(key) = event::read()? {
                match key.code {
                    KeyCode::Char('d') | KeyCode::Char('D') => {
                        current_page = current_page.next();
                    }
                    KeyCode::Char('a') | KeyCode::Char('A') => {
                        current_page = current_page.prev();
                    }
                    KeyCode::Char('q') | KeyCode::Char('Q') => {
                        crossterm::terminal::disable_raw_mode()?;
                        stdout().execute(crossterm::terminal::LeaveAlternateScreen)?;
                        break;
                    }
                    _ => {}
                }
            }
        }

    }
    Ok(())
}


