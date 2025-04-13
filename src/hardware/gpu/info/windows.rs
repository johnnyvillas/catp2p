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

//! Windows-specific GPU information utilities.

use std::process::Command;
use super::{GpuInfo, format_bytes, determine_architecture};
use crate::error::Error;

/// Gets the GPU driver version on Windows
pub fn get_driver_version() -> String {
    // Try using WMI to get driver version
    if let Ok(output) = Command::new("powershell")
        .args(&["-Command", "Get-WmiObject Win32_VideoController | Select-Object DriverVersion | ConvertTo-Json"])
        .output()
    {
        if output.status.success() {
            let stdout = String::from_utf8_lossy(&output.stdout);
            if let Ok(json) = serde_json::from_str::<serde_json::Value>(&stdout) {
                if json.is_array() {
                    if let Some(first) = json.as_array().unwrap().first() {
                        if let Some(version) = first.get("DriverVersion") {
                            if let Some(version_str) = version.as_str() {
                                return version_str.to_string();
                            }
                        }
                    }
                } else if let Some(version) = json.get("DriverVersion") {
                    if let Some(version_str) = version.as_str() {
                        return version_str.to_string();
                    }
                }
            }
        }
    }
    
    "Unknown".to_string()
}

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
        
        None
    }

/// Enhances the GPU info with Windows-specific information
pub fn enhance_gpu_info(gpu_info: &mut GpuInfo) {
    // Get basic info from WMI (but don't override VRAM if already set by common)
    if let Some((vram_bytes, driver, additional_props)) = get_gpu_info_wmi() {
        // Only set VRAM if not already set by common
        if (gpu_info.vram.is_empty() || gpu_info.vram == "Unknown") && vram_bytes > 0 {
            gpu_info.vram = format_bytes(vram_bytes);
        }
        
        // Only set driver if not already set by common
        if (gpu_info.driver.is_empty() || gpu_info.driver == "Unknown") && !driver.is_empty() {
            gpu_info.driver = driver;
        }
        
        // Get temperature information
        if let Some(temp) = get_temperature() {
            gpu_info.temperature = Some(temp);
        }
        
        // Add additional properties
        for (key, value) in additional_props {
            gpu_info.additional_properties.insert(key, value);
        }

    }
    
    // Add NVIDIA-specific properties for NVIDIA GPUs
    if gpu_info.name.contains("NVIDIA") || gpu_info.vendor.contains("NVIDIA") {
        if let Some(nvidia_props) = get_nvidia_info() {
            for (key, value) in nvidia_props {
                // Don't override VRAM if already set
                if key != "Total VRAM" || gpu_info.vram.is_empty() || gpu_info.vram == "Unknown" {
                    gpu_info.additional_properties.insert(key, value);
                }
            }
        }
    }
}

/// Gets GPU usage information for the primary GPU
pub fn get_gpu_usage(usage_info: &mut super::GpuUsageInfo) {
    // Try nvidia-smi for NVIDIA GPUs first (most accurate)
    if usage_info.name.contains("NVIDIA") || usage_info.vendor.contains("NVIDIA") {
        if let Some((total_bytes, used_bytes, gpu_util)) = get_nvidia_usage() {
            usage_info.total_vram_bytes = total_bytes;
            usage_info.total_vram = format_bytes(total_bytes);
            
            usage_info.used_vram_bytes = used_bytes;
            usage_info.used_vram = format_bytes(used_bytes);
            
            usage_info.gpu_usage_percent = gpu_util;
            
            return;
        }
    }


    
    // Try using DXGI for other GPUs
    if let Some((total_bytes, used_bytes)) = get_dxgi_gpu_memory() {
        usage_info.total_vram_bytes = total_bytes;
        usage_info.total_vram = format_bytes(total_bytes);
        
        usage_info.used_vram_bytes = used_bytes;
        usage_info.used_vram = format_bytes(used_bytes);
        
        // We don't have GPU utilization from DXGI, so try to get it from WMI
        if let Some(gpu_util) = get_gpu_utilization_wmi() {
            usage_info.gpu_usage_percent = gpu_util;
        }
    }
}

