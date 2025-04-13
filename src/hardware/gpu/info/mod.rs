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

//! GPU information utilities.

use crate::error::Error;
use wgpu::Adapter;
use std::collections::HashMap;
use std::fmt;
use std::process::Command;
use std::time::{Duration, Instant};
use regex::Regex;

// Import platform-specific modules
mod common; // Add the common module
#[cfg(target_os = "windows")]
mod windows;
#[cfg(target_os = "linux")]
mod linux;
#[cfg(target_os = "macos")]
mod macos;

/// GPU information structure with comprehensive details.
#[derive(Debug, Clone, Default)]
pub struct GpuInfo {
    /// GPU model name
    pub name: String,
    /// GPU vendor
    pub vendor: String,
    /// GPU architecture/generation
    pub architecture: String,
    /// GPU driver info
    pub driver: String,
    /// Total VRAM in bytes
    pub vram_bytes: u64,
    /// Total VRAM formatted as string (e.g., "8.0 GB")
    pub vram: String,
    /// Available/free VRAM in bytes
    pub vram_free_bytes: Option<u64>,
    /// Available/free VRAM formatted as string
    pub vram_free: Option<String>,
    /// Memory type (GDDR5, GDDR6, HBM2, etc.)
    pub memory_type: String,
    /// Memory bus width (in bits)
    pub memory_bus_width: Option<u32>,
    /// Core/shader count
    pub core_count: Option<u32>,
    /// Base clock speed (MHz)
    pub base_clock_mhz: Option<u32>,
    /// Boost clock speed (MHz)
    pub boost_clock_mhz: Option<u32>,
    /// Compute capability/version
    pub compute_capability: String,
    /// Current temperature (in Celsius)
    pub temperature: Option<f32>,
    /// Power usage (Watts)
    pub power_usage_watts: Option<f32>,
    /// Thermal Design Power (TDP) in Watts
    pub tdp_watts: Option<u32>,
    /// PCIe version
    pub pcie_version: String,
    /// PCIe link width
    pub pcie_width: Option<u32>,
    /// Supported APIs and versions
    pub api_support: HashMap<String, String>,
    /// Performance characteristics (TFLOPS)
    pub tflops: Option<f32>,
    /// Physical dimensions/form factor
    pub form_factor: String,
    /// Current GPU utilization percentage
    pub utilization_percent: Option<f32>,
    /// Memory utilization percentage
    pub memory_utilization_percent: Option<f32>,
    /// Encoder utilization percentage
    pub encoder_utilization_percent: Option<f32>,
    /// Decoder utilization percentage
    pub decoder_utilization_percent: Option<f32>,
    /// Maximum supported resolution
    pub max_resolution: String,
    /// Backend used (Vulkan, DirectX, Metal, etc.)
    pub backend: String,
    /// Whether the GPU is integrated or discrete
    pub is_integrated: bool,
    /// Additional properties that don't fit in the standard fields
    pub additional_properties: HashMap<String, String>,
}

/// Temperature unit for conversion
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum TemperatureUnit {
    /// Celsius (°C) - Default
    Celsius,
    /// Fahrenheit (°F)
    Fahrenheit,
}

impl GpuInfo {
    /// Get temperature in the specified unit
    /// 
    /// # Arguments
    /// 
    /// * `unit` - The temperature unit to convert to
    /// 
    /// # Returns
    /// 
    /// The temperature in the specified unit, or None if temperature is not available
    /// 
    /// # Examples
    /// 
    /// ```
    /// use catp2p::hardware::gpu::{get_info, TemperatureUnit};
    /// 
    /// let gpu_info = get_info().unwrap();
    /// 
    /// // Get temperature in Celsius (default)
    /// if let Some(temp_c) = gpu_info.temperature {
    ///     println!("Temperature: {}°C", temp_c);
    /// }
    /// 
    /// // Get temperature in Fahrenheit
    /// if let Some(temp_f) = gpu_info.temperature_in(TemperatureUnit::Fahrenheit) {
    ///     println!("Temperature: {}°F", temp_f);
    /// }
    /// ```
    pub fn temperature_in(&self, unit: TemperatureUnit) -> Option<f32> {
        self.temperature.map(|temp_c| {
            match unit {
                TemperatureUnit::Celsius => temp_c,
                TemperatureUnit::Fahrenheit => (temp_c * 9.0/5.0) + 32.0,
            }
        })
    }
}

