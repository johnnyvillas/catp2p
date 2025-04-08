---
sidebar_position: 2
---

# Drive Benchmarking API Reference

This page provides detailed API reference for the drive benchmarking functionality in CatP2P.

## Structures

### `DriveBenchmarkResult`

Contains detailed information about the results of a drive benchmark.

| Field | Type | Description | Example Access |
|-------|------|-------------|----------------|
| `path` | PathBuf | Path to the drive or directory that was benchmarked | `result.path.display()` |
| `write_speed` | f64 | Write speed in MB/s | `result.write_speed` |
| `read_speed` | f64 | Read speed in MB/s | `result.read_speed` |
| `random_access_speed` | f64 | Random access operations per second | `result.random_access_speed` |
| `overall_score` | f64 | Overall benchmark score (higher is better) | `result.overall_score` |
| `total_capacity` | u64 | Total capacity of the drive in bytes | `result.total_capacity` |
| `available_space` | u64 | Available space on the drive in bytes | `result.available_space` |

### `DriveBenchmarkConfig`

Configuration options for drive benchmarks.

| Field | Type | Description | Default Value | Example Access |
|-------|------|-------------|---------------|----------------|
| `file_size_mb` | usize | Size of the test file in MB | 100 | `config.file_size_mb` |
| `random_access_ops` | usize | Number of random access operations to perform | 1000 | `config.random_access_ops` |
| `include_random_access` | bool | Whether to include random access test | true | `config.include_random_access` |

### `DriveInfo`

Information about a drive.

| Field | Type | Description | Example Access |
|-------|------|-------------|----------------|
| `path` | PathBuf | Path to the drive | `drive_info.path.display()` |
| `name` | String | Name of the drive | `drive_info.name` |
| `total_capacity` | u64 | Total capacity in bytes | `drive_info.total_capacity` |
| `available_space` | u64 | Available space in bytes | `drive_info.available_space` |
| `file_system` | String | File system type (e.g., NTFS, ext4) | `drive_info.file_system` |
| `is_removable` | bool | Whether the drive is removable | `drive_info.is_removable` |

## Functions

### Information Gathering

| Function | Return Type | Description | Example Usage | Possible Errors |
|----------|-------------|-------------|--------------|-----------------|
| `get_drives_info()` | Vec\<DriveInfo\> | Gets information about all available drives | `let drives = drives::get_drives_info();` | None - returns empty vector if no drives found |
| `get_available_drives()` | Vec\<PathBuf\> | Gets a list of all available drive paths | `let drive_paths = drives::get_available_drives();` | None - returns empty vector if no drives found |
| `get_drive_info(path: &Path)` | Result\<(u64, u64), Error\> | Gets capacity and available space for a specific drive | `let (total, available) = drives::get_drive_info(&path)?;` | Drive not found or inaccessible |

### Performance Testing

| Function | Return Type | Description | Example Usage | Performance Impact |
|----------|-------------|-------------|--------------|-------------------|
| `run_drive_benchmark()` | Result\<f64, Error\> | Runs a benchmark on the system's temp directory | `let score = drives::run_drive_benchmark()?;` | Medium - creates and reads a 100MB file |
| `run_drive_benchmark_with_config(target_dir: &Path, config: &DriveBenchmarkConfig)` | Result\<DriveBenchmarkResult, Error\> | Runs a benchmark on a specific directory with custom configuration | `let result = drives::run_drive_benchmark_with_config(&path, &config)?;` | Varies based on configuration |
| `run_all_drives_benchmark()` | Result\<Vec\<DriveBenchmarkResult\>, Error\> | Benchmarks all available drives with default configuration | `let results = drives::run_all_drives_benchmark()?;` | High - benchmarks all drives |
| `run_all_drives_benchmark_with_config(config: &DriveBenchmarkConfig)` | Result\<Vec\<DriveBenchmarkResult\>, Error\> | Benchmarks all available drives with custom configuration | `let results = drives::run_all_drives_benchmark_with_config(&config)?;` | Varies based on configuration |
| `run_write_benchmark(file_path: &Path, size_mb: usize)` | Result\<f64, Error\> | Runs only the write portion of the benchmark | `let write_speed = drives::run_write_benchmark(&path, 100)?;` | Medium - writes a file of specified size |
| `run_read_benchmark(file_path: &Path)` | Result\<f64, Error\> | Runs only the read portion of the benchmark | `let read_speed = drives::run_read_benchmark(&path)?;` | Medium - reads an existing file |
| `run_random_access_benchmark(file_path: &Path, num_accesses: usize)` | Result\<f64, Error\> | Runs only the random access portion of the benchmark | `let random_speed = drives::run_random_access_benchmark(&path, 1000)?;` | Low to Medium - performs random reads |

### Function Relationships

