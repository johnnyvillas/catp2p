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

//! Linux-specific GPU information utilities.

use super::{determine_architecture, extract_vram_from_name, format_bytes, GpuInfo, GpuUsageInfo};
use crate::error::Error;
use std::process::Command;

/// Gets current GPU temperature in Celsius
pub fn get_temperature() -> Option<f32> {
    // Try using nvidia-smi for NVIDIA GPUs
    if let Ok(output) = Command::new("nvidia-smi")
        .args(&["--query-gpu=temperature.gpu", "--format=csv,noheader,nounits"])
        .output()
    {
        if output.status.success() {
            let stdout = String::from_utf8_lossy(&output.stdout).trim().to_string();
            return stdout.parse::<f32>().ok();
        }
    }
    
    // For AMD GPUs
    if let Ok(output) = Command::new("sh")
        .args(&["-c", "find /sys/class/drm/card*/device/hwmon/hwmon*/temp1_input -type f 2>/dev/null | xargs cat 2>/dev/null"])
        .output()
    {
        if output.status.success() {
            let stdout = String::from_utf8_lossy(&output.stdout).trim().to_string();
            if let Ok(temp_millicelsius) = stdout.parse::<u32>() {
                return Some(temp_millicelsius as f32 / 1000.0);
            }
        }
    }
    
    None
}

/// Enhances the GPU info with Linux-specific information
pub fn enhance_gpu_info(gpu_info: &mut GpuInfo) {
    // For NVIDIA GPUs, use a single nvidia-smi call to get all info
    if gpu_info.name.contains("NVIDIA") || gpu_info.vendor.contains("NVIDIA") {
        if let Some((vram_bytes, driver, nvidia_props)) = get_nvidia_info() {
            if vram_bytes > 0 {
                gpu_info.vram = format_bytes(vram_bytes);
            }

            if !driver.is_empty() {
                gpu_info.driver = driver;
            }

            // Get temperature information
            if let Some(temp) = get_temperature() {
                gpu_info.temperature = Some(temp);
            }

            // Add additional properties
            for (key, value) in nvidia_props {
                gpu_info.additional_properties.insert(key, value);
            }
        }
    }

    // For AMD GPUs, try to get info from sysfs
    if gpu_info.name.contains("AMD") || gpu_info.vendor.contains("AMD") {
        if let Some((vram_bytes, driver, amd_props)) = get_amd_info() {
            if vram_bytes > 0 {
                gpu_info.vram = format_bytes(vram_bytes);
            }

            if !driver.is_empty() {
                gpu_info.driver = driver;
            }

            // Add additional properties
            for (key, value) in amd_props {
                gpu_info.additional_properties.insert(key, value);
            }
        }
    }

    // Generic fallback for any GPU
    if gpu_info.vram.is_empty() || gpu_info.vram == "Unknown" {
        if let Some(vram_bytes) = get_vram_bytes() {
            gpu_info.vram = format_bytes(vram_bytes);
        } else if let Some(vram) = extract_vram_from_name(&gpu_info.name) {
            gpu_info.vram = vram;
        }
    }

    if gpu_info.driver.is_empty() || gpu_info.driver == "Unknown" {
        if let Some(driver) = get_driver_version() {
            gpu_info.driver = driver;
        }
    }

    // Add kernel driver info
    if let Some(output) = Command::new("sh")
        .args(&[
            "-c",
            "lspci -v | grep -i vga -A 10 | grep 'Kernel driver in use'",
        ])
        .output()
        .ok()
        .filter(|output| output.status.success())
        .map(|output| String::from_utf8_lossy(&output.stdout).to_string())
    {
        if let Some(driver_pos) = output.find("Kernel driver in use:") {
            let driver = output[driver_pos + 21..].trim().to_string();
            gpu_info
                .additional_properties
                .insert("Kernel Driver".to_string(), driver);
        }
    }

    // If architecture is still unknown, try to determine from name and vendor
    if gpu_info.architecture.is_empty() || gpu_info.architecture == "Unknown" {
        gpu_info.architecture = determine_architecture(&gpu_info.name, &gpu_info.vendor);
    }
}

