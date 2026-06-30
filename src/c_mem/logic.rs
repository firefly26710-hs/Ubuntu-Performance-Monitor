use crate::a_data_source::data::{DataSource, MAX_PUBLIC_ARRAY_SIZE, PADDING_SIZE};
use crate::c_mem::collection::{MEMORY_AVAIL_START, MEMORY_TOTAL_START};


const OLD_MEMORY_TOTAL_START:usize = PADDING_SIZE*2;
const OLD_MEMORY_AVAIL_START:usize = PADDING_SIZE*3;
const U64_SIZE: usize = 8;
fn cal(source:&mut DataSource){
    let data_source:&mut[u8; MAX_PUBLIC_ARRAY_SIZE] = &mut source.public_array;

    //step 1
    let n_total_slice: &[u8; U64_SIZE] = (&data_source[MEMORY_TOTAL_START..MEMORY_TOTAL_START + U64_SIZE]).try_into().unwrap();
    let o_total_slice: &[u8; U64_SIZE] = (&data_source[OLD_MEMORY_TOTAL_START..OLD_MEMORY_TOTAL_START + U64_SIZE]).try_into().unwrap();
    let n_avail_slice: &[u8; U64_SIZE] = (&data_source[MEMORY_AVAIL_START..MEMORY_AVAIL_START + U64_SIZE]).try_into().unwrap();
    let o_avail_slice: &[u8; U64_SIZE] = (&data_source[OLD_MEMORY_AVAIL_START..OLD_MEMORY_AVAIL_START + U64_SIZE]).try_into().unwrap();

    // step 2: 傳入 from_be_bytes，用 * 解開引用，CPU 直接在暫存器秒殺轉出 u64
    let n_total = u64::from_be_bytes(*n_total_slice);
    let o_total = u64::from_be_bytes(*o_total_slice);
    let n_avail = u64::from_be_bytes(*n_avail_slice);
    let o_avail = u64::from_be_bytes(*o_avail_slice);

    let total = n_total - o_total;
    let avail = n_avail - o_avail;

    let rating = ((total - avail)/total)*100;
    println!("rating : {}", rating);
}