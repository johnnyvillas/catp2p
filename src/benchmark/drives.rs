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

//! Drive benchmarking functionality for assessing storage performance.

use crate::error::Error;
use std::fs::{File, OpenOptions};
use std::io::{Read, Write, Seek, SeekFrom};
use std::path::{Path, PathBuf};
use std::time::Instant;
use sysinfo::{System, SystemExt, DiskExt};

/// Result of a drive benchmark containing detailed performance metrics.
#[derive(Debug, Clone)]
pub struct DriveBenchmarkResult {
    /// Path to the drive or directory that was benchmarked
    pub path: PathBuf,
    /// Write speed in MB/s
    pub write_speed: f64,
    /// Read speed in MB/s
    pub read_speed: f64,
    /// Random access operations per second
    pub random_access_speed: f64,
    /// Overall benchmark score (higher is better)
    pub overall_score: f64,
    /// Total capacity of the drive in bytes
    pub total_capacity: u64,
    /// Available space on the drive in bytes
    pub available_space: u64,
}

/// Configuration options for drive benchmarks.
#[derive(Debug, Clone)]
pub struct DriveBenchmarkConfig {
    /// Size of the test file in MB
    pub file_size_mb: usize,
    /// Number of random access operations to perform
    pub random_access_ops: usize,
    /// Whether to include random access test
    pub include_random_access: bool,
}

impl Default for DriveBenchmarkConfig {
    fn default() -> Self {
        Self {
            file_size_mb: 100,
            random_access_ops: 1000,
            include_random_access: true,
        }
    }
}

/// Runs a drive benchmark on the system's temporary directory.
/// 
/// This is a convenience function that maintains backward compatibility
/// with the original API. For more control, use `run_drive_benchmark_with_config`.
pub fn run_drive_benchmark() -> Result<f64, Error> {
    let temp_dir = std::env::temp_dir();
    let result = run_drive_benchmark_with_config(&temp_dir, &DriveBenchmarkConfig::default())?;
    Ok(result.overall_score)
}

/// Runs a drive benchmark on a specific directory with custom configuration.
///
/// # Arguments
///
/// * `target_dir` - The directory to benchmark
/// * `config` - Configuration options for the benchmark
///
/// # Returns
///
/// A `DriveBenchmarkResult` containing detailed performance metrics
pub fn run_drive_benchmark_with_config(
    target_dir: &Path,
    config: &DriveBenchmarkConfig,
) -> Result<DriveBenchmarkResult, Error> {
    if !target_dir.exists() {
        return Err(Error::Benchmark(format!(
            "Target directory does not exist: {}",
            target_dir.display()
        )));
    }

    // Create a safer path for the temporary file
    let temp_file_path = create_safe_temp_path(target_dir)?;
    
    // Run the benchmark
    let start_time = Instant::now();
    
    // Write benchmark
    let write_speed = run_write_benchmark(&temp_file_path, config.file_size_mb)?;
    
    // Read benchmark
    let read_speed = run_read_benchmark(&temp_file_path)?;
    
    // Random access benchmark (optional)
    let random_access_speed = if config.include_random_access {
        run_random_access_benchmark(&temp_file_path, config.random_access_ops)?
    } else {
        0.0
    };
    
    // Clean up
    if let Err(e) = std::fs::remove_file(&temp_file_path) {
        // Just log the error but don't fail the benchmark if cleanup fails
        eprintln!("Warning: Failed to remove temporary file {}: {}", temp_file_path.display(), e);
    }
    
    let _elapsed = start_time.elapsed();
    
    // Get drive information
    let (total_capacity, available_space) = get_drive_info(target_dir)?;
    
    // Calculate the score based on the speeds
    // If random access is not included, only average write and read speeds
    let overall_score = if config.include_random_access {
        (write_speed + read_speed + random_access_speed) / 3.0
    } else {
        (write_speed + read_speed) / 2.0
    };
    
    Ok(DriveBenchmarkResult {
        path: target_dir.to_path_buf(),
        write_speed,
        read_speed,
        random_access_speed,
        overall_score,
        total_capacity,
        available_space,
    })
}

