## Architecture
![architecture](./photo/arch.png)
<br>
<br>
## illustrate
### HPL(Hardware Protool Layer) : 
#### Dynamically calculates the exact number of hardware threads for any host CPU in Initialization phase, This can make calculate the fixed size array CPU needs

### Collection Layer : 
#### Parses raw kernel telemetry (/proc/stat, /proc/cpuinfo, /proc/meminfo, statvfs, NVML) directly into byte slices and writes them into pre-allocated, cache-aligned memory slots with zero per-telemetry heap allocations.