/// Gets GPU usage information for the primary GPU
pub fn get_gpu_usage(usage_info: &mut super::GpuUsageInfo) {
    // Try nvidia-smi for NVIDIA GPUs first (most accurate)
    if usage_info.name.contains("NVIDIA")
        || usage_info.name.contains("GeForce")
        || usage_info.name.contains("Quadro")
    {
        if let Some((total_bytes, used_bytes, gpu_util)) = get_nvidia_usage() {
            usage_info.total_vram_bytes = total_bytes;
            usage_info.total_vram = super::format_bytes(total_bytes);

            usage_info.used_vram_bytes = used_bytes;
            usage_info.used_vram = super::format_bytes(used_bytes);

            usage_info.gpu_usage_percent = gpu_util;

            return;
        }
    }

    // Try AMD GPUs using sysfs
    if usage_info.name.contains("AMD") || usage_info.name.contains("Radeon") {
        if let Some((total_bytes, used_bytes, gpu_util)) = get_amd_usage() {
            usage_info.total_vram_bytes = total_bytes;
            usage_info.total_vram = super::format_bytes(total_bytes);

            usage_info.used_vram_bytes = used_bytes;
            usage_info.used_vram = super::format_bytes(used_bytes);

            usage_info.gpu_usage_percent = gpu_util;

            return;
        }
    }

    // Generic fallback for any GPU using Vulkan
    if let Some((total_bytes, used_bytes)) = get_vulkan_memory() {
        usage_info.total_vram_bytes = total_bytes;
        usage_info.total_vram = super::format_bytes(total_bytes);

        usage_info.used_vram_bytes = used_bytes;
        usage_info.used_vram = super::format_bytes(used_bytes);

        // Try to get GPU utilization from top
        if let Some(gpu_util) = get_gpu_utilization_top() {
            usage_info.gpu_usage_percent = gpu_util;
        }
    }
}

/// Gets GPU usage information for a specific GPU by name
pub fn get_gpu_usage_by_name(usage_info: &mut super::GpuUsageInfo) {
    // Just use the same function for now, as we're identifying by name inside
    get_gpu_usage(usage_info)
}

/// Gets NVIDIA GPU information using a single nvidia-smi call
fn get_nvidia_info() -> Option<(u64, String, Vec<(String, String)>)> {
    // Use a single nvidia-smi call to get multiple properties
    let nvidia_query = "--query-gpu=driver_version,memory.total,memory.free,temperature.gpu,utilization.gpu,utilization.memory,pcie.link.gen.current,pcie.link.width.current --format=csv,noheader";

    let output = Command::new("nvidia-smi")
        .args(&[nvidia_query])
        .output()
        .ok()?;

    if !output.status.success() {
        return None;
    }

    let stdout = String::from_utf8_lossy(&output.stdout).to_string();
    let parts: Vec<&str> = stdout.split(',').collect();

    if parts.len() < 8 {
        return None;
    }

    let mut props = Vec::new();
    let driver = parts[0].trim().to_string();
    let mut vram_bytes = 0;

    // Memory total
    if let Ok(mb) = parts[1].trim().parse::<u64>() {
        vram_bytes = mb * 1024 * 1024; // Convert MB to bytes
        let gb = mb as f64 / 1024.0;
        props.push(("Total VRAM".to_string(), format!("{:.1} GB", gb)));
    }

    // Memory free
    if let Ok(mb) = parts[2].trim().parse::<u64>() {
        let gb = mb as f64 / 1024.0;
        props.push(("Free VRAM".to_string(), format!("{:.1} GB", gb)));
    }

    // Temperature
    if parts[3].trim() != "N/A" {
        props.push(("Temperature".to_string(), format!("{}°C", parts[3].trim())));
    }

    // GPU utilization
    if parts[4].trim() != "N/A" {
        props.push((
            "GPU Utilization".to_string(),
            format!("{}%", parts[4].trim()),
        ));
    }

    // Memory utilization
    if parts[5].trim() != "N/A" {
        props.push((
            "Memory Utilization".to_string(),
            format!("{}%", parts[5].trim()),
        ));
    }

    // PCIe version
    if parts[6].trim() != "N/A" {
        props.push((
            "PCIe Version".to_string(),
            format!("Gen {}", parts[6].trim()),
        ));
    }

    // PCIe width
    if parts[7].trim() != "N/A" {
        props.push(("PCIe Width".to_string(), format!("x{}", parts[7].trim())));
    }

    Some((vram_bytes, driver, props))
}

