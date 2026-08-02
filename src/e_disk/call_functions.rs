use nix::libc::statvfs;
use ratatui::Frame;
use crate::a_data_source::data::DataSource;
use crate::e_disk::collection::disk_collection;
use crate::e_disk::logic::disk_logic;
use crate::e_disk::present::disk_present;

pub fn disk_call(f: &mut Frame,read:&statvfs, source: &mut DataSource){
    disk_collection(read,source);
    disk_logic(source);
    disk_present(f, source);

}
