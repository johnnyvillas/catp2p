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

//! Common GPU information utilities shared across platforms.

use std::process::Command;
use super::{GpuInfo, format_bytes};

// Platform-specific imports
#[cfg(target_os = "windows")]
use super::windows;
#[cfg(target_os = "linux")]
use super::linux;
#[cfg(target_os = "macos")]
use super::macos;

/// Enhances the GPU info with accurate VRAM information based on GPU vendor
pub fn enhance_gpu_vram(gpu_info: &mut GpuInfo) {
    // For NVIDIA GPUs, use nvidia-smi which is more accurate
    if gpu_info.name.contains("NVIDIA") || gpu_info.vendor.contains("NVIDIA") {
        if let Some(vram_bytes) = get_nvidia_vram() {
            gpu_info.vram = format_bytes(vram_bytes);
            return;
        }
    }
    
    // For AMD GPUs on Linux, try to use AMD-specific methods
    #[cfg(target_os = "linux")]
    if (gpu_info.name.contains("AMD") || gpu_info.vendor.contains("AMD")) && gpu_info.vram.is_empty() {
        if let Some(vram_bytes) = linux::get_amd_vram() {
            gpu_info.vram = format_bytes(vram_bytes);
            return;
        }
    }
    
    // For Apple Silicon, estimate based on system memory
    #[cfg(target_os = "macos")]
    if (gpu_info.name.contains("Apple") || gpu_info.vendor.contains("Apple")) && gpu_info.vram.is_empty() {
        if let Some(vram_bytes) = macos::get_apple_vram() {
            gpu_info.vram = format_bytes(vram_bytes);
            return;
        }
    }
    
    // If we still don't have VRAM info, try to extract from the GPU name
    if gpu_info.vram.is_empty() || gpu_info.vram == "Unknown" {
        if let Some(vram) = super::extract_vram_from_name(&gpu_info.name) {
            gpu_info.vram = vram;
        }
    }
}

/// Gets VRAM information from nvidia-smi (works on all platforms with NVIDIA drivers)
pub fn get_nvidia_vram() -> Option<u64> {
    let output = Command::new("nvidia-smi")
        .args(&["--query-gpu=memory.total", "--format=csv,noheader,nounits"])
        .output()
        .ok()?;
    
    if !output.status.success() {
        return None;
    }
    
    let stdout = String::from_utf8_lossy(&output.stdout).trim().to_string();
    if let Ok(mb) = stdout.parse::<u64>() {
        // Convert MB to bytes
        return Some(mb * 1024 * 1024);
    }
    
    None
}

/// Gets current GPU temperature in Celsius
pub fn get_temperature() -> Option<f32> {
    #[cfg(target_os = "windows")]
    {
        windows::get_temperature()
    }
    
    #[cfg(target_os = "linux")]
    {
        linux::get_temperature()
    }
    
    #[cfg(target_os = "macos")]
    {
        macos::get_temperature()
    }
    
    #[cfg(not(any(target_os = "windows", target_os = "linux", target_os = "macos")))]
    {
        None
    }
}

/// Gets GPU driver version in a cross-platform way
pub fn get_gpu_driver_version(gpu_info: &GpuInfo) -> String {
    // For NVIDIA GPUs, use nvidia-smi which works on all platforms
    if gpu_info.name.contains("NVIDIA") || gpu_info.vendor.contains("NVIDIA") {
        if let Some(driver) = get_nvidia_driver_version() {
            return driver;
        }
    }
    
    // Use platform-specific methods as fallback
    #[cfg(target_os = "windows")]
    {
        return windows::get_driver_version();
    }
    
    #[cfg(target_os = "linux")]
    {
        return linux::get_driver_version();
    }
    
    #[cfg(target_os = "macos")]
    {
        return macos::get_driver_version();
    }
    
    #[cfg(not(any(target_os = "windows", target_os = "linux", target_os = "macos")))]
    {
        "Unknown".to_string()
    }
}

/// Gets NVIDIA driver version using nvidia-smi
fn get_nvidia_driver_version() -> Option<String> {
    let output = Command::new("nvidia-smi")
        .args(&["--query-gpu=driver_version", "--format=csv,noheader,nounits"])
        .output()
        .ok()?;
    
    if !output.status.success() {
        return None;
    }
    
    let stdout = String::from_utf8_lossy(&output.stdout).trim().to_string();
    if !stdout.is_empty() {
        return Some(stdout);
    }
    
    None
}

/// Enhances the GPU info with platform-independent information
pub fn enhance_gpu_info(gpu_info: &mut GpuInfo) {
    // First enhance with accurate VRAM information
    enhance_gpu_vram(gpu_info);
    
    // Get driver information if not already set
    if gpu_info.driver.is_empty() || gpu_info.driver == "Unknown" {
        gpu_info.driver = get_gpu_driver_version(gpu_info);
    }
    
    // Get temperature information
    if gpu_info.temperature.is_none() {
        gpu_info.temperature = get_temperature();
    }
    
    // Now call platform-specific enhancement
    #[cfg(target_os = "windows")]
    windows::enhance_gpu_info(gpu_info);
    
    #[cfg(target_os = "linux")]
    linux::enhance_gpu_info(gpu_info);
    
    #[cfg(target_os = "macos")]
    macos::enhance_gpu_info(gpu_info);
    
    // If architecture is still unknown, try to determine from name and vendor
    if gpu_info.architecture.is_empty() || gpu_info.architecture == "Unknown" {
        gpu_info.architecture = super::determine_architecture(&gpu_info.name, &gpu_info.vendor);
    }
}