impl fmt::Display for GpuInfo {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        writeln!(f, "GPU Information:")?;
        writeln!(f, "  Name: {}", self.name)?;
        writeln!(f, "  Vendor: {}", self.vendor)?;
        writeln!(f, "  Architecture: {}", self.architecture)?;
        writeln!(f, "  Driver: {}", self.driver)?;
        writeln!(f, "  Backend: {}", self.backend)?;
        writeln!(f, "  Type: {}", if self.is_integrated { "Integrated" } else { "Discrete" })?;
        
        if !self.vram.is_empty() && self.vram != "Unknown" {
            writeln!(f, "  VRAM: {}", self.vram)?;
        }
        
        if let Some(temp) = self.temperature {
            writeln!(f, "  Temperature: {:.1}°C", temp)?;
        }
        
        if !self.additional_properties.is_empty() {
            writeln!(f, "  Additional Properties:")?;
            for (key, value) in &self.additional_properties {
                writeln!(f, "    {}: {}", key, value)?;
            }
        }
        
        Ok(())
    }
}


/// GPU usage information structure.
#[derive(Debug, Clone)]
pub struct GpuUsageInfo {
    /// GPU model name
    pub name: String,
    /// GPU vendor
    pub vendor: String,
    /// Total VRAM in bytes
    pub total_vram_bytes: u64,
    /// Total VRAM formatted as string (e.g., "8.0 GB")
    pub total_vram: String,
    /// Used VRAM in bytes
    pub used_vram_bytes: u64,
    /// Used VRAM formatted as string
    pub used_vram: String,
    /// GPU usage percentage
    pub gpu_usage_percent: f32,
    /// Timestamp when this information was collected
    pub timestamp: Instant,
}

impl Default for GpuUsageInfo {
    fn default() -> Self {
        Self {
            name: String::new(),
            vendor: String::new(),
            total_vram_bytes: 0,
            total_vram: String::new(),
            used_vram_bytes: 0,
            used_vram: String::new(),
            gpu_usage_percent: 0.0,
            timestamp: Instant::now(),
        }
    }
}


/// Gets GPU information from the adapter.
pub fn get_gpu_info_from_adapter(adapter: &Adapter) -> Result<GpuInfo, Error> {
    let info = adapter.get_info();
    
    // Create a basic GpuInfo from adapter info
    let mut gpu_info = GpuInfo {
        name: info.name.clone(),
        vendor: format!("{:?}", info.vendor),
        backend: format!("{:?}", info.backend),
        is_integrated: matches!(info.device_type, wgpu::DeviceType::IntegratedGpu),
        driver: format!("{:?}", info.driver),
        architecture: "Unknown".to_string(),
        ..Default::default()
    };
    
    // Try to get platform-specific information using the common module
    common::enhance_gpu_info(&mut gpu_info);
    
    // If architecture is still unknown, try to determine from name and vendor
    if gpu_info.architecture == "Unknown" {
        gpu_info.architecture = determine_architecture(&gpu_info.name, &gpu_info.vendor);
    }
    
    Ok(gpu_info)
}

/// Gets information about the GPU.
pub fn get_gpu_info() -> Result<GpuInfo, Error> {
    // Initialize wgpu
    let instance = wgpu::Instance::new(wgpu::InstanceDescriptor {
        backends: wgpu::Backends::all(),
        dx12_shader_compiler: Default::default(),
    });
    
    // Request adapter without surface (headless)
    let adapter_result = pollster::block_on(async {
        // First try high performance adapter
        let high_perf = instance.request_adapter(&wgpu::RequestAdapterOptions {
            power_preference: wgpu::PowerPreference::HighPerformance,
            compatible_surface: None,
            force_fallback_adapter: false,
        }).await;
        
        if high_perf.is_some() {
            high_perf
        } else {
            // If that fails, try low power adapter
            instance.request_adapter(&wgpu::RequestAdapterOptions {
                power_preference: wgpu::PowerPreference::LowPower,
                compatible_surface: None,
                force_fallback_adapter: true,
            }).await
        }
    });
    
    match adapter_result {
        Some(adapter) => {
            // Get GPU information from the adapter
            get_gpu_info_from_adapter(&adapter)
        },
        None => {
            // No adapter found, try to get information from platform-specific fallback methods
            let platform_fallback = {
                #[cfg(target_os = "windows")]
                {
                    windows::get_fallback_gpu_info()
                }
                
                #[cfg(target_os = "linux")]
                {
                    linux::get_fallback_gpu_info()
                }
                
                #[cfg(target_os = "macos")]
                {
                    macos::get_fallback_gpu_info()
                }
                
                #[cfg(not(any(target_os = "windows", target_os = "linux", target_os = "macos")))]
                {
                    Err(Error::Benchmark("No suitable GPU adapter found".to_string()))
                }
            };
            
            platform_fallback
        }
    }
}

