use criterion::{criterion_group, criterion_main, Criterion};
use std::hint::black_box;
use nix::libc::statvfs;
use Preformce_tool::a_data_source::data::{DataSource};
use Preformce_tool::a_data_source::hpl::thread_number;
use Preformce_tool::b_cpu::{collection::cpu_collection, logic::cpu_logic};
use Preformce_tool::c_mem::{collection::mem_collection, logic::mem_logic};
use Preformce_tool::d_gpu::{collection::gpu_collection, logic::gpu_logic};
use Preformce_tool::e_disk::{collection::disk_collection, logic::disk_logic};

fn bench_individual_telemetry(c: &mut Criterion) {
    let mut source = DataSource::new();
    let thread_number = thread_number() ;
    let nvml = nvml_wrapper::Nvml::init().ok();
    let mut read: statvfs = unsafe { std::mem::zeroed() };
    unsafe { statvfs("/\0".as_ptr() as *const i8, &mut read); }

    // 1. CPU 模組測試 (/proc/stat & /proc/cpuinfo)
    c.bench_function("bench_cpu_module", |b| {
        b.iter(|| {
            cpu_collection(black_box(&mut source), black_box(thread_number));
            cpu_logic(black_box(&mut source), black_box(thread_number));
        });
    });

    // 2. Memory 模組測試 (/proc/meminfo Parsing)
    c.bench_function("bench_mem_module", |b| {
        b.iter(|| {
            mem_collection(black_box(&mut source));
            mem_logic(black_box(&mut source));
        });
    });

    // 3. Disk 模組測試 (statvfs Syscall)
    c.bench_function("bench_disk_module", |b| {
        b.iter(|| {
            // 把 statvfs 放進來，每次 iteration 都向系統重新查詢一次
            unsafe {
                statvfs("/\0".as_ptr() as *const i8, &mut read);
            }
            let _ = disk_collection(black_box(&mut read), black_box(&mut source));
            disk_logic(black_box(&mut source));
        });
    });

    // 4. GPU 模組測試 (NVML C-FFI Driver Query)
    if let Some(ref nvml_ctx) = nvml {
        c.bench_function("bench_gpu_module", |b| {
            b.iter(|| {
                let _ = gpu_collection(black_box(nvml_ctx), black_box(&mut source));
                gpu_logic( black_box(&mut source));
            });
        });
    }
}

criterion_group!(benches, bench_individual_telemetry);
criterion_main!(benches);