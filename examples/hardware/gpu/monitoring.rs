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

//! Example demonstrating GPU usage monitoring over time.

use catp2p::hardware::gpu::{monitor_usage, get_info, TemperatureUnit};
use catp2p::error::Error;
use std::time::Duration;
use colored::*;

fn main() -> Result<(), Error> {
    // Enable colored output
    colored::control::set_override(true);

    println!("{}", "=== GPU Usage Monitoring Example ===".bright_green().bold());
    println!();

    println!("{}", "This example monitors your GPU usage over a short period to show how usage patterns can be tracked in real-time.".blue());
    println!();
    
    // First, get the current temperature as a baseline
    match get_info() {
        Ok(gpu_info) => {
            println!("{} {}", "GPU:".cyan(), gpu_info.name.white());
            
            // Display temperature if available (in both Celsius and Fahrenheit)
            if let Some(temp) = gpu_info.temperature {
                println!("{} {:.1}°C / {:.1}°F", "Current Temperature:".cyan(), 
                         temp, 
                         gpu_info.temperature_in(TemperatureUnit::Fahrenheit).unwrap());
            } else {
                println!("{} {}", "Current Temperature:".cyan(), "Not available".yellow());
            }
        },
        Err(e) => {
            println!("{}: {}", "Failed to get GPU information".red().bold(), e);
        }
    }
    
    println!();
    println!("{}", "Monitoring GPU usage for 3 seconds...".blue());
    
    match monitor_usage(Duration::from_secs(3), Duration::from_millis(500)) {
        Ok(stats) => {
            println!("\n{}", "GPU Usage Statistics:".cyan().bold());
            println!("{} {}", "Name:".cyan(), stats.name.white());
            println!("{} {:.1}%", "Average GPU Utilization:".cyan(), stats.avg_usage_percent);
            println!("{} {:.1}% / {:.1}%", "Min/Max GPU Utilization:".cyan(), 
                    stats.min_usage_percent, stats.max_usage_percent);
            println!("{} {}", "Average VRAM Usage:".cyan(), stats.avg_used_vram.white());
            println!("{} {} / {}", "Min/Max VRAM Usage:".cyan(), 
                    stats.min_used_vram.white(), stats.max_used_vram.white());
            println!("{} {}", "Total VRAM:".cyan(), stats.total_vram.white());
            println!("{} {}", "Samples Collected:".cyan(), stats.sample_count.to_string().white());
            println!("{} {:.2?}", "Monitoring Duration:".cyan(), stats.duration);
            
            // Check temperature again after monitoring
            match get_info() {
                Ok(gpu_info) => {
                    if let Some(temp) = gpu_info.temperature {
                        println!("{} {:.1}°C / {:.1}°F", "Temperature After Monitoring:".cyan(), 
                                 temp, 
                                 gpu_info.temperature_in(TemperatureUnit::Fahrenheit).unwrap());
                        
                        // Show a simple temperature change indicator
                        match get_info() {
                            Ok(initial_info) => {
                                if let Some(initial_temp) = initial_info.temperature {
                                    let temp_diff = temp - initial_temp;
                                    if temp_diff > 0.5 {
                                        println!("{} +{:.1}°C", "Temperature Change:".cyan(), temp_diff.to_string().red());
                                    } else if temp_diff < -0.5 {
                                        println!("{} {:.1}°C", "Temperature Change:".cyan(), temp_diff.to_string().green());
                                    } else {
                                        println!("{} {:.1}°C", "Temperature Change:".cyan(), temp_diff.to_string().white());
                                    }
                                }
                            },
                            Err(_) => {}
                        }
                    }
                },
                Err(_) => {}
            }
        },
        Err(e) => {
            println!("{}: {}", "Failed".red().bold(), e);
            println!("{}", "Could not monitor GPU usage.".yellow());
        }
    }
    
    println!();
    println!("{}", "=== End of GPU Usage Monitoring Example ===".bright_green().bold());
    
    Ok(())
}
