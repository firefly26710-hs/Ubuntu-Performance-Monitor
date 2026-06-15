use std::fs::File;
use std::io::{BufRead, BufReader, Write};


fn main() {
    let opener = File::open("/proc/stat");
    let reader = BufReader::new(opener.unwrap());
    for(index, line) in reader.lines().enumerate(){
        println!("{}\n", line.unwrap());
    }
}
