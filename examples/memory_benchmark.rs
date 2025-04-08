/* Copyright 2025 Joao Guimaraes, Catp2p Project
 *
 * Licensed under the Apache License, Version 2.0 (the "License");
 * you may not use this file except in compliance with the License.
 * You may obtain a copy of the License at
 *
 *     http://www.apache.org/licenses/LICENSE-2.0
 *
 * Unless required by applicable law or agreed to in writing, software
 * distributed under the License is distributed on an "AS IS" BASIS,
 * WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
 * See the License for the specific language governing permissions and
 * limitations under the License.
 */

 use catp2p::benchmark::memory;
 use catp2p::benchmark::cpu;
 use catp2p::error::Error;
 use std::time::Instant;
 
 #[tokio::main]
 async fn main() -> Result<(), Error> {
     println!("=== CatP2P Memory Information and Benchmarking ===\n");
     
     // Part 1: Get memory information
     println!("--- Memory Information ---");
     
     // Get memory information using our new function
     let memory_info = memory::get_memory_info()?;
     
     println!("Total Memory: {:.2} GB", bytes_to_gb(memory_info.total_memory));
     println!("Available Memory: {:.2} GB", bytes_to_gb(memory_info.available_memory));
     println!("Used Memory: {:.2} GB", bytes_to_gb(memory_info.used_memory));
     println!("Memory Usage: {:.2}%", memory_info.usage_percent);
     
     // Get CPU information for context
     let cpu_info = cpu::get_cpu_info()?;
     println!("CPU: {} ({} cores)", cpu_info.name, cpu_info.logical_cores);
     println!("Memory per CPU core: {:.2} GB", bytes_to_gb(memory_info.memory_per_core));
     
     // Part 2: Run overall memory benchmark
     println!("\n--- Memory Performance Benchmark ---");
     println!("Running memory benchmark...");
     
     let start_time = Instant::now();
     let memory_score = memory::run_memory_benchmark()?;
     let elapsed = start_time.elapsed();
     
     println!("Memory Benchmark Score: {:.2}", memory_score);
     println!("Benchmark completed in {:.2} seconds", elapsed.as_secs_f64());
     
     // Part 3: Run specific memory benchmarks
     println!("\n--- Memory Allocation Performance ---");
     let allocation_start = Instant::now();
     let allocation_score = memory::run_allocation_benchmark()?;
     let allocation_time = allocation_start.elapsed();
     println!("Allocation Benchmark Score: {:.2}", allocation_score);
     println!("Completed in {:.2} seconds", allocation_time.as_secs_f64());
     
     println!("\n--- Memory Read/Write Performance ---");
     let rw_start = Instant::now();
     let rw_score = memory::run_read_write_benchmark()?;
     let rw_time = rw_start.elapsed();
     println!("Read/Write Benchmark Score: {:.2}", rw_score);
     println!("Completed in {:.2} seconds", rw_time.as_secs_f64());
     
     println!("\n--- Memory Random Access Performance ---");
     let random_start = Instant::now();
     let random_score = memory::run_random_access_benchmark()?;
     let random_time = random_start.elapsed();
     println!("Random Access Benchmark Score: {:.2}", random_score);
     println!("Completed in {:.2} seconds", random_time.as_secs_f64());
     
     // Part 4: Visualize benchmark results
     println!("\n--- Memory Benchmark Visualization ---");
     println!("Higher is better:");
     
     let max_score = [allocation_score, rw_score, random_score, memory_score]
         .iter()
         .fold(0.0f64, |a: f64, &b| a.max(b));
     
     let scale = 50.0 / max_score;
     
     visualize_score("Allocation", allocation_score, scale);
     visualize_score("Read/Write", rw_score, scale);
     visualize_score("Random Access", random_score, scale);
     visualize_score("Overall", memory_score, scale);
     
     // Part 5: Memory performance analysis
     println!("\n--- Memory Performance Analysis ---");
     
     // Analyze allocation performance
     if allocation_score > 800.0 {
         println!("✅ Memory allocation performance is excellent");
     } else if allocation_score > 500.0 {
         println!("✓ Memory allocation performance is good");
     } else {
         println!("⚠ Memory allocation performance is below average");
     }
     
     // Analyze read/write performance
     if rw_score > 800.0 {
         println!("✅ Memory read/write performance is excellent");
     } else if rw_score > 500.0 {
         println!("✓ Memory read/write performance is good");
     } else {
         println!("⚠ Memory read/write performance is below average");
     }
     
     // Analyze random access performance
     if random_score > 800.0 {
         println!("✅ Memory random access performance is excellent");
     } else if random_score > 500.0 {
         println!("✓ Memory random access performance is good");
     } else {
         println!("⚠ Memory random access performance is below average");
     }
     
     // Overall memory performance
     if memory_score > 800.0 {
         println!("✅ Overall memory performance is excellent");
     } else if memory_score > 500.0 {
         println!("✓ Overall memory performance is good");
     } else {
         println!("⚠ Overall memory performance is below average");
     }
     
     // Part 6: Memory recommendations
     println!("\n--- Memory Recommendations ---");
     
     if memory_info.memory_per_core < 1_073_741_824 { // Less than 1 GB per core
         println!("⚠ Limited memory per CPU core. Consider reducing parallel workloads.");
     } else if memory_info.memory_per_core > 4_294_967_296 { // More than 4 GB per core
         println!("✅ Excellent memory-to-CPU ratio. Suitable for memory-intensive tasks.");
     } else {
         println!("✓ Good memory-to-CPU ratio. Suitable for most workloads.");
     }
     
     if memory_info.usage_percent > 80.0 {
         println!("⚠ High memory usage detected. Some benchmarks may be affected.");
     }
     
     println!("\nMemory benchmarking completed!");
     
     Ok(())
 }
 
 // Helper function to convert bytes to gigabytes
 fn bytes_to_gb(bytes: u64) -> f64 {
     bytes as f64 / 1_073_741_824.0 // 1024^3
 }
 
 // Helper function to visualize a score with a bar chart
 fn visualize_score(name: &str, score: f64, scale: f64) {
     let bar_length = (score * scale) as usize;
     let bar = "#".repeat(bar_length);
     println!("{:14}: {:7.2} |{}|", name, score, bar);
 }
 