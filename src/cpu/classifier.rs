use crate::data_source::data::{PADDING_SIZE};

const THREAD_NUMBER:usize = 12;

const NAME_INFO_START:usize = 0;
const NAME_INFO_END: usize = PADDING_SIZE - NAME_INFO_START;
const THREAD_SIZE:usize = THREAD_NUMBER * PADDING_SIZE;

pub mod information_cpu{
    use std::str::from_utf8;
    use crate::cpu::classifier::{NAME_INFO_END, NAME_INFO_START};
    use crate::data_source::data::{DataSource, PADDING_SIZE};

    pub fn catch_cpu_name(source:&mut DataSource){
        let data_source = &mut source.public_array[NAME_INFO_START..NAME_INFO_END];
        let mut temp_array = [0; NAME_INFO_END - NAME_INFO_START];
        if let Some(start) = data_source.iter().position(|&c| c == b'A'){
            let actual_len = NAME_INFO_END - start;
            temp_array[0..actual_len].copy_from_slice(&data_source[start..]);
            data_source.fill(0);
            data_source[NAME_INFO_START..NAME_INFO_END].copy_from_slice(&temp_array);
            println!("{}", from_utf8(&data_source).unwrap());
        }
    }
}




