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

//! Network benchmarking functionality.

use crate::error::Error;
use std::time::{Duration, Instant};
use tokio::io::{AsyncReadExt, AsyncWriteExt};
use tokio::net::{TcpListener, TcpStream};

/// Network benchmark result.
#[derive(Debug, Clone)]
pub struct NetworkBenchmarkResult {
    /// Download speed in bytes per second.
    pub download_speed: f64,
    /// Upload speed in bytes per second.
    pub upload_speed: f64,
    /// Latency in milliseconds.
    pub latency: f64,
}

/// Runs a network benchmark and returns a result.
pub async fn run_network_benchmark(server_addr: &str) -> Result<NetworkBenchmarkResult, Error> {
    // Run latency benchmark
    let latency = run_latency_benchmark(server_addr).await?;
    
    // Run download benchmark
    let download_speed = run_download_benchmark(server_addr).await?;
    
    // Run upload benchmark
    let upload_speed = run_upload_benchmark(server_addr).await?;
    
    Ok(NetworkBenchmarkResult {
        download_speed,
        upload_speed,
        latency,
    })
}

/// Runs a latency benchmark.
pub async fn run_latency_benchmark(server_addr: &str) -> Result<f64, Error> {
    let num_pings = 10;
    let mut total_latency = Duration::from_secs(0);
    
    for _ in 0..num_pings {
        let start_time = Instant::now();
        
        // Connect to the server
        let mut stream = TcpStream::connect(server_addr).await
            .map_err(|e| Error::Benchmark(format!("Failed to connect to server: {}", e)))?;
        
        // Send a ping message
        stream.write_all(b"PING").await
            .map_err(|e| Error::Benchmark(format!("Failed to send ping: {}", e)))?;
        
        // Read the pong response
        let mut buffer = [0u8; 4];
        stream.read_exact(&mut buffer).await
            .map_err(|e| Error::Benchmark(format!("Failed to receive pong: {}", e)))?;
        
        if &buffer != b"PONG" {
            return Err(Error::Benchmark("Invalid pong response".to_string()));
        }
        
        // Calculate latency
        let latency = start_time.elapsed();
        total_latency += latency;
    }
    
    // Calculate average latency in milliseconds
    let avg_latency = total_latency.as_secs_f64() * 1000.0 / num_pings as f64;
    
    Ok(avg_latency)
}

/// Runs a download benchmark.
pub async fn run_download_benchmark(server_addr: &str) -> Result<f64, Error> {
    let download_size = 10 * 1024 * 1024; // 10 MB
    let buffer_size = 4096;
    let mut buffer = vec![0u8; buffer_size];
    
    // Connect to the server
    let mut stream = TcpStream::connect(server_addr).await
        .map_err(|e| Error::Benchmark(format!("Failed to connect to server: {}", e)))?;
    
    // Send a download request
    stream.write_all(b"DOWNLOAD").await
        .map_err(|e| Error::Benchmark(format!("Failed to send download request: {}", e)))?;
    
    // Read the response
    let start_time = Instant::now();
    let mut bytes_read = 0;
    
    while bytes_read < download_size {
        let read = stream.read(&mut buffer).await
            .map_err(|e| Error::Benchmark(format!("Failed to read data: {}", e)))?;
        
        if read == 0 {
            break; // End of stream
        }
        
        bytes_read += read;
    }
    
    let elapsed = start_time.elapsed();
    
    // Calculate download speed in bytes per second
    let speed = bytes_read as f64 / elapsed.as_secs_f64();
    
    Ok(speed)
}

/// Runs an upload benchmark.
pub async fn run_upload_benchmark(server_addr: &str) -> Result<f64, Error> {
    let upload_size = 10 * 1024 * 1024; // 10 MB
    let buffer_size = 4096;
    let buffer = vec![0u8; buffer_size];
    
    // Connect to the server
    let mut stream = TcpStream::connect(server_addr).await
        .map_err(|e| Error::Benchmark(format!("Failed to connect to server: {}", e)))?;
    
    // Send an upload request
    stream.write_all(b"UPLOAD").await
        .map_err(|e| Error::Benchmark(format!("Failed to send upload request: {}", e)))?;
    
    // Upload data
    let start_time = Instant::now();
    let mut bytes_written = 0;
    
    while bytes_written < upload_size {
        let to_write = std::cmp::min(buffer_size, upload_size - bytes_written);
        stream.write_all(&buffer[0..to_write]).await
            .map_err(|e| Error::Benchmark(format!("Failed to write data: {}", e)))?;
        bytes_written += to_write;
    }
    
    // Flush the stream
    stream.flush().await
        .map_err(|e| Error::Benchmark(format!("Failed to flush stream: {}", e)))?;
    
    let elapsed = start_time.elapsed();
    
    // Calculate upload speed in bytes per second
    let speed = bytes_written as f64 / elapsed.as_secs_f64();
    
    Ok(speed)
}

/// Starts a benchmark server.
pub async fn start_benchmark_server(addr: &str) -> Result<(), Error> {
    let listener = TcpListener::bind(addr).await
        .map_err(|e| Error::Benchmark(format!("Failed to bind to address: {}", e)))?;
    
    println!("Benchmark server listening on {}", addr);
    
    loop {
        let (mut socket, _) = listener.accept().await
            .map_err(|e| Error::Benchmark(format!("Failed to accept connection: {}", e)))?;
        
        // Handle the connection in a new task
        tokio::spawn(async move {
            let mut buffer = [0u8; 8];
            
            // Read the request
            if let Err(e) = socket.read_exact(&mut buffer[0..4]).await {
                eprintln!("Failed to read request: {}", e);
                return;
            }
            
            match &buffer[0..4] {
                b"PING" => {
                    // Respond with PONG
                    if let Err(e) = socket.write_all(b"PONG").await {
                        eprintln!("Failed to send pong: {}", e);
                    }
                },
                b"DOWN" => {
                    // Send download data
                    let data = vec![0u8; 4096];
                    let mut bytes_sent = 0;
                    let download_size = 10 * 1024 * 1024; // 10 MB
                    
                    while bytes_sent < download_size {
                        let to_send = std::cmp::min(data.len(), download_size - bytes_sent);
                        if let Err(e) = socket.write_all(&data[0..to_send]).await {
                            eprintln!("Failed to send download data: {}", e);
                            break;
                        }
                        bytes_sent += to_send;
                    }
                },
                b"UPLO" => {
                    // Receive upload data
                    let mut buffer = vec![0u8; 4096];
                    let mut bytes_received = 0;
                    
                    loop {
                        match socket.read(&mut buffer).await {
                            Ok(0) => break, // End of stream
                            Ok(n) => bytes_received += n,
                            Err(e) => {
                                eprintln!("Failed to receive upload data: {}", e);
                                break;
                            }
                        }
                    }
                    
                    println!("Received {} bytes of upload data", bytes_received);
                },
                _ => {
                    eprintln!("Unknown request");
                }
            }
        });
    }
}
