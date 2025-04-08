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
use tokio::net::{TcpListener, TcpStream};
use tokio::io::{AsyncReadExt, AsyncWriteExt};
use tokio::time;

/// Runs a network benchmark and returns a score.
pub async fn run_network_benchmark() -> Result<f64, Error> {
    // Run the benchmark
    let latency_score = run_network_latency_benchmark().await?;
    let throughput_score = run_network_throughput_benchmark().await?;
    
    // Calculate the overall score
    let score = (latency_score + throughput_score) / 2.0;
    
    Ok(score)
}

/// Runs a network latency benchmark.
async fn run_network_latency_benchmark() -> Result<f64, Error> {
    // Start a local echo server
    let server = tokio::spawn(async {
        let listener = TcpListener::bind("127.0.0.1:0").await.unwrap();
        let addr = listener.local_addr().unwrap();
        
        // Accept a connection
        let (mut socket, _) = listener.accept().await.unwrap();
        
        // Echo data back to the client
        let mut buf = [0u8; 1024];
        loop {
            match socket.read(&mut buf).await {
                Ok(0) => break, // Connection closed
                Ok(n) => {
                    if socket.write_all(&buf[0..n]).await.is_err() {
                        break;
                    }
                }
                Err(_) => break,
            }
        }
        
        addr
    });
    
    // Wait for the server to start
    time::sleep(Duration::from_millis(100)).await;
    
    // Get the server address
    let server_addr = server.await.map_err(|e| {
        Error::Benchmark(format!("Server task failed: {}", e))
    })?;
    
    // Connect to the server
    let mut client = TcpStream::connect(server_addr).await.map_err(|e| {
        Error::Benchmark(format!("Failed to connect to server: {}", e))
    })?;
    
    // Measure round-trip time
    let start_time = Instant::now();
    let num_pings = 100;
    
    for _ in 0..num_pings {
        // Send a ping
        client.write_all(b"ping").await.map_err(|e| {
            Error::Benchmark(format!("Failed to send ping: {}", e))
        })?;
        
        // Receive a pong
        let mut buf = [0u8; 4];
        client.read_exact(&mut buf).await.map_err(|e| {
            Error::Benchmark(format!("Failed to receive pong: {}", e))
        })?;
    }
    
    let elapsed = start_time.elapsed();
    
    // Calculate the average round-trip time
    let avg_rtt = elapsed.as_secs_f64() / (num_pings as f64);
    
    // Calculate the score based on the average round-trip time
    // Lower time is better, so we invert it
    let score = 1.0 / avg_rtt;
    
    Ok(score)
}

/// Runs a network throughput benchmark.
async fn run_network_throughput_benchmark() -> Result<f64, Error> {
    // Start a local server
    let server = tokio::spawn(async {
        let listener = TcpListener::bind("127.0.0.1:0").await.unwrap();
        let addr = listener.local_addr().unwrap();
        
        // Accept a connection
        let (mut socket, _) = listener.accept().await.unwrap();
        
        // Receive data from the client
        let mut total_bytes = 0;
        let mut buf = [0u8; 65536];
        loop {
            match socket.read(&mut buf).await {
                Ok(0) => break, // Connection closed
                Ok(n) => {
                    total_bytes += n;
                }
                Err(_) => break,
            }
        }
        
        (addr, total_bytes)
    });
    
    // Wait for the server to start
    time::sleep(Duration::from_millis(100)).await;
    
    // Get the server address
    let server_result = server.await.map_err(|e| {
        Error::Benchmark(format!("Server task failed: {}", e))
    })?;
    
    let (server_addr, _) = server_result;
    
    // Connect to the server
    let mut client = TcpStream::connect(server_addr).await.map_err(|e| {
        Error::Benchmark(format!("Failed to connect to server: {}", e))
    })?;
    
    // Measure throughput
    let start_time = Instant::now();
    let data_size = 100 * 1024 * 1024; // 100 MB
    let chunk_size = 65536; // 64 KB
    let data = vec![0u8; chunk_size];
    
    let mut total_sent = 0;
    while total_sent < data_size {
        let to_send = std::cmp::min(chunk_size, data_size - total_sent);
        client.write_all(&data[0..to_send]).await.map_err(|e| {
            Error::Benchmark(format!("Failed to send data: {}", e))
        })?;
        total_sent += to_send;
    }
    
    // Close the connection
    drop(client);
    
    let elapsed = start_time.elapsed();
    
    // Calculate the throughput in MB/s
    let throughput = (data_size as f64) / elapsed.as_secs_f64() / (1024.0 * 1024.0);
    
    // Calculate the score based on the throughput
    // Higher throughput is better
    let score = throughput;
    
    Ok(score)
}