/// Gets information about all available GPUs.
pub fn get_all_gpu_info() -> Result<Vec<GpuInfo>, Error> {
    // Initialize wgpu
    let instance = wgpu::Instance::new(wgpu::InstanceDescriptor {
        backends: wgpu::Backends::all(),
        dx12_shader_compiler: Default::default(),
    });
    
    // Get all available adapters
    let adapters = pollster::block_on(async {
        let mut adapters = Vec::new();
        
        // Try high performance adapters first
        if let Some(adapter) = instance.request_adapter(&wgpu::RequestAdapterOptions {
            power_preference: wgpu::PowerPreference::HighPerformance,
            compatible_surface: None,
            force_fallback_adapter: false,
        }).await {
            adapters.push(adapter);
        }
        
        // Then try low power adapters
        if let Some(adapter) = instance.request_adapter(&wgpu::RequestAdapterOptions {
            power_preference: wgpu::PowerPreference::LowPower,
            compatible_surface: None,
            force_fallback_adapter: false,
        }).await {
            // Check if this is a different adapter than the one we already have
            let info = adapter.get_info();
            if adapters.is_empty() || adapters[0].get_info().name != info.name {
                adapters.push(adapter);
            }
        }
        
        // Finally try fallback adapter
        if let Some(adapter) = instance.request_adapter(&wgpu::RequestAdapterOptions {
            power_preference: wgpu::PowerPreference::LowPower,
            compatible_surface: None,
            force_fallback_adapter: true,
        }).await {
            // Check if this is a different adapter than the ones we already have
            let info = adapter.get_info();
            if adapters.iter().all(|a| a.get_info().name != info.name) {
                adapters.push(adapter);
            }
        }
        
        adapters
    });
    
    // If no adapters found, try platform-specific fallbacks
    if adapters.is_empty() {
        let mut gpu_infos = Vec::new();
        
        #[cfg(target_os = "windows")]
        {
            if let Ok(info) = windows::get_fallback_gpu_info() {
                gpu_infos.push(info);
                return Ok(gpu_infos);
            }
        }
        
        #[cfg(target_os = "linux")]
        {
            if let Ok(info) = linux::get_fallback_gpu_info() {
                gpu_infos.push(info);
                return Ok(gpu_infos);
            }
        }
        
        #[cfg(target_os = "macos")]
        {
            if let Ok(info) = macos::get_fallback_gpu_info() {
                gpu_infos.push(info);
                return Ok(gpu_infos);
            }
        }
        
        return Err(Error::Benchmark("No GPU adapters found".to_string()));
    }
    
    // Get info for each adapter
    let mut gpu_infos = Vec::new();
    for adapter in &adapters {
        match get_gpu_info_from_adapter(adapter) {
            Ok(info) => {
                // Filter out Microsoft Basic Render Driver and other software renderers
                if !is_software_renderer(&info.name) {
                    gpu_infos.push(info);
                }
            },
            Err(e) => eprintln!("Failed to get info for adapter: {}", e),
        }
    }
    
    // If we filtered out all GPUs, try to get at least one
    if gpu_infos.is_empty() {
        for adapter in &adapters {
            match get_gpu_info_from_adapter(adapter) {
                Ok(info) => {
                    gpu_infos.push(info);
                    break;
                },
                Err(e) => eprintln!("Failed to get info for adapter: {}", e),
            }
        }
    }
    
    Ok(gpu_infos)
}

/// Checks if a GPU is a software renderer
fn is_software_renderer(name: &str) -> bool {
    name.contains("Basic Render") || 
    name.contains("Microsoft Remote Display") || 
    name.contains("Software Adapter") ||
    name.contains("WARP") ||
    name.contains("llvmpipe")
}


/// Gets current usage information for the primary GPU.
pub fn get_gpu_usage() -> Result<GpuUsageInfo, Error> {
    // First get basic GPU info
    let gpu_info = get_gpu_info()?;
    
    // Create a usage info struct with basic information
    let mut usage_info = GpuUsageInfo {
        name: gpu_info.name,
        vendor: gpu_info.vendor,
        timestamp: Instant::now(),
        ..Default::default()
    };
    
    // Get platform-specific usage information
    #[cfg(target_os = "windows")]
    windows::get_gpu_usage(&mut usage_info);
    
    #[cfg(target_os = "linux")]
    linux::get_gpu_usage(&mut usage_info);
    
    #[cfg(target_os = "macos")]
    macos::get_gpu_usage(&mut usage_info);
    
    Ok(usage_info)
}

