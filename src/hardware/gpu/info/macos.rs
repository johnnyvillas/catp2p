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

//! macOS-specific GPU information utilities.

use super::{determine_architecture, extract_vram_from_name, format_bytes, GpuInfo, GpuUsageInfo};
use crate::error::Error;
use std::process::Command;

/// Enhances the GPU info with macOS-specific information
pub fn enhance_gpu_info(gpu_info: &mut GpuInfo) {
    // Get all GPU info in a single system_profiler call for performance
    if let Some((vram_bytes, driver, additional_props)) = get_gpu_info_system_profiler() {
        if vram_bytes > 0 {
            gpu_info.vram = format_bytes(vram_bytes);
        }

        if !driver.is_empty() {
            gpu_info.driver = driver;
        }

        // macOS doesn't typically provide direct temperature readings for GPUs
        // but we can check if it's in the additional properties
        if let Some(temp_str) = gpu_info.additional_properties.get("Temperature") {
            if let Some(temp_value) = temp_str.split('°').next() {
                if let Ok(temp) = temp_value.parse::<f32>() {
                    gpu_info.temperature = Some(temp);
                }
            }
        }

        // Add additional properties
        for (key, value) in additional_props {
            gpu_info.additional_properties.insert(key, value);
        }
    }

    // If VRAM is still unknown, try to extract from name
    if gpu_info.vram.is_empty() || gpu_info.vram == "Unknown" {
        if let Some(vram) = extract_vram_from_name(&gpu_info.name) {
            gpu_info.vram = vram;
        }
    }

    // If architecture is still unknown, try to determine from name and vendor
    if gpu_info.architecture.is_empty() || gpu_info.architecture == "Unknown" {
        gpu_info.architecture = determine_architecture(&gpu_info.name, &gpu_info.vendor);
    }
}

/// Gets GPU usage information for the primary GPU
pub fn get_gpu_usage(usage_info: &mut super::GpuUsageInfo) {
    // macOS doesn't provide easy access to GPU memory usage through command line
    // We'll use a combination of system_profiler and top to get what we can

    // Get total VRAM from system_profiler
    if let Some((total_bytes, _, _)) = get_gpu_info_system_profiler() {
        usage_info.total_vram_bytes = total_bytes;
        usage_info.total_vram = super::format_bytes(total_bytes);

        // For Apple Silicon, estimate usage based on system memory pressure
        if usage_info.name.contains("Apple") {
            if let Some((used_bytes, gpu_util)) = get_apple_silicon_usage(total_bytes) {
                usage_info.used_vram_bytes = used_bytes;
                usage_info.used_vram = super::format_bytes(used_bytes);
                usage_info.gpu_usage_percent = gpu_util;
                return;
            }
        }

        // For discrete GPUs, try to estimate usage
        if let Some((used_bytes, gpu_util)) = get_discrete_gpu_usage(total_bytes) {
            usage_info.used_vram_bytes = used_bytes;
            usage_info.used_vram = super::format_bytes(used_bytes);
            usage_info.gpu_usage_percent = gpu_util;
            return;
        }

        // Fallback: estimate 50% usage
        let used_bytes = total_bytes / 2;
        usage_info.used_vram_bytes = used_bytes;
        usage_info.used_vram = super::format_bytes(used_bytes);
        usage_info.gpu_usage_percent = 0.0;
    }
}

/// Gets GPU usage information for a specific GPU by name
pub fn get_gpu_usage_by_name(usage_info: &mut super::GpuUsageInfo) {
    // Just use the same function for now, as we're identifying by name inside
    get_gpu_usage(usage_info)
}