/// Gets NVIDIA GPU usage information using nvidia-smi
fn get_nvidia_usage() -> Option<(u64, u64, f32)> {
    // Use nvidia-smi to get memory and utilization information
    let output = Command::new("nvidia-smi")
        .args(&[
            "--query-gpu=memory.total,memory.used,utilization.gpu",
            "--format=csv,noheader,nounits",
        ])
        .output()
        .ok()?;

    if !output.status.success() {
        return None;
    }

    let stdout = String::from_utf8_lossy(&output.stdout).trim().to_string();
    let parts: Vec<&str> = stdout.split(',').collect();

    if parts.len() < 3 {
        return None;
    }

    // Parse memory values (in MiB)
    let total_mb = parts[0].trim().parse::<u64>().ok()?;
    let used_mb = parts[1].trim().parse::<u64>().ok()?;

    // Convert to bytes
    let total_bytes = total_mb * 1024 * 1024;
    let used_bytes = used_mb * 1024 * 1024;

    // Parse utilization values
    let gpu_util = parts[2].trim().parse::<f32>().ok()?;

    Some((total_bytes, used_bytes, gpu_util))
}

/// Gets AMD GPU information from sysfs
fn get_amd_info() -> Option<(u64, String, Vec<(String, String)>)> {
    let mut vram_bytes = 0;
    let mut driver = String::new();
    let mut props = Vec::new();

    // Get VRAM total
    if let Some(output) = Command::new("sh")
        .args(&["-c", "find /sys/class/drm/card*/device/mem_info_vram_total -type f 2>/dev/null | xargs cat 2>/dev/null"])
        .output()
        .ok()
        .filter(|output| output.status.success())
        .map(|output| String::from_utf8_lossy(&output.stdout).to_string())
    {
        if let Ok(bytes) = output.trim().parse::<u64>() {
            vram_bytes = bytes;
            let gb = bytes as f64 / (1024.0 * 1024.0 * 1024.0);
            props.push(("Total VRAM".to_string(), format!("{:.1} GB", gb)));
        }
    }

    // Get VRAM used
    if let Some(output) = Command::new("sh")
        .args(&["-c", "find /sys/class/drm/card*/device/mem_info_vram_used -type f 2>/dev/null | xargs cat 2>/dev/null"])
        .output()
        .ok()
        .filter(|output| output.status.success())
        .map(|output| String::from_utf8_lossy(&output.stdout).to_string())
    {
        if let Ok(used_bytes) = output.trim().parse::<u64>() {
            let free_bytes = vram_bytes.saturating_sub(used_bytes);
            let free_gb = free_bytes as f64 / (1024.0 * 1024.0 * 1024.0);
            props.push(("Free VRAM".to_string(), format!("{:.1} GB", free_gb)));
        }
    }

    // Get driver info
    if let Some(output) = Command::new("sh")
        .args(&[
            "-c",
            "lspci -v | grep -i vga -A 10 | grep 'Kernel driver in use'",
        ])
        .output()
        .ok()
        .filter(|output| output.status.success())
        .map(|output| String::from_utf8_lossy(&output.stdout).to_string())
    {
        if let Some(driver_pos) = output.find("Kernel driver in use:") {
            driver = output[driver_pos + 21..].trim().to_string();
            props.push(("Kernel Driver".to_string(), driver.clone()));
        }
    }

    // Get GPU temperature
    if let Some(output) = Command::new("sh")
        .args(&["-c", "find /sys/class/drm/card*/device/hwmon/hwmon*/temp1_input -type f 2>/dev/null | xargs cat 2>/dev/null"])
        .output()
        .ok()
        .filter(|output| output.status.success())
        .map(|output| String::from_utf8_lossy(&output.stdout).to_string())
    {
        if let Ok(temp_millicelsius) = output.trim().parse::<u32>() {
            let temp_celsius = temp_millicelsius as f32 / 1000.0;
            props.push(("Temperature".to_string(), format!("{:.1}°C", temp_celsius)));
        }
    }

    // Get GPU utilization
    if let Some(output) = Command::new("sh")
        .args(&["-c", "find /sys/class/drm/card*/device/gpu_busy_percent -type f 2>/dev/null | xargs cat 2>/dev/null"])
        .output()
        .ok()
        .filter(|output| output.status.success())
        .map(|output| String::from_utf8_lossy(&output.stdout).to_string())
    {
        if let Ok(util) = output.trim().parse::<u32>() {
            props.push(("GPU Utilization".to_string(), format!("{}%", util)));
        }
    }

    if vram_bytes > 0 || !driver.is_empty() || !props.is_empty() {
        Some((vram_bytes, driver, props))
    } else {
        None
    }
}