/// Gets current usage information for a specific GPU by name.
pub fn get_gpu_usage_by_name(gpu_name: &str) -> Result<GpuUsageInfo, Error> {
    // First get all GPUs
    let gpu_infos = get_all_gpu_info()?;
    
    // Find the GPU with the matching name
    let gpu_info = gpu_infos.iter()
        .find(|info| info.name.contains(gpu_name))
        .ok_or_else(|| Error::Benchmark(format!("GPU with name '{}' not found", gpu_name)))?;
    
        // Create a usage info struct with basic information
        let mut usage_info = GpuUsageInfo {
            name: gpu_info.name.clone(),
            vendor: gpu_info.vendor.clone(),
            timestamp: Instant::now(),
            ..Default::default()
        };
        
        // Get platform-specific usage information
        #[cfg(target_os = "windows")]
        windows::get_gpu_usage_by_name(&mut usage_info);
        
        #[cfg(target_os = "linux")]
        linux::get_gpu_usage_by_name(&mut usage_info);
        
        #[cfg(target_os = "macos")]
        macos::get_gpu_usage_by_name(&mut usage_info);
        
        Ok(usage_info)
    }
    
    /// Monitors GPU usage over a specified duration.
    pub fn monitor_gpu_usage(duration: Duration, sample_interval: Duration) -> Result<super::GpuUsageStats, Error> {
        // First get basic GPU info
        let gpu_info = get_gpu_info()?;
        
        // Initialize stats
        let mut stats = super::GpuUsageStats {
            name: gpu_info.name.clone(),
            vendor: gpu_info.vendor.clone(),
            total_vram: gpu_info.vram.clone(),
            avg_usage_percent: 0.0,
            min_usage_percent: f32::MAX,
            max_usage_percent: 0.0,
            avg_used_vram: "0 B".to_string(),
            min_used_vram: "0 B".to_string(),
            max_used_vram: "0 B".to_string(),
            sample_count: 0,
            duration: Duration::from_secs(0),
        };
        
        // Collect samples
        let start_time = Instant::now();
        let mut total_usage_percent = 0.0;
        let mut total_used_vram_bytes = 0;
        let mut min_used_vram_bytes = u64::MAX;
        let mut max_used_vram_bytes = 0;
        
        while start_time.elapsed() < duration {
            // Get current usage
            match get_gpu_usage() {
                Ok(usage) => {
                    // Update usage stats
                    total_usage_percent += usage.gpu_usage_percent;
                    stats.min_usage_percent = stats.min_usage_percent.min(usage.gpu_usage_percent);
                    stats.max_usage_percent = stats.max_usage_percent.max(usage.gpu_usage_percent);
                    
                    // Update VRAM stats
                    total_used_vram_bytes += usage.used_vram_bytes;
                    min_used_vram_bytes = min_used_vram_bytes.min(usage.used_vram_bytes);
                    max_used_vram_bytes = max_used_vram_bytes.max(usage.used_vram_bytes);
                    
                    stats.sample_count += 1;
                },
                Err(e) => {
                    eprintln!("Error getting GPU usage: {}", e);
                }
            }
            
            // Sleep for the sample interval
            if start_time.elapsed() + sample_interval < duration {
                std::thread::sleep(sample_interval);
            } else {
                break;
            }
        }
        
        // Calculate averages
        if stats.sample_count > 0 {
            stats.avg_usage_percent = total_usage_percent / stats.sample_count as f32;
            let avg_used_vram_bytes = total_used_vram_bytes / stats.sample_count as u64;
            
            stats.avg_used_vram = format_bytes(avg_used_vram_bytes);
            stats.min_used_vram = format_bytes(min_used_vram_bytes);
            stats.max_used_vram = format_bytes(max_used_vram_bytes);
        }
        
        stats.duration = start_time.elapsed();
        
        Ok(stats)
    }
    
    /// Checks if GPU is available.
    pub fn is_gpu_available() -> bool {
        match get_gpu_info() {
            Ok(_) => true,
            Err(_) => false,
        }
    }
    
    /// Formats bytes to a human-readable string
    pub fn format_bytes(bytes: u64) -> String {
        const KB: u64 = 1024;
        const MB: u64 = KB * 1024;
        const GB: u64 = MB * 1024;
        
        if bytes >= GB {
            format!("{:.1} GB", bytes as f64 / GB as f64)
        } else if bytes >= MB {
            format!("{:.1} MB", bytes as f64 / MB as f64)
        } else if bytes >= KB {
            format!("{:.1} KB", bytes as f64 / KB as f64)
        } else {
            format!("{} B", bytes)
        }
    }
    
    /// Runs a command and returns the trimmed output if successful
    pub fn run_command(cmd: &str, args: &[&str]) -> Option<String> {
        match Command::new(cmd).args(args).output() {
            Ok(output) if output.status.success() => {
                Some(String::from_utf8_lossy(&output.stdout).trim().to_string())
            },
            _ => None,
        }
    }
    
    /// Extracts VRAM information from GPU name using regex patterns
    pub fn extract_vram_from_name(gpu_name: &str) -> Option<String> {
        // Common patterns for VRAM in GPU names
        let patterns = [
            (r"(\d+)\s*GB", 1024 * 1024 * 1024),      // Match "8GB" or "8 GB"
            (r"(\d+)\s*G\b", 1024 * 1024 * 1024),     // Match "8G" or "8 G"
            (r"(\d+)\s*MB", 1024 * 1024),             // Match "8MB" or "8 MB"
            (r"(\d+)\s*M\b", 1024 * 1024),            // Match "8M" or "8 M"
        ];
        
        for (pattern, multiplier) in patterns.iter() {
            if let Ok(re) = Regex::new(pattern) {
                if let Some(captures) = re.captures(gpu_name) {
                    if let Some(size_match) = captures.get(1) {
                        if let Ok(size) = size_match.as_str().parse::<u64>() {
                            let bytes = size * multiplier;
                            let gb = (bytes as f64) / (1024.0 * 1024.0 * 1024.0);
                            return Some(format!("{:.1} GB", gb));
                        }
                    }
                }
            }
        }
        
        None
    }
    
    /// Determines GPU architecture from device name and vendor
    pub fn determine_architecture(device_name: &str, vendor: &str) -> String {
        // NVIDIA GPUs
        if device_name.contains("NVIDIA") || vendor.contains("NVIDIA") {
            if device_name.contains("RTX 40") || device_name.contains("AD10") {
                return "Ada Lovelace".to_string();
            } else if device_name.contains("RTX 30") || device_name.contains("GA10") {
                return "Ampere".to_string();
            } else if device_name.contains("RTX 20") || device_name.contains("TU10") {
                return "Turing".to_string();
            } else if device_name.contains("GTX 16") {
                return "Turing (GTX)".to_string();
            } else if device_name.contains("GTX 10") || device_name.contains("GP10") {
                return "Pascal".to_string();
            } else if device_name.contains("GTX 9") || device_name.contains("GM20") {
                return "Maxwell".to_string();
            } else if device_name.contains("GTX 7") || device_name.contains("GK10") {
                return "Kepler".to_string();
            }
        }
        
        // AMD GPUs
        if device_name.contains("AMD") || device_name.contains("Radeon") || vendor.contains("AMD") {
            if device_name.contains("RX 7") || device_name.contains("Navi 3") {
                return "RDNA 3".to_string();
            } else if device_name.contains("RX 6") || device_name.contains("Navi 2") {
                return "RDNA 2".to_string();
            } else if device_name.contains("RX 5") || device_name.contains("Navi 1") {
                return "RDNA".to_string();
            } else if device_name.contains("Vega") {
                return "Vega (GCN 5)".to_string();
            } else if device_name.contains("Polaris") || device_name.contains("RX 4") {
                return "Polaris (GCN 4)".to_string();
            }
        }
        
        // Intel GPUs
        if device_name.contains("Intel") || vendor.contains("Intel") {
            if device_name.contains("Arc") || device_name.contains("DG2") {
                return "Xe HPG".to_string();
            } else if device_name.contains("Iris Xe") || device_name.contains("DG1") {
                return "Xe LP".to_string();
            } else if device_name.contains("UHD Graphics") || device_name.contains("HD Graphics") {
                return "Gen9/Gen11".to_string();
            }
        }
        
        // Apple GPUs
        if device_name.contains("Apple") || vendor.contains("Apple") {
            if device_name.contains("M3") {
                return "Apple Silicon (M3)".to_string();
            } else if device_name.contains("M2") {
                return "Apple Silicon (M2)".to_string();
            } else if device_name.contains("M1") {
                return "Apple Silicon (M1)".to_string();
            } else {
                return "Apple Silicon".to_string();
            }
        }
        
        // If we couldn't determine the architecture, return Unknown
        "Unknown".to_string()
    }
    
        /// Gets GPU information using a platform-specific fallback method
        #[cfg(not(any(target_os = "windows", target_os = "linux", target_os = "macos")))]
        pub fn get_fallback_gpu_info() -> Result<GpuInfo, Error> {
            Err(Error::Benchmark("No suitable GPU adapter found".to_string()))
        }
    