/// Gets GPU information using system_profiler
fn get_gpu_info_system_profiler() -> Option<(u64, String, Vec<(String, String)>)> {
    // Use a single system_profiler call to get all the information we need
    let output = Command::new("system_profiler")
        .args(&["SPDisplaysDataType"])
        .output()
        .ok()?;

    if !output.status.success() {
        return None;
    }

    let stdout = String::from_utf8_lossy(&output.stdout).to_string();
    let mut vram_bytes = 0;
    let mut driver = String::new();
    let mut props = Vec::new();

    // Extract VRAM info
    if let Some(vram_pos) = stdout.find("VRAM") {
        let vram_str = &stdout[vram_pos..];

        // Extract the number and unit
        let num_str: String = vram_str
            .chars()
            .skip_while(|c| !c.is_digit(10))
            .take_while(|c| c.is_digit(10) || *c == '.')
            .collect();

        if let Ok(num) = num_str.parse::<f64>() {
            // Determine unit (GB or MB)
            if vram_str.contains("GB") {
                vram_bytes = (num * 1024.0 * 1024.0 * 1024.0) as u64;
                props.push(("Total VRAM".to_string(), format!("{:.1} GB", num)));
            } else if vram_str.contains("MB") {
                vram_bytes = (num * 1024.0 * 1024.0) as u64;
                let gb = num / 1024.0;
                props.push(("Total VRAM".to_string(), format!("{:.1} GB", gb)));
            }
        }
    }

    // Extract Metal version
    if let Some(metal_pos) = stdout.find("Metal:") {
        let metal_line = stdout[metal_pos..].lines().next().unwrap_or("");
        if metal_line.len() > 6 {
            let metal_version = metal_line[6..].trim().to_string();
            driver = format!("Metal {}", metal_version);
            props.push(("Metal Version".to_string(), metal_version));
        }
    }

    // Extract device ID and vendor
    for line in stdout.lines() {
        if line.contains("Vendor:") {
            if let Some(vendor_pos) = line.find("Vendor:") {
                let vendor = line[vendor_pos + 7..].trim().to_string();
                props.push(("Vendor ID".to_string(), vendor));
            }
        } else if line.contains("Device ID:") {
            if let Some(device_pos) = line.find("Device ID:") {
                let device = line[device_pos + 10..].trim().to_string();
                props.push(("Device ID".to_string(), device));
            }
        } else if line.contains("Type:") {
            if let Some(type_pos) = line.find("Type:") {
                let display_type = line[type_pos + 5..].trim().to_string();
                props.push(("Display Type".to_string(), display_type));
            }
        }
    }

    // If we're on Apple Silicon, try to estimate VRAM
    if vram_bytes == 0
        && (stdout.contains("Apple M1")
            || stdout.contains("Apple M2")
            || stdout.contains("Apple M3"))
    {
        // For Apple Silicon, get system memory and estimate GPU memory
        if let Some(output) = Command::new("sysctl")
            .args(&["-n", "hw.memsize"])
            .output()
            .ok()
            .filter(|output| output.status.success())
            .map(|output| String::from_utf8_lossy(&output.stdout).to_string())
        {
            if let Ok(system_bytes) = output.trim().parse::<u64>() {
                // Apple Silicon typically allocates up to 1/4 of system memory
                // to the GPU, but it's dynamic. We'll estimate conservatively.
                vram_bytes = system_bytes / 4;
                let gb = vram_bytes as f64 / (1024.0 * 1024.0 * 1024.0);
                props.push((
                    "Estimated VRAM".to_string(),
                    format!("~{:.1} GB (shared)", gb),
                ));
            }
        }
    }

    Some((vram_bytes, driver, props))
}

/// Gets Apple Silicon GPU usage information
fn get_apple_silicon_usage(total_bytes: u64) -> Option<(u64, f32)> {
    // For Apple Silicon, we'll estimate GPU memory usage based on system memory pressure
    // and GPU utilization based on CPU usage of GPU-related processes

    // Get memory pressure
    let output = Command::new("sh")
        .args(&["-c", "top -l 1 -n 0 | grep 'PhysMem'"])
        .output()
        .ok()?;

    if !output.status.success() {
        return None;
    }

    let stdout = String::from_utf8_lossy(&output.stdout).trim().to_string();

    // Parse memory usage
    let mut memory_pressure = 0.5; // Default to 50% if we can't determine

    if let Some(used_pos) = stdout.find("used:") {
        let used_str = &stdout[used_pos + 5..];
        if let Some(comma_pos) = used_str.find(',') {
            let used_part = &used_str[..comma_pos].trim();

            // Parse the value and unit
            let num_str: String = used_part
                .chars()
                .take_while(|c| c.is_digit(10) || *c == '.')
                .collect();

            if let Ok(num) = num_str.parse::<f64>() {
                // Determine unit (G or M)
                if used_part.contains('G') {
                    let used_gb = num;

                    // Get total memory
                    if let Some(total_pos) = stdout.find("wired:") {
                        if let Some(total_str) = stdout[total_pos..].find("used:") {
                            let total_part = &stdout[total_pos + total_str + 5..];
                            if let Some(comma_pos) = total_part.find(',') {
                                let total_val = &total_part[..comma_pos].trim();
                                let total_num_str: String = total_val
                                    .chars()
                                    .take_while(|c| c.is_digit(10) || *c == '.')
                                    .collect();

                                if let Ok(total_num) = total_num_str.parse::<f64>() {
                                    if total_part.contains('G') {
                                        let total_gb = total_num;
                                        memory_pressure = used_gb / total_gb;
                                    }
                                }
                            }
                        }
                    }
                }
            }
        }
    }

    // Estimate GPU memory usage based on memory pressure
    let used_bytes = (total_bytes as f64 * memory_pressure) as u64;

    // Get GPU utilization by looking at WindowServer process
    let output = Command::new("sh")
        .args(&[
            "-c",
            "top -l 1 -stats pid,command,cpu | grep -E 'WindowServer|MTLCompilerService'",
        ])
        .output()
        .ok()?;

    let stdout = String::from_utf8_lossy(&output.stdout).trim().to_string();

    // Parse CPU usage of GPU-related processes
    let mut gpu_util = 0.0;

    for line in stdout.lines() {
        let parts: Vec<&str> = line.split_whitespace().collect();
        if parts.len() >= 3 {
            if let Ok(cpu_usage) = parts[parts.len() - 1].parse::<f32>() {
                gpu_util += cpu_usage;
            }
        }
    }

    // Cap at 100%
    gpu_util = gpu_util.min(100.0);

    Some((used_bytes, gpu_util))
}