/// Gets AMD GPU usage information from sysfs
fn get_amd_usage() -> Option<(u64, u64, f32)> {
    let mut total_bytes = 0;
    let mut used_bytes = 0;
    let mut gpu_util = 0.0;

    // Get VRAM total
    if let Some(output) = Command::new("sh")
        .args(&["-c", "find /sys/class/drm/card*/device/mem_info_vram_total -type f 2>/dev/null | xargs cat 2>/dev/null"])
        .output()
        .ok()
        .filter(|output| output.status.success())
        .map(|output| String::from_utf8_lossy(&output.stdout).to_string())
    {
        if let Ok(bytes) = output.trim().parse::<u64>() {
            total_bytes = bytes;
        }
    }

    // Get VRAM used
    if let Some(output) = Command::new("sh")
        .args(&["-c", "find /sys/class/drm/card*/device/mem_info_vram_used -type f 2>/dev/null | xargs cat 2>/dev/null"])
        .output()
        .ok()
        .filter(|output| output.status.success())
        .map(|output| String::from_utf8_lossy(&output.stdout).to_string())
    {
        if let Ok(bytes) = output.trim().parse::<u64>() {
            used_bytes = bytes;
        }
    }

    // Get GPU utilization
    if let Some(output) = Command::new("sh")
        .args(&["-c", "find /sys/class/drm/card*/device/gpu_busy_percent -type f 2>/dev/null | xargs cat 2>/dev/null"])
        .output()
        .ok()
        .filter(|output| output.status.success())
        .map(|output| String::from_utf8_lossy(&output.stdout).to_string())
    {
        if let Ok(util) = output.trim().parse::<f32>() {
            gpu_util = util;
        }
    }

    if total_bytes > 0 {
        Some((total_bytes, used_bytes, gpu_util))
    } else {
        None
    }
}

/// Gets GPU memory information using Vulkan
fn get_vulkan_memory() -> Option<(u64, u64)> {
    // This is a simplified approach - in a real implementation, we would use the Vulkan API directly
    // For now, we'll use a simple approach with vulkaninfo
    let output = Command::new("sh")
        .args(&[
            "-c",
            "vulkaninfo --summary 2>/dev/null | grep -A 5 'VkPhysicalDeviceMemoryProperties'",
        ])
        .output()
        .ok()?;

    if !output.status.success() {
        return None;
    }

    let stdout = String::from_utf8_lossy(&output.stdout).to_string();

    // Parse the output to find heap size
    let mut total_bytes = 0;

    for line in stdout.lines() {
        if line.contains("heapSize") {
            let parts: Vec<&str> = line.split('=').collect();
            if parts.len() >= 2 {
                let size_str = parts[1].trim();
                if let Ok(size) = size_str.parse::<u64>() {
                    total_bytes = size;
                    break;
                }
            }
        }
    }

    // We don't have a direct way to get used memory from vulkaninfo
    // Estimate 50% usage as a fallback
    let used_bytes = total_bytes / 2;

    if total_bytes > 0 {
        Some((total_bytes, used_bytes))
    } else {
        None
    }
}

/// Gets GPU utilization using top
fn get_gpu_utilization_top() -> Option<f32> {
    // Try to get GPU utilization from top
    let output = Command::new("sh")
        .args(&["-c", "top -bn1 | grep '%Cpu' | awk '{print $10}'"])
        .output()
        .ok()?;

    if !output.status.success() {
        return None;
    }

    let stdout = String::from_utf8_lossy(&output.stdout).trim().to_string();

    // Parse the idle percentage and convert to usage percentage
    if let Ok(idle) = stdout.parse::<f32>() {
        return Some(100.0 - idle);
    }

    None
}

