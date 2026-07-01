use crate::a_data_source::data::{DataSource, HALF_PADDING_SIZE, PADDING_SIZE};
use crate::b_cpu::collection::THREAD_START;

pub fn cpu_rating(source:&mut DataSource) {
    let data_array = &source.data_array;

    for (NUMBER, _) in data_array.chunks(PADDING_SIZE).enumerate(){
        let history = &mut source.history_array[NUMBER];

        let offest = NUMBER * PADDING_SIZE;
        let start = THREAD_START + offest;
        let end = start + PADDING_SIZE;
        let mid = (start + end) / 2;


        let total_slice:&[u8; HALF_PADDING_SIZE] = (&data_array[start..start + HALF_PADDING_SIZE]).try_into().unwrap();
        let idle_slice:&[u8; HALF_PADDING_SIZE] = (&data_array[mid..mid + HALF_PADDING_SIZE]).try_into().unwrap();

        let total_to_u64 = u64::from_be_bytes(*total_slice);
        let idle_to_u64 = u64::from_be_bytes(*idle_slice);

        let total_use = total_to_u64 as f64;
        let idle_use = idle_to_u64 as f64;

        if total_use > 0.0{
            let rating = (1.0 - (idle_use/total_use))*100.0;

            history.copy_within(0..29, 1);
            history[0] = rating;
            println!("history[0]: {:.2}, history[1]: {:.2}, history[2]: {:.2}", history[0], history[1], history[2]);
        }

    }
}
