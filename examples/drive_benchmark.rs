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

//! Example demonstrating drive benchmarking functionality.

use catp2p::benchmark::drives::{
    DriveBenchmarkConfig, run_drive_benchmark, run_drive_benchmark_with_config,
    run_all_drives_benchmark_with_config, get_drives_info
};
use catp2p::error::Error;


fn main() -> Result<(), Error> {
    println!("=== CatP2P Drive Benchmarking Example ===\n");

    // 1. List all available drives
    println!("Available Drives:");
    println!("------------------");
    for (i, drive_info) in get_drives_info().iter().enumerate() {
        println!("Drive {}: {} ({})", i + 1, drive_info.name, drive_info.path.display());
        println!("  File System: {}", drive_info.file_system);
        println!("  Total Capacity: {:.2} GB", bytes_to_gb(drive_info.total_capacity));
        println!("  Available Space: {:.2} GB ({:.1}%)", 
            bytes_to_gb(drive_info.available_space),
            (drive_info.available_space as f64 / drive_info.total_capacity as f64) * 100.0
        );
        println!("  Removable: {}", if drive_info.is_removable { "Yes" } else { "No" });
        println!();
    }

    // 2. Run a simple benchmark on the default drive (temp directory)
    println!("Running simple benchmark on default drive...");
    let simple_score = run_drive_benchmark()?;
    println!("Simple benchmark score: {:.2}\n", simple_score);

    // 3. Run a custom benchmark on a specific drive
    if let Some(first_drive) = get_drives_info().first() {
        println!("Running custom benchmark on drive: {}", first_drive.path.display());
        
        // Create a custom configuration
        let config = DriveBenchmarkConfig {
            file_size_mb: 50,           // Smaller file size for quicker test
            random_access_ops: 500,      // Fewer random access operations
            include_random_access: true, // Include random access test
        };
        
        // Run the benchmark with custom configuration
        match run_drive_benchmark_with_config(&first_drive.path, &config) {
            Ok(result) => {
                println!("Custom benchmark results:");
                println!("  Write Speed: {:.2} MB/s", result.write_speed);
                println!("  Read Speed: {:.2} MB/s", result.read_speed);
                println!("  Random Access: {:.2} ops/s", result.random_access_speed);
                println!("  Overall Score: {:.2}", result.overall_score);
                println!();
            },
            Err(e) => {
                println!("Error running custom benchmark: {}", e);
                println!("This might happen if the drive doesn't have write permissions.");
                println!();
            }
        }
    }

    // 4. Run benchmarks on all available drives
    println!("Running benchmarks on all available drives...");
    println!("(This may take some time and require permissions)");
    
    // Create a configuration for all drives benchmark
    let all_drives_config = DriveBenchmarkConfig {
        file_size_mb: 25,            // Smaller file for quicker tests
        random_access_ops: 250,       // Fewer operations
        include_random_access: true,  // Include random access test
    };
    
    match run_all_drives_benchmark_with_config(&all_drives_config) {
        Ok(results) => {
            if results.is_empty() {
                println!("No drives were successfully benchmarked.");
                println!("This might be due to permission issues or insufficient space.");
            } else {
                println!("\nResults for all drives:");
                println!("----------------------");
                
                for (i, result) in results.iter().enumerate() {
                    println!("Drive {}: {}", i + 1, result.path.display());
                    println!("  Write Speed: {:.2} MB/s", result.write_speed);
                    println!("  Read Speed: {:.2} MB/s", result.read_speed);
                    println!("  Random Access: {:.2} ops/s", result.random_access_speed);
                    println!("  Overall Score: {:.2}", result.overall_score);
                    println!("  Available Space: {:.2} GB / {:.2} GB", 
                        bytes_to_gb(result.available_space),
                        bytes_to_gb(result.total_capacity)
                    );
                    println!();
                }
                
                // Find the drive with the highest score
                if let Some(best_drive) = results.iter().max_by(|a, b| 
                    a.overall_score.partial_cmp(&b.overall_score).unwrap_or(std::cmp::Ordering::Equal)
                ) {
                    println!("Best performing drive: {}", best_drive.path.display());
                    println!("  Overall Score: {:.2}", best_drive.overall_score);
                    println!("  Write Speed: {:.2} MB/s", best_drive.write_speed);
                    println!("  Read Speed: {:.2} MB/s", best_drive.read_speed);
                    println!();
                }
            }
        },
        Err(e) => {
            println!("Error running benchmarks on all drives: {}", e);
        }
    }

    // 5. Demonstrate a specific use case: finding the best drive for storage
    println!("Finding the best drive for storage...");
    
    // Get all drives with at least 1GB free space
    let min_free_space = 1024 * 1024 * 1024; // 1GB in bytes
    let suitable_drives: Vec<_> = get_drives_info()
        .into_iter()
        .filter(|drive| drive.available_space >= min_free_space)
        .collect();
    
    if suitable_drives.is_empty() {
        println!("No drives with at least 1GB free space found.");
    } else {
        println!("Drives with at least 1GB free space:");
        for drive in &suitable_drives {
            println!("  {} ({:.2} GB free)", 
                drive.path.display(), 
                bytes_to_gb(drive.available_space)
            );
        }
        
        // For a real application, you would benchmark these drives
        // and choose the one with the best performance/space ratio
        println!("\nIn a real application, you would benchmark these drives");
        println!("and choose the one with the best performance/space ratio.");
    }

    Ok(())
}

// Helper function to convert bytes to gigabytes
fn bytes_to_gb(bytes: u64) -> f64 {
    bytes as f64 / (1024.0 * 1024.0 * 1024.0)
}
