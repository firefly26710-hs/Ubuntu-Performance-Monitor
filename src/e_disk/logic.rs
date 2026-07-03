use nix::libc::statvfs;
use crate::a_data_source::data::{DataSource, HALF_PADDING_SIZE};
use crate::e_disk::collection::{disk_collection, DISK_AVAIL_END, DISK_AVAIL_START, DISK_TOTAL_END, DISK_TOTAL_START};

pub fn disk_logic(source:&mut DataSource){
    let data_source = &mut source.data_array;
    let gauge_array = &mut source.gauge_array;

    let total_slice: &[u8; HALF_PADDING_SIZE] = (&data_source[DISK_TOTAL_START..DISK_TOTAL_END]).try_into().unwrap();
    let avail_slice: &[u8; HALF_PADDING_SIZE] = (&data_source[DISK_AVAIL_START..DISK_AVAIL_END]).try_into().unwrap();

    let total_to_u64 = u64::from_be_bytes(*total_slice);
    let avail_to_u64 = u64::from_be_bytes(*avail_slice);

    let total_disk = total_to_u64 as f64;
    let avail_disk = avail_to_u64 as f64;
    let used_disk = (total_disk - avail_disk).max(0.0);
    
    let total_disk = total_disk / 1024.0 / 1024.0 / 1024.0;
    let avail_disk = avail_disk / 1024.0 / 1024.0 / 1024.0;
    let used_disk = used_disk / 1024.0 / 1024.0 / 1024.0;

    gauge_array[0] = total_disk;
    gauge_array[1] = avail_disk;
    gauge_array[2] = used_disk;


}

#[test]
fn test_disk_logic(){
    let mut source = DataSource::new();
    let mut read: statvfs = unsafe { std::mem::zeroed() };
    unsafe { statvfs("/\0".as_ptr() as *const i8, &mut read); }


    disk_collection(&read,&mut source);
    disk_logic(&mut source);
    let gauge_array = &mut source.gauge_array;
    let check_total_disk = gauge_array[0];
    let check_avail_disk = gauge_array[1];
    let check_used_disk = gauge_array[2];
    eprintln!(" Disk Total : {:.2} , Disk Avail : {:.2}, Disk Used : {:.2} "
              , check_total_disk, check_avail_disk, check_used_disk);
}