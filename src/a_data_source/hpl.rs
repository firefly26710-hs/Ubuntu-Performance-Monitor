use std::fs::File;
use std::io::{BufRead, BufReader};
use crate::a_data_source::data::DataSource;
use crate::b_cpu::collection::cpu_collection;
use crate::b_cpu::logic::cpu_logic;

fn thread_number() -> usize {
    let file = File::open("/proc/stat");
    let reader = BufReader::new(file.unwrap());
    let res = reader.lines().map(|l| l.unwrap()).
        filter(|is| is.contains("cpu")).count();

    res - 1
}

fn cpu_company(source: &DataSource) -> usize {
    let name_array = source.name_array;
    let name = std::str::from_utf8(&name_array).unwrap().trim_matches(char::from(0)).trim();
    let name = name.to_lowercase();

    if name.contains("intel"){
        1
    } else if name.contains("amd") {
        2
    } else{
        10
    }

}

#[test]
fn test_hpl() {
    let source = &mut DataSource::new();
    cpu_collection(source);
    cpu_logic(source);
    cpu_company(source);
    eprintln!("Is Thread Number is true? {}", thread_number() == 12);
    eprintln!("Is CPU Name is true? {}", cpu_company(source) == 2);


}