/// Gets VRAM information
fn get_vram_bytes() -> Option<u64> {
    // Try using lspci as a fallback
    if let Some(output) = Command::new("sh")
        .args(&[
            "-c",
            "lspci -v | grep -A 12 VGA | grep 'Memory.*size' | head -n 1",
        ])
        .output()
        .ok()
        .filter(|output| output.status.success())
        .map(|output| String::from_utf8_lossy(&output.stdout).to_string())
    {
        // Parse output like "Memory at ... (64-bit, prefetchable) [size=4G]"
        if let Some(size_pos) = output.find("[size=") {
            let size_str = &output[size_pos + 6..];
            if let Some(end_pos) = size_str.find(']') {
                let size = &size_str[..end_pos];

                // Convert to standardized format and bytes
                if size.ends_with('G') || size.ends_with("GB") {
                    let num_str: String = size
                        .chars()
                        .filter(|c| c.is_digit(10) || *c == '.')
                        .collect();
                    if let Ok(num) = num_str.parse::<f64>() {
                        return Some((num * 1024.0 * 1024.0 * 1024.0) as u64);
                    }
                } else if size.ends_with('M') || size.ends_with("MB") {
                    let num_str: String = size
                        .chars()
                        .filter(|c| c.is_digit(10) || *c == '.')
                        .collect();
                    if let Ok(num) = num_str.parse::<f64>() {
                        return Some((num * 1024.0 * 1024.0) as u64);
                    }
                }
            }
        }
    }

    None
}

/// Gets the GPU driver version
fn get_driver_version() -> Option<String> {
    // Try using glxinfo for other GPUs
    if let Some(output) = Command::new("sh")
        .args(&["-c", "glxinfo | grep 'OpenGL version'"])
        .output()
        .ok()
        .filter(|output| output.status.success())
        .map(|output| String::from_utf8_lossy(&output.stdout).to_string())
    {
        if let Some(version_pos) = output.find("OpenGL version string:") {
            return Some(output[version_pos..].trim().to_string());
        }
    }

    None
}

/// Gets fallback GPU information when the main method fails
pub fn get_fallback_gpu_info() -> Result<GpuInfo, Error> {
    let mut gpu_info = GpuInfo {
        name: "Unknown GPU".to_string(),
        vendor: "Unknown Vendor".to_string(),
        driver: "Unknown".to_string(),
        vram: "Unknown".to_string(),
        architecture: "Unknown".to_string(),
        is_integrated: false,
        ..Default::default()
    };

    // Try to get basic GPU info using lspci
    if let Some(output) = Command::new("sh")
        .args(&["-c", "lspci -nn | grep -i vga"])
        .output()
        .ok()
        .filter(|output| output.status.success())
        .map(|output| String::from_utf8_lossy(&output.stdout).to_string())
    {
        // Extract GPU name
        if !output.is_empty() {
            // Format is typically: "01:00.0 VGA compatible controller: NVIDIA Corporation ... [10de:2204]"
            if let Some(controller_pos) = output.find("VGA compatible controller:") {
                let name_part = &output[controller_pos + 25..];

                // Extract name up to the PCI ID
                let name = if let Some(id_pos) = name_part.find('[') {
                    name_part[..id_pos].trim().to_string()
                } else {
                    name_part.trim().to_string()
                };

                gpu_info.name = name.clone();

                // Try to determine vendor from name
                if name.contains("NVIDIA") {
                    gpu_info.vendor = "NVIDIA Corporation".to_string();
                } else if name.contains("AMD")
                    || name.contains("ATI")
                    || name.contains("Advanced Micro Devices")
                {
                    gpu_info.vendor = "Advanced Micro Devices, Inc.".to_string();
                } else if name.contains("Intel") {
                    gpu_info.vendor = "Intel Corporation".to_string();

                    // Intel GPUs are typically integrated
                    if name.contains("HD Graphics")
                        || name.contains("UHD Graphics")
                        || name.contains("Iris")
                    {
                        gpu_info.is_integrated = true;
                    }
                }

                // Try to extract device ID
                if let Some(id_start) = output.find('[') {
                    if let Some(id_end) = output[id_start..].find(']') {
                        let device_id = &output[id_start..id_start + id_end + 1];
                        gpu_info
                            .additional_properties
                            .insert("Device ID".to_string(), device_id.to_string());
                    }
                }
            }
        }
    }

    // Enhance with additional info
    enhance_gpu_info(&mut gpu_info);

    // If architecture is still unknown, try to determine from name and vendor
    if gpu_info.architecture == "Unknown" {
        gpu_info.architecture = determine_architecture(&gpu_info.name, &gpu_info.vendor);
    }

    Ok(gpu_info)
}
