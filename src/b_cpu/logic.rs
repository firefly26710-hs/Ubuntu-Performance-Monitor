use crate::a_data_source::data::{DataSource, HALF_PADDING_SIZE, PADDING_SIZE};
use crate::b_cpu::collection::THREAD_START;

pub fn cpu_rating(source:&mut DataSource) {
    let data_array = &source.data_array;
    let prev_data_array = &mut source.prev_data_array;
    for (NUMBER, _) in data_array.chunks(PADDING_SIZE).enumerate(){
        let history = &mut source.history_array[NUMBER];

        let offest = NUMBER * PADDING_SIZE;
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
            history.copy_within(0..29, 1);
            history[0] = rating;
            println!("history[0]: {:.2}, history[1]: {:.2}, history[2]: {:.2}", history[0], history[1], history[2]);

        }

    }
}