/// Gets GPU usage information for a specific GPU by name
pub fn get_gpu_usage_by_name(usage_info: &mut super::GpuUsageInfo) {
    // Just use the same function for now, as we're identifying by name inside
    get_gpu_usage(usage_info)
}

/// Gets GPU information using a single WMI query
fn get_gpu_info_wmi() -> Option<(u64, String, Vec<(String, String)>)> {
    // Use a single WMI query to get all the information we need
    let wmi_query = "Get-WmiObject Win32_VideoController | Select-Object Name, AdapterRAM, DriverVersion, VideoProcessor, VideoModeDescription, DriverDate, CurrentHorizontalResolution, CurrentVerticalResolution | ConvertTo-Json";
    
    let output = Command::new("powershell")
        .args(&["-Command", wmi_query])
        .output()
        .ok()?;
    
    if !output.status.success() {
        return None;
    }
    
    let stdout = String::from_utf8_lossy(&output.stdout).to_string();
    let json: serde_json::Value = serde_json::from_str(&stdout).ok()?;
    
    // Handle both single GPU and multiple GPU cases
    let gpu_json = if json.is_array() {
        // Multiple GPUs, use the first one with the most VRAM
        let mut max_vram = 0;
        let mut max_vram_index = 0;
        
        for (i, gpu) in json.as_array()?.iter().enumerate() {
            if let Some(adapter_ram) = gpu.get("AdapterRAM").and_then(|v| v.as_u64()) {
                if adapter_ram > max_vram {
                    max_vram = adapter_ram;
                    max_vram_index = i;
                }
            }
        }
        
        json.as_array()?.get(max_vram_index)
    } else {
        // Single GPU
        Some(&json)
    };
    
    let gpu = gpu_json?;
    
    // Extract VRAM
    let vram_bytes = gpu.get("AdapterRAM")
        .and_then(|v| v.as_u64())
        .unwrap_or(0);
    
    // Extract driver version
    let driver = gpu.get("DriverVersion")
        .and_then(|v| v.as_str())
        .unwrap_or("")
        .to_string();
    
    // Extract additional properties
    let mut additional_props = Vec::new();
    
    if let Some(processor) = gpu.get("VideoProcessor").and_then(|v| v.as_str()) {
        additional_props.push(("Video Processor".to_string(), processor.to_string()));
    }
    
    if let Some(mode) = gpu.get("VideoModeDescription").and_then(|v| v.as_str()) {
        additional_props.push(("Video Mode".to_string(), mode.to_string()));
    }
    
    if let Some(date) = gpu.get("DriverDate").and_then(|v| v.as_str()) {
        additional_props.push(("Driver Date".to_string(), date.to_string()));
    }
    
    // Get current resolution
    if let (Some(width), Some(height)) = (
        gpu.get("CurrentHorizontalResolution").and_then(|v| v.as_u64()),
        gpu.get("CurrentVerticalResolution").and_then(|v| v.as_u64())
    ) {
        additional_props.push(("Resolution".to_string(), format!("{}x{}", width, height)));
    }
    
    Some((vram_bytes, driver, additional_props))
}

/// Gets NVIDIA GPU information using a single nvidia-smi call
fn get_nvidia_info() -> Option<Vec<(String, String)>> {
    // Use a single nvidia-smi call to get multiple properties
    let nvidia_query = "--query-gpu=driver_version,memory.total,memory.free,temperature.gpu,utilization.gpu,utilization.memory --format=csv,noheader";
    
    let output = Command::new("nvidia-smi")
        .args(&[nvidia_query])
        .output()
        .ok()?;
    
    if !output.status.success() {
        return None;
    }
    
    let stdout = String::from_utf8_lossy(&output.stdout).to_string();
    let parts: Vec<&str> = stdout.split(',').collect();
    
    if parts.len() < 6 {
        return None;
    }
    
    let mut props = Vec::new();
    
    // Driver version
    props.push(("NVIDIA Driver".to_string(), parts[0].trim().to_string()));
    
    // Memory total
    if let Ok(mb) = parts[1].trim().parse::<u64>() {
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
        props.push(("GPU Utilization".to_string(), format!("{}%", parts[4].trim())));
    }
    
    // Memory utilization
    if parts[5].trim() != "N/A" {
        props.push(("Memory Utilization".to_string(), format!("{}%", parts[5].trim())));
    }
    
    Some(props)
}