| Function | Related Functions | Notes |
|----------|-------------------|-------|
| `run_drive_benchmark()` | `run_drive_benchmark_with_config()` | Simplified version that uses default configuration |
| `run_all_drives_benchmark()` | `run_all_drives_benchmark_with_config()` | Simplified version that uses default configuration |
| `run_drive_benchmark_with_config()` | `run_write_benchmark()`, `run_read_benchmark()`, `run_random_access_benchmark()` | Combines all three benchmark types |
| `get_drives_info()` | `get_available_drives()`, `get_drive_info()` | Provides more detailed information than `get_available_drives()` |

### Parameter Details

| Function | Parameter | Description | Recommended Values |
|----------|-----------|-------------|-------------------|
| `run_drive_benchmark_with_config()` | `target_dir` | Directory to benchmark | Any directory with write permissions |
| `run_drive_benchmark_with_config()` | `config` | Benchmark configuration | `DriveBenchmarkConfig::default()` or custom |
| `run_write_benchmark()` | `file_path` | Path to create test file | Temporary file in writable directory |
| `run_write_benchmark()` | `size_mb` | Size of test file in MB | 50-500 (larger for more accurate results) |
| `run_random_access_benchmark()` | `num_accesses` | Number of random reads | 500-5000 (more for more accurate results) |

## Understanding Drive Benchmark Results

The drive benchmark in CatP2P measures several aspects of storage performance:

1. **Write Speed**: How quickly data can be written to the drive (in MB/s)
2. **Read Speed**: How quickly data can be read from the drive (in MB/s)
3. **Random Access Speed**: How many random access operations can be performed per second

### Interpreting the Score

The overall drive benchmark score is a composite value that represents:

- Higher scores indicate better drive performance
- Scores are influenced by:
  - Drive technology (SSD vs HDD)
  - Interface type (SATA, NVMe, USB)
  - Drive age and health
  - File system type and fragmentation
  - System load during testing

### Typical Performance Ranges

| Drive Type | Typical Write Speed | Typical Read Speed | Typical Random Access | Expected Score Range |
|------------|---------------------|-------------------|------------------------|---------------------|
| NVMe SSD | 1000-3500 MB/s | 2000-7000 MB/s | 200,000+ ops/s | 100,000+ |
| SATA SSD | 300-550 MB/s | 500-600 MB/s | 80,000-150,000 ops/s | 30,000-80,000 |
| 7200 RPM HDD | 80-160 MB/s | 100-180 MB/s | 300-500 ops/s | 5,000-15,000 |
| External USB 3.0 | 40-120 MB/s | 80-250 MB/s | 1,000-10,000 ops/s | 3,000-25,000 |
| External USB 2.0 | 20-35 MB/s | 25-40 MB/s | 500-1,000 ops/s | 1,000-5,000 |

Note: Actual performance can vary significantly based on specific hardware, system conditions, and benchmark parameters.

### Factors Affecting Benchmark Results

| Factor | Impact | Notes |
|--------|--------|-------|
| System Activity | High | Other processes using the drive can significantly reduce benchmark scores |
| Drive Fullness | Medium | SSDs in particular may slow down as they fill up |
| TRIM Status (SSD) | Medium | SSDs that haven't been TRIMmed recently may show lower performance |
| Thermal Throttling | Medium | Drives may slow down if they overheat during benchmarking |
| File System | Low to Medium | Different file systems have different performance characteristics |
| Drive Fragmentation | Low to High | Heavily fragmented HDDs will show significantly worse performance |
| Benchmark File Size | Medium | Larger test files generally provide more accurate results |

## Implementation Details

### Safe Temporary File Creation

The drive benchmarking functionality includes a robust mechanism for creating temporary files that works across different operating systems and permission models:

1. First tries to use existing temp directories on the drive
2. Falls back to creating a dedicated temp directory if needed
3. Uses the system's temp directory as a last resort
4. Includes proper error handling and cleanup

This approach ensures that benchmarks can run successfully even on drives with restricted permissions, such as system drives.

### Benchmark Methodology

The benchmark process follows these steps:

1. **Write Test**: Creates a file of specified size with random data and measures the time taken
2. **Read Test**: Reads the entire file sequentially and measures the time taken
3. **Random Access Test** (optional): Performs random reads at different positions in the file
4. **Cleanup**: Removes the temporary file
5. **Score Calculation**: Computes a weighted average of the three metrics

The methodology is designed to provide a balanced assessment of drive performance for typical application workloads.

## Error Handling

The drive benchmarking functions use Rust's `Result` type to handle errors gracefully. Common errors include:

- `Error::Benchmark("Failed to create file: {error}")`: Permission issues or disk full
- `Error::Benchmark("Failed to write to file: {error}")`: I/O errors during write
- `Error::Benchmark("Failed to read from file: {error}")`: I/O errors during read
- `Error::Benchmark("Failed to get drive information: {error}")`: System API errors

When benchmarking multiple drives, the library will continue even if some drives fail, returning results for the drives that succeeded.
