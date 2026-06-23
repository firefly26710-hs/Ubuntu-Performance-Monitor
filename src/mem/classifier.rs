use std::fs::File;
use std::io::BufReader;
use std::str::from_utf8;

pub fn read_mem(source:&mut DataSource) {
    let data_source = &mut source.public_array;
    if let Ok(meminfo_file) = File::open("/proc/meminfo") {
        let meminfo_reader = BufReader::new(meminfo_file);
        for (number, info) in meminfo_reader.lines().take(3).enumerate() {
            if let Ok(info) = info {
                let actual_length = info.len();
                let byte_char = info.as_bytes();
                let mut buffer_padding = [0u8; PADDING_SIZE];
                buffer_padding[0..actual_length].copy_from_slice(&byte_char[0..actual_length]);
                match number {
                    TOTAL_MEMORY_INFO
                    => self.public_array[0..PADDING_SIZE].copy_from_slice(&buffer_padding),

                    AVAILABLE_MEMORY_INFO
                    => self.public_array[PADDING_SIZE..2 * PADDING_SIZE].copy_from_slice(&buffer_padding),

                    _
                    => {}
                }

                match number {
                    0 => {
                        println!("-----------");
                        println!("{}\n", from_utf8(&self.public_array[0..PADDING_SIZE]).unwrap());
                        println!("-----------");
                    },
                    2 => {
                        println!("-----------");
                        println!("{}\n", from_utf8(&self.public_array[PADDING_SIZE..2 * PADDING_SIZE]).unwrap());
                        println!("-----------");
                    },
                    _ => {}
                }
            }
        }
    }
}