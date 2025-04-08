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

//! Disk benchmarking functionality.

use crate::error::Error;
use std::fs::{File, OpenOptions};
use std::io::{Read, Write, Seek, SeekFrom}; // Removed unused self import
use std::path::Path;
use std::time::Instant; // Removed unused Duration import

/// Runs a disk benchmark and returns a score.
pub fn run_disk_benchmark() -> Result<f64, Error> {
    // Create a temporary file for the benchmark
    let temp_dir = std::env::temp_dir();
    let file_path = temp_dir.join("catp2p_disk_benchmark.tmp");
    
    // Run the benchmark
    let start_time = Instant::now();
    
    // Write benchmark
    let write_speed = run_write_benchmark(&file_path, 100)?; // 100 MB
    
    // Read benchmark
    let read_speed = run_read_benchmark(&file_path)?;
    
    // Random access benchmark
    let random_access_speed = run_random_access_benchmark(&file_path)?;
    
    // Clean up
    std::fs::remove_file(&file_path)
        .map_err(|e| Error::Benchmark(format!("Failed to remove temporary file: {}", e)))?;
    
    // Prefix with underscore to indicate intentional non-use
    let _elapsed = start_time.elapsed();
    
    // Calculate the score based on the speeds
    // Higher speeds are better
    let score = (write_speed + read_speed + random_access_speed) / 3.0;
    
    Ok(score)
}

/// Runs a write benchmark.
pub fn run_write_benchmark(file_path: &Path, size_mb: usize) -> Result<f64, Error> {
    let size = size_mb * 1024 * 1024;
    let buffer_size = 4096;
    let buffer = vec![0u8; buffer_size];
    
    let start_time = Instant::now();
    
    // Create a file for writing
    let mut file = OpenOptions::new()
        .write(true)
        .create(true)
        .truncate(true)
        .open(file_path)
        .map_err(|e| Error::Benchmark(format!("Failed to create file: {}", e)))?;
    
    // Write data to the file
    let mut bytes_written = 0;
    while bytes_written < size {
        let to_write = std::cmp::min(buffer_size, size - bytes_written);
        file.write_all(&buffer[0..to_write])
            .map_err(|e| Error::Benchmark(format!("Failed to write to file: {}", e)))?;
        bytes_written += to_write;
    }
    
    // Flush the file
    file.flush()
        .map_err(|e| Error::Benchmark(format!("Failed to flush file: {}", e)))?;
    
    let elapsed = start_time.elapsed();
    
    // Calculate write speed in MB/s
    let speed = (size as f64) / elapsed.as_secs_f64() / (1024.0 * 1024.0);
    
    Ok(speed)
}

/// Runs a read benchmark.
pub fn run_read_benchmark(file_path: &Path) -> Result<f64, Error> {
    let buffer_size = 4096;
    let mut buffer = vec![0u8; buffer_size];
    
    // Open the file for reading
    let mut file = File::open(file_path)
        .map_err(|e| Error::Benchmark(format!("Failed to open file: {}", e)))?;
    
    // Get the file size
    let file_size = file.metadata()
        .map_err(|e| Error::Benchmark(format!("Failed to get file metadata: {}", e)))?
        .len() as usize;
    
    let start_time = Instant::now();
    
    // Read data from the file
    let mut bytes_read = 0;
    while bytes_read < file_size {
        let to_read = std::cmp::min(buffer_size, file_size - bytes_read);
        let read = file.read(&mut buffer[0..to_read])
            .map_err(|e| Error::Benchmark(format!("Failed to read from file: {}", e)))?;
        
        if read == 0 {
            break; // End of file
        }
        
        bytes_read += read;
    }
    
    let elapsed = start_time.elapsed();
    
    // Calculate read speed in MB/s
    let speed = (file_size as f64) / elapsed.as_secs_f64() / (1024.0 * 1024.0);
    
    Ok(speed)
}

/// Runs a random access benchmark.
pub fn run_random_access_benchmark(file_path: &Path) -> Result<f64, Error> {
    let buffer_size = 4096;
    let mut buffer = vec![0u8; buffer_size];
    
    // Open the file for reading
    let mut file = File::open(file_path)
        .map_err(|e| Error::Benchmark(format!("Failed to open file: {}", e)))?;
    
    // Get the file size
    let file_size = file.metadata()
        .map_err(|e| Error::Benchmark(format!("Failed to get file metadata: {}", e)))?
        .len() as usize;
    
    // Generate random positions
    let num_accesses = 1000;
    let mut positions = Vec::with_capacity(num_accesses);
    for _ in 0..num_accesses {
        positions.push(rand::random::<u64>() % (file_size as u64));
    }
    
    let start_time = Instant::now();
    
    // Perform random accesses
    for &pos in &positions {
        file.seek(SeekFrom::Start(pos))
            .map_err(|e| Error::Benchmark(format!("Failed to seek in file: {}", e)))?;
        
        let to_read = std::cmp::min(buffer_size, file_size - pos as usize);
        if to_read == 0 {
            continue;
        }
        
        file.read(&mut buffer[0..to_read])
            .map_err(|e| Error::Benchmark(format!("Failed to read from file: {}", e)))?;
    }
    
    let elapsed = start_time.elapsed();
    
    // Calculate random access speed in operations per second
    let speed = (num_accesses as f64) / elapsed.as_secs_f64();
    
    Ok(speed)
}
