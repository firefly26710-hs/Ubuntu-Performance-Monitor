use std::fs::File;
use std::io::{BufRead, BufReader};


fn thread_number() -> usize {
    let file = File::open("/proc/stat");
    let reader = BufReader::new(file.unwrap());
    let res = reader.lines().map(|l| l.unwrap()).
        filter(|is_contain| is_contain.contains("cpu")).count();

    res - 1
}

#[test]
fn test_thread_number() {
    eprintln!("Is Thread Number is true? {}", thread_number() == 12);
}
