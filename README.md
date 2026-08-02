## Architecture
![architecture](./photo/arch.png)
<br>
<br>
<br>
## illustrate
### HPL(Hardware Protool Layer) : 
#### Dynamically calculates the exact number of hardware threads for any host CPU in Initialization phase, This can make calculate the fixed size array CPU needs

### Collection Layer : 
#### Parses raw kernel telemetry (/proc/stat, /proc/cpuinfo, /proc/meminfo, statvfs, NVML) directly into byte slices and writes them into pre-allocated, cache-aligned memory slots with zero per-telemetry heap allocations

### Data Source : 
#### Designed a polymorphic shared memory architecture using a fixed-size byte array across all metric modules, maximizing memory reuse and minimizing footprint and explicit 16-byte memory alignment for CPU thread slots to ensure peak cache efficiency

### Logic Layer :
#### Processes hardware state transitions and computes real-time metric deltas (e.g., CPU usage rates via byte-slice conversions) directly within contiguous buffers, pumping calculated metrics straight to the chart arrays for the UI presentation layer

### Present Layer :
#### Built an event-driven, single-threaded loop (`event::poll` with 1s timeout) that fetches target hardware metrics on-demand based on UI interaction (`A`/`D` navigation, `Q` exit). Completely eliminates thread context-switching overhead, mutex contention, and data races while controlling sampling frequency
<br>
<br>
<br>
## Telemetry Ingestion & Processing Pipeline

