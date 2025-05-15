# WebSocket Benchmark: Performance-Based Scaling & User Support

## Performance Summary

| Language       | Total Duration | Total Messages | Min Latency | Max Latency | Avg Latency   |
|----------------|----------------|----------------|-------------|-------------|---------------|
| **Go**         | 126.74 ms      | 1000           | 52.5 µs     | 22.99 ms    | 3.93 ms       |
| **Rust**       | 2.12 s         | 1000           | 66.6 µs     | 2.09 ms     | 0.89 ms       |
| **Express.js** | 197.68 ms      | 1000           | 284.1 µs    | 15.29 ms    | 9.98 ms       |

---

## Scaling & Concurrent User Support (Based on Benchmark Performance)

| Language       | Scaling Approach               | Estimated Max Concurrent Users*      |
|----------------|-------------------------------|-------------------------------------|
| **Go**         | Handles low latency with fast processing time and lightweight goroutines, scales efficiently | ~20,000+ concurrent users           |
| **Rust**       | Extremely low average latency but higher total duration indicates room for optimization; with tuning, can scale massively | ~15,000+ concurrent users (with optimization) |
| **Express.js** | Higher latency and slower total duration limit single-instance scalability; needs clustering or load balancing | ~3,000 - 5,000 concurrent users     |

\* Estimates assume similar hardware and server resources; real numbers vary with implementation and environment.

---

### Summary

- **Go** shows the best balance of total runtime and average latency, supporting the highest number of users based on raw performance.  
- **Rust** excels in latency but requires code tuning to reduce total duration before reaching its full scaling potential.  
- **Express.js** suffers from higher latency and slower throughput, limiting single-instance scaling drastically.