/// Gets NVIDIA GPU usage information using nvidia-smi
fn get_nvidia_usage() -> Option<(u64, u64, f32)> {
    // Use nvidia-smi to get memory and utilization information
    let output = Command::new("nvidia-smi")
        .args(&["--query-gpu=memory.total,memory.used,utilization.gpu", "--format=csv,noheader,nounits"])
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


/// Gets GPU memory information using DXGI
fn get_dxgi_gpu_memory() -> Option<(u64, u64)> {
    // This requires using the Windows API directly with DXGI
    // For simplicity, we'll use a PowerShell script that leverages DXGI
    let ps_script = r#"
    Add-Type -TypeDefinition @"
    using System;
    using System.Runtime.InteropServices;
    
    public class DXGIInfo {
        [DllImport("dxgi.dll")]
        public static extern int CreateDXGIFactory1(ref Guid guid, out IntPtr factory);
        
        [ComImport, Guid("770aae78-f26f-4dba-a829-253c83d1b387"), InterfaceType(ComInterfaceType.InterfaceIsIUnknown)]
        public interface IDXGIFactory1 {
            void CreateSwapChain();
            void CreateSoftwareAdapter();
            int EnumAdapters1(uint index, out IntPtr adapter);
            bool IsCurrent();
        }
        
        [ComImport, Guid("00cddea8-939b-4b83-a340-a685226666cc"), InterfaceType(ComInterfaceType.InterfaceIsIUnknown)]
        public interface IDXGIAdapter3 {
            void GetDesc();
            void GetDesc1();
            void GetDesc2();
            void RegisterHardwareContentProtectionTeardownStatusEvent();
            void RegisterVideoMemoryBudgetChangeNotificationEvent();
            void SetVideoMemoryReservation();
            void UnregisterHardwareContentProtectionTeardownStatus();
            void UnregisterVideoMemoryBudgetChangeNotification();
            int QueryVideoMemoryInfo(uint NodeIndex, int MemorySegmentGroup, out DXGI_QUERY_VIDEO_MEMORY_INFO pVideoMemoryInfo);
        }
        
        [StructLayout(LayoutKind.Sequential)]
        public struct DXGI_QUERY_VIDEO_MEMORY_INFO {
            public ulong Budget;
            public ulong CurrentUsage;
            public ulong AvailableForReservation;
            public ulong CurrentReservation;
        }
    }
"@
    
    try {
        $factory = [IntPtr]::Zero
        $factoryGuid = [Guid]"770aae78-f26f-4dba-a829-253c83d1b387"
        $factoryInterfaceGuid = [Guid]"7b7166ec-21c7-44ae-b21a-c9ae321ae369"
        $adapter3Guid = [Guid]"00cddea8-939b-4b83-a340-a685226666cc"
        
        $hr = [DXGIInfo]::CreateDXGIFactory1([ref]$factoryInterfaceGuid, [ref]$factory)
        if ($hr -ge 0) {
            $factoryObj = [System.Runtime.InteropServices.Marshal]::GetObjectForIUnknown($factory)
            $factory1 = [System.Runtime.InteropServices.Marshal]::QueryInterface($factoryObj, [ref]$factoryGuid)
            
            $adapter = [IntPtr]::Zero
            $index = 0
            
            $hr = $factoryObj.GetType().GetMethod("EnumAdapters1").Invoke($factoryObj, @($index, [ref]$adapter))
            if ($hr -ge 0) {
                $adapter3 = [System.Runtime.InteropServices.Marshal]::QueryInterface([System.Runtime.InteropServices.Marshal]::GetObjectForIUnknown($adapter), [ref]$adapter3Guid)
                
                $memInfo = New-Object DXGIInfo+DXGI_QUERY_VIDEO_MEMORY_INFO
                $hr = $adapter3.GetType().GetMethod("QueryVideoMemoryInfo").Invoke($adapter3, @(0, 0, [ref]$memInfo))
                
                if ($hr -ge 0) {
                    Write-Output "$($memInfo.Budget)|$($memInfo.CurrentUsage)"
                }
                
                [System.Runtime.InteropServices.Marshal]::ReleaseComObject($adapter3) | Out-Null
                [System.Runtime.InteropServices.Marshal]::ReleaseComObject([System.Runtime.InteropServices.Marshal]::GetObjectForIUnknown($adapter)) | Out-Null
            }
            
            [System.Runtime.InteropServices.Marshal]::ReleaseComObject($factoryObj) | Out-Null
        }
    } catch {
        # Silently fail
    }
    "#;
    
    let output = Command::new("powershell")
        .args(&["-Command", ps_script])
        .output()
        .ok()?;
    
    if !output.status.success() {
        return None;
    }
    
    let stdout = String::from_utf8_lossy(&output.stdout).trim().to_string();
    
    // Parse the output (format: "Budget|CurrentUsage")
    let parts: Vec<&str> = stdout.split('|').collect();
    if parts.len() == 2 {
        if let (Ok(total), Ok(used)) = (parts[0].parse::<u64>(), parts[1].parse::<u64>()) {
            return Some((total, used));
        }
    }
    
    None
}

/// Gets GPU utilization using WMI
fn get_gpu_utilization_wmi() -> Option<f32> {
    // Use WMI to get GPU utilization
    let wmi_query = "Get-WmiObject Win32_PerfFormattedData_GPUPerformanceCounters_GPUEngine | Where-Object { $_.Name -like '*3D*' } | Measure-Object -Property UtilizationPercentage -Average | Select-Object -ExpandProperty Average";
    
    let output = Command::new("powershell")
        .args(&["-Command", wmi_query])
        .output()
        .ok()?;
    
    if !output.status.success() {
        return None;
    }
    
    let stdout = String::from_utf8_lossy(&output.stdout).trim().to_string();
    stdout.parse::<f32>().ok()
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
    
    // Try to get basic GPU info using WMI (single query)
    if let Some(output) = Command::new("powershell")
        .args(&["-Command", "Get-WmiObject Win32_VideoController | Select-Object Name, AdapterRAM, DriverVersion | ConvertTo-Json"])
        .output()
        .ok()
        .filter(|output| output.status.success())
        .map(|output| String::from_utf8_lossy(&output.stdout).to_string())
    {
        if let Ok(json) = serde_json::from_str::<serde_json::Value>(&output) {
            // Handle both single GPU and multiple GPU cases
            let gpu_json = if json.is_array() {
                // Multiple GPUs, use the first one
                json.as_array().unwrap().first()
            } else {
                // Single GPU
                Some(&json)
            };
            
            if let Some(gpu) = gpu_json {
                // Get GPU name
                if let Some(name) = gpu.get("Name").and_then(|v| v.as_str()) {
                    gpu_info.name = name.to_string();
                    
                    // Try to determine vendor from name
                    if name.contains("NVIDIA") || name.contains("GeForce") || name.contains("Quadro") {
                        gpu_info.vendor = "NVIDIA Corporation".to_string();
                    } else if name.contains("AMD") || name.contains("Radeon") || name.contains("FirePro") {
                        gpu_info.vendor = "Advanced Micro Devices, Inc.".to_string();
                    } else if name.contains("Intel") || name.contains("HD Graphics") || name.contains("UHD Graphics") || name.contains("Iris") {
                        gpu_info.vendor = "Intel Corporation".to_string();
                        gpu_info.is_integrated = true;
                    }
                }
                
                // Get VRAM
                if let Some(vram) = gpu.get("AdapterRAM").and_then(|v| v.as_u64()) {
                    gpu_info.vram = format_bytes(vram);
                }
                
                // Get driver version
                if let Some(driver) = gpu.get("DriverVersion").and_then(|v| v.as_str()) {
                    gpu_info.driver = driver.to_string();
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