/// Gets discrete GPU usage information
fn get_discrete_gpu_usage(total_bytes: u64) -> Option<(u64, f32)> {
    // For discrete GPUs, we'll use a similar approach as for Apple Silicon
    // but look for different processes

    // Estimate GPU memory usage as 50% of total by default
    let used_bytes = total_bytes / 2;

    // Get GPU utilization by looking at GPU-related processes
    let output = Command::new("sh")
        .args(&[
            "-c",
            "top -l 1 -stats pid,command,cpu | grep -E 'AMDRadeonX|AppleGPU|GPUWorker'",
        ])
        .output()
        .ok()?;

    let stdout = String::from_utf8_lossy(&output.stdout).trim().to_string();

    // Parse CPU usage of GPU-related processes
    let mut gpu_util = 0.0;

    for line in stdout.lines() {
        let parts: Vec<&str> = line.split_whitespace().collect();
        if parts.len() >= 3 {
            if let Ok(cpu_usage) = parts[parts.len() - 1].parse::<f32>() {
                gpu_util += cpu_usage;
            }
        }
    }

    // Cap at 100%
    gpu_util = gpu_util.min(100.0);

    Some((used_bytes, gpu_util))
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

    // Try to get GPU info using system_profiler
    if let Some(output) = Command::new("system_profiler")
        .args(&["SPDisplaysDataType"])
        .output()
        .ok()
        .filter(|output| output.status.success())
        .map(|output| String::from_utf8_lossy(&output.stdout).to_string())
    {
        // Extract GPU name (Chipset Model)
        if let Some(chipset_pos) = output.find("Chipset Model:") {
            let chipset_line = output[chipset_pos..].lines().next().unwrap_or("");
            if chipset_line.len() > 14 {
                let name = chipset_line[14..].trim().to_string();
                gpu_info.name = name.clone();

                // Try to determine if it's Apple Silicon
                if name.contains("Apple") {
                    gpu_info.vendor = "Apple Inc.".to_string();
                    gpu_info.is_integrated = true;

                    // Determine architecture based on name
                    if name.contains("M1") {
                        gpu_info.architecture = "Apple Silicon (M1)".to_string();
                    } else if name.contains("M2") {
                        gpu_info.architecture = "Apple Silicon (M2)".to_string();
                    } else if name.contains("M3") {
                        gpu_info.architecture = "Apple Silicon (M3)".to_string();
                    } else {
                        gpu_info.architecture = "Apple Silicon".to_string();
                    }
                } else if name.contains("AMD") || name.contains("Radeon") {
                    gpu_info.vendor = "Advanced Micro Devices, Inc.".to_string();
                } else if name.contains("NVIDIA") || name.contains("GeForce") {
                    gpu_info.vendor = "NVIDIA Corporation".to_string();
                } else if name.contains("Intel") {
                    gpu_info.vendor = "Intel Corporation".to_string();
                    gpu_info.is_integrated = true;
                }
            }
        }
    }

    // If we still don't have a name, try a simpler approach
    if gpu_info.name == "Unknown GPU" {
        if let Some(output) = Command::new("sysctl")
            .args(&["-n", "machdep.cpu.brand_string"])
            .output()
            .ok()
            .filter(|output| output.status.success())
            .map(|output| String::from_utf8_lossy(&output.stdout).to_string())
        {
            // Check if it's Apple Silicon
            if output.contains("Apple") {
                gpu_info.name = "Apple Integrated GPU".to_string();
                gpu_info.vendor = "Apple Inc.".to_string();
                gpu_info.is_integrated = true;

                // Try to determine which Apple Silicon
                if output.contains("M1") {
                    gpu_info.name = "Apple M1 GPU".to_string();
                    gpu_info.architecture = "Apple Silicon (M1)".to_string();
                } else if output.contains("M2") {
                    gpu_info.name = "Apple M2 GPU".to_string();
                    gpu_info.architecture = "Apple Silicon (M2)".to_string();
                } else if output.contains("M3") {
                    gpu_info.name = "Apple M3 GPU".to_string();
                    gpu_info.architecture = "Apple Silicon (M3)".to_string();
                } else {
                    gpu_info.architecture = "Apple Silicon".to_string();
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
