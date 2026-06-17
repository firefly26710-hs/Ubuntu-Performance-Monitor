use std::fs::File;
use std::io::{BufRead, BufReader};
use nix::libc::__u8;

pub mod cpu;
pub mod mem;
pub mod gpu;

pub mod disk;
pub mod data_source;


#[derive(Debug, Copy, Clone)]
pub struct DiskMetrics {
    pub total_gb: f32,
    pub used_gb: f32,
    pub usage_percent: f32,
}

// 2. 核心提取函數
fn get_disk_usage() -> Result<DiskMetrics, nix::Error> {
    // 💥 這裡改用絕對路徑呼叫，直接破除 scope 詛咒！
    let stats = nix::sys::statvfs::statvfs("/")?;

    // 取得物理數值（注意：有些 nix 版本函數名是 f_frsize() 或 fragment_size()）
    // 這裡我們直接用 nix 標準的 method 呼叫：
    let block_size = stats.fragment_size() as u64;
    let total_blocks = stats.blocks();
    let avail_blocks = stats.blocks_available();

    let total_bytes = total_blocks * block_size;
    let avail_bytes = avail_blocks * block_size;
    let used_bytes = total_bytes - avail_bytes;

    let bytes_to_gb = 1024.0 * 1024.0 * 1024.0;
    let total_gb = total_bytes as f32 / bytes_to_gb;
    let used_gb = used_bytes as f32 / bytes_to_gb;
    let usage_percent = (used_bytes as f32 / total_bytes as f32) * 100.0;

    Ok(DiskMetrics {
        total_gb,
        used_gb,
        usage_percent,
    })
}


fn main() {
    println!("🚀 [Stage 1] 實體硬碟資訊強制提取中...");
    match get_disk_usage() {
        Ok(metrics) => {
            println!("硬碟總容量: {:.2} GB", metrics.total_gb);
            println!("已使用容量: {:.2} GB", metrics.used_gb);
            println!("即時使用率: {:.2} %", metrics.usage_percent);
        }
        Err(e) => eprintln!("幹，系統呼叫失敗: {:?}", e),
    }
    #[allow(E0061)]println!("{}", size_of::<u16>());
}
