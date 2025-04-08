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
use std::fs::{self, File};
use std::io::{self, Read, Write, Seek, SeekFrom};
use std::path::Path;
use std::time::{Duration, Instant};

/// Runs a disk benchmark and returns a score.
pub fn run_disk_benchmark<P: AsRef<Path>>(path: P) -> Result<f64, Error> {
    // Create a temporary file for the benchmark
    let file_path = path.as_ref().join("catp2p_disk_benchmark.tmp");
    
    // Run the benchmark
    let write_score = run_disk_write_benchmark(&file_path, 100)?;
    let read_score = run_disk_read_benchmark(&file_path)?;
    let random_score = run_disk_random_benchmark(&file_path)?;
    
    // Clean up
    fs::remove_file(&file_path).map_err(|e| {
        Error::Benchmark(format!("Failed to remove temporary file: {}", e))
    })?;
    
    // Calculate the overall score
    let score = (write_score + read_score + random_score) / 3.0;
    
    Ok(score)
}

/// Runs a disk write benchmark.
fn run_disk_write_benchmark<P: AsRef<Path>>(path: P, size_mb: usize) -> Result<f64, Error> {
    let size = size_mb * 1024 * 1024;
    
    // Create a buffer with random data
    let mut buffer = Vec::with_capacity(size);
    for i in 0..size {
        buffer.push((i % 256) as u8);
    }
    
    let start_time = Instant::now();
    
    // Write to the file
    let mut file = File::create(path).map_err(|e| {
        Error::Benchmark(format!("Failed to create file: {}", e))
    })?;
    
    file.write_all(&buffer).map_err(|e| {
        Error::Benchmark(format!("Failed to write to file: {}", e))
    })?;
    
    file.sync_all().map_err(|e| {
        Error::Benchmark(format!("Failed to sync file: {}", e))
    })?;
    
    let elapsed = start_time.elapsed();
    
    // Calculate the score based on the elapsed time
    // Lower time is better, so we invert it
    let score = 1000.0 / elapsed.as_secs_f64();
    
    Ok(score)
}

/// Runs a disk read benchmark.
fn run_disk_read_benchmark<P: AsRef<Path>>(path: P) -> Result<f64, Error> {
    let start_time = Instant::now();
    
    // Read from the file
    let mut file = File::open(path).map_err(|e| {
        Error::Benchmark(format!("Failed to open file: {}", e))
    })?;
    
    let mut buffer = Vec::new();
    file.read_to_end(&mut buffer).map_err(|e| {
        Error::Benchmark(format!("Failed to read from file: {}", e))
    })?;
    
    let elapsed = start_time.elapsed();
    
    // Calculate the score based on the elapsed time
    // Lower time is better, so we invert it
    let score = 1000.0 / elapsed.as_secs_f64();
    
    Ok(score)
}

/// Runs a disk random access benchmark.
fn run_disk_random_benchmark<P: AsRef<Path>>(path: P) -> Result<f64, Error> {
    let start_time = Instant::now();
    
    // Open the file
    let mut file = File::open(path).map_err(|e| {
        Error::Benchmark(format!("Failed to open file: {}", e))
    })?;
    
    // Get the file size
    let file_size = file.metadata().map_err(|e| {
        Error::Benchmark(format!("Failed to get file metadata: {}", e))
    })?.len();
    
    // Perform random reads
    let mut buffer = [0u8; 4096];
    let num_reads = 1000;
    
    for i in 0..num_reads {
        let position = (i * 4096) % file_size;
        
        file.seek(SeekFrom::Start(position)).map_err(|e| {
            Error::Benchmark(format!("Failed to seek in file: {}", e))
        })?;
        
        file.read_exact(&mut buffer).map_err(|e| {
            Error::Benchmark(format!("Failed to read from file: {}", e))
        })?;
    }
    
    let elapsed = start_time.elapsed();
    
    // Calculate the score based on the elapsed time
    // Lower time is better, so we invert it
    let score = 1000.0 / elapsed.as_secs_f64();
    
    Ok(score)
}
