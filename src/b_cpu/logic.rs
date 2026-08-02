use crate::a_data_source::data::{DataSource, HALF_PADDING_SIZE, PADDING_SIZE};
use crate::a_data_source::hpl::thread_number;
use crate::b_cpu::collection::{cpu_collection, THREAD_START};

pub fn cpu_logic(source:&mut DataSource, thread_number:usize) {
    let data_array = &source.data_array;
    let prev_data_array = &mut source.prev_data_array;
    for (number, _) in data_array.chunks(PADDING_SIZE).take(thread_number).enumerate(){
        let history = &mut source.chart_array;

        let offest = number * PADDING_SIZE;
        let start = THREAD_START + offest;
        let end = start + PADDING_SIZE;
        let mid = (start + end) / 2;

        let curr_total_slice:[u8; HALF_PADDING_SIZE] = (&data_array[start..mid]).try_into().unwrap();
        let curr_idle_slice:[u8; HALF_PADDING_SIZE] = (&data_array[mid..end]).try_into().unwrap();
        let prev_total_slice:[u8;HALF_PADDING_SIZE] = (&prev_data_array[start..mid]).try_into().unwrap();
        let prev_idle_slice:[u8; HALF_PADDING_SIZE] = (&prev_data_array[mid..end]).try_into().unwrap();

        prev_data_array[start..mid].copy_from_slice(&curr_total_slice);
        prev_data_array[mid..end].copy_from_slice(&curr_idle_slice);


        let curr_total_to_u64 = u64::from_be_bytes(curr_total_slice);
        let curr_idle_to_u64 = u64::from_be_bytes(curr_idle_slice);
        let prev_total_to_u64 = u64::from_be_bytes(prev_total_slice);
        let prev_idle_to_u64 = u64::from_be_bytes(prev_idle_slice);


        let curr_total = curr_total_to_u64 as f64;
        let curr_idle = curr_idle_to_u64 as f64;
        let prev_total = prev_total_to_u64 as f64;
        let prev_idle = prev_idle_to_u64 as f64;

        let total = curr_total - prev_total;
        let idle = curr_idle - prev_idle;

        if total > 0.0{

            let rating = (1.0 - (idle / total)) * 100.0;
            history[number] = rating;
            //println!("history[0]: {:.2}, history[1]: {:.2}, history[2]: {:.2}", history[0], history[1], history[2]);

        }

    }
}

#[test]
fn test_cpu_logic(){
    let mut source = DataSource::new();
    let thread_number = thread_number();
    cpu_collection(&mut source, thread_number);
    cpu_logic(&mut source, thread_number);
    let chart_array = source.chart_array;
    for (number, data) in chart_array.iter().take(thread_number).enumerate(){
        eprintln!("THREAD : {}, USAGE : {:.2}", number, data);
    }
}