/// Creates a safe path for a temporary file on the specified drive
fn create_safe_temp_path(drive_path: &Path) -> Result<PathBuf, Error> {
    // First try to use a dedicated temp directory on the drive
    let temp_dir_candidates = [
        // Windows-specific paths
        drive_path.join("Temp"),
        drive_path.join("Windows").join("Temp"),
        drive_path.join("Users").join("Public").join("Documents").join("Temp"),
        
        // Unix-like paths
        drive_path.join("tmp"),
        drive_path.join("var").join("tmp"),
    ];
    
    // Try to find an existing temp directory
    for temp_dir in &temp_dir_candidates {
        if temp_dir.exists() && is_directory_writable(temp_dir) {
            return Ok(temp_dir.join("catp2p_drive_benchmark.tmp"));
        }
    }
    
    // If no suitable temp directory found, try to create one
    let catp2p_temp_dir = drive_path.join("catp2p_temp");
    if !catp2p_temp_dir.exists() {
        match std::fs::create_dir(&catp2p_temp_dir) {
            Ok(_) => return Ok(catp2p_temp_dir.join("benchmark.tmp")),
            Err(_) => {} // Continue to next option if this fails
        }
    } else if is_directory_writable(&catp2p_temp_dir) {
        return Ok(catp2p_temp_dir.join("benchmark.tmp"));
    }
    
    // As a last resort, try to use the system's temp directory but on the same drive
    let system_temp = std::env::temp_dir();
    if system_temp.exists() && is_directory_writable(&system_temp) {
        // Generate a unique filename to avoid conflicts
        let unique_id = std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)
            .unwrap_or_default()
            .as_millis();
        
        return Ok(system_temp.join(format!("catp2p_drive_benchmark_{}.tmp", unique_id)));
    }
    
    // If all else fails, try to use the provided path directly
    // but warn that this might fail due to permissions
    eprintln!("Warning: Could not find a suitable temporary directory on {}. Attempting to use the drive root, which may fail due to permissions.", drive_path.display());
    Ok(drive_path.join("catp2p_drive_benchmark.tmp"))
}


/// Runs benchmarks on all available drives.
///
/// # Returns
///
/// A vector of `DriveBenchmarkResult` for each drive
pub fn run_all_drives_benchmark() -> Result<Vec<DriveBenchmarkResult>, Error> {
    run_all_drives_benchmark_with_config(&DriveBenchmarkConfig::default())
}

/// Runs benchmarks on all available drives with custom configuration.
///
/// # Arguments
///
/// * `config` - Configuration options for the benchmarks
///
/// # Returns
///
/// A vector of `DriveBenchmarkResult` for each drive
pub fn run_all_drives_benchmark_with_config(
    config: &DriveBenchmarkConfig,
) -> Result<Vec<DriveBenchmarkResult>, Error> {
    let system = System::new_all();
    let mut results = Vec::new();
    
    for disk in system.disks() {
        let mount_point = disk.mount_point();
        
        // Skip system drives on Windows (like C:) if they don't have enough free space
        #[cfg(target_os = "windows")]
        {
            let available_space = disk.available_space();
            if available_space < (config.file_size_mb as u64 * 1024 * 1024 * 2) {
                continue;
            }
        }
        
        // Create a temporary directory on this drive for testing
        let test_dir = Path::new(mount_point);
        
        // Skip if we can't write to this directory
        if !is_directory_writable(test_dir) {
            continue;
        }
        
        match run_drive_benchmark_with_config(test_dir, config) {
            Ok(result) => results.push(result),
            Err(e) => {
                // Log the error but continue with other drives
                eprintln!("Error benchmarking drive {}: {}", test_dir.display(), e);
            }
        }
    }
    
    Ok(results)
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
pub fn run_random_access_benchmark(file_path: &Path, num_accesses: usize) -> Result<f64, Error> {
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

/// Gets information about a drive.
fn get_drive_info(path: &Path) -> Result<(u64, u64), Error> {
    let system = System::new_all();
    
    for disk in system.disks() {
        let mount_point = disk.mount_point();
        if path.starts_with(mount_point) {
            return Ok((disk.total_space(), disk.available_space()));
        }
    }
    
    Err(Error::Benchmark(format!(
        "Could not find drive information for path: {}",
        path.display()
    )))
}

/// Checks if a directory is writable.
fn is_directory_writable(dir: &Path) -> bool {
    let test_file = dir.join(".catp2p_write_test");
    match OpenOptions::new().write(true).create(true).open(&test_file) {
        Ok(_) => {
            // Clean up the test file
            let _ = std::fs::remove_file(test_file);
            true
        }
        Err(_) => false,
    }
}

/// Gets a list of all available drives.
pub fn get_available_drives() -> Vec<PathBuf> {
    let system = System::new_all();
    let mut drives = Vec::new();
    
    for disk in system.disks() {
        drives.push(disk.mount_point().to_path_buf());
    }
    
    drives
}

/// Gets detailed information about all drives.
pub struct DriveInfo {
    /// Path to the drive
    pub path: PathBuf,
    /// Name of the drive
    pub name: String,
    /// Total capacity in bytes
    pub total_capacity: u64,
    /// Available space in bytes
    pub available_space: u64,
    /// File system type (e.g., NTFS, ext4)
    pub file_system: String,
    /// Whether the drive is removable
    pub is_removable: bool,
}

/// Gets information about all available drives.
pub fn get_drives_info() -> Vec<DriveInfo> {
    let system = System::new_all();
    let mut drives_info = Vec::new();
    
    for disk in system.disks() {
        drives_info.push(DriveInfo {
            path: disk.mount_point().to_path_buf(),
            // For name, which returns &OsStr
            name: disk.name().to_string_lossy().into_owned(),
            total_capacity: disk.total_space(),
            available_space: disk.available_space(),
            // For file_system, which returns &[u8]
            file_system: String::from_utf8_lossy(disk.file_system()).into_owned(),
            is_removable: disk.is_removable(),
        });
    }
    
    drives_info
}



