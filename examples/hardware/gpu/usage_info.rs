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

//! Example demonstrating GPU usage information retrieval.

use catp2p::hardware::gpu::{get_info, get_usage, get_usage_by_name, TemperatureUnit};
use catp2p::error::Error;
use std::time::Instant;
use std::io::{self, Write};
use colored::*;

fn main() -> Result<(), Error> {
    // Enable colored output
    colored::control::set_override(true);

    println!("{}", "=== GPU Usage Information Example ===".bright_green().bold());
    println!();

    println!("{}", "This example retrieves real-time usage information about your GPU.".blue());
    println!();
    
    // First get basic GPU info to show what GPU we're monitoring
    print!("{}", "Retrieving GPU information... ".blue());
    io::stdout().flush().unwrap();
    
    let info_start = Instant::now();
    match get_info() {
        Ok(gpu_info) => {
            let info_time = info_start.elapsed();
            println!("{} (took {:.2?})", "Done!".green().bold(), info_time);
            
            println!("\n{}", "GPU Details:".cyan().bold());
            println!("{} {}", "Name:".cyan(), gpu_info.name.white());
            println!("{} {}", "Vendor:".cyan(), gpu_info.vendor.white());
            println!("{} {}", "VRAM:".cyan(), gpu_info.vram.white());
            
            // Display temperature if available (in both Celsius and Fahrenheit)
            if let Some(temp) = gpu_info.temperature {
                println!("{} {:.1}°C / {:.1}°F", "Temperature:".cyan(), 
                         temp, 
                         gpu_info.temperature_in(TemperatureUnit::Fahrenheit).unwrap());
            }
            
            // Now get real-time usage information
            print!("\n{}", "Retrieving real-time GPU usage... ".blue());
            io::stdout().flush().unwrap();
            
            let usage_start = Instant::now();
            match get_usage() {
                Ok(usage_info) => {
                    let usage_time = usage_start.elapsed();
                    println!("{} (took {:.2?})", "Done!".green().bold(), usage_time);
                    
                    println!("\n{}", "Real-time GPU Usage:".cyan().bold());
                    println!("{} {:.1}%", "GPU Utilization:".cyan(), usage_info.gpu_usage_percent);
                    println!("{} {} / {}", "VRAM Usage:".cyan(), 
                            usage_info.used_vram.white(), 
                            usage_info.total_vram.white());
                    
                    // Calculate percentage of VRAM used
                    if usage_info.total_vram_bytes > 0 {
                        let vram_percent = (usage_info.used_vram_bytes as f32 / usage_info.total_vram_bytes as f32) * 100.0;
                        println!("{} {:.1}%", "VRAM Utilization:".cyan(), vram_percent);
                    }
                    
                    // Create a simple ASCII bar chart for GPU utilization
                    let bar_length = (usage_info.gpu_usage_percent / 2.0) as usize;
                    let bar = "#".repeat(bar_length);
                    println!("\n{}", "GPU Utilization:".cyan());
                    println!("0%{:─<50}100%", "");
                    println!("{}{}", bar.green(), " ".repeat(50 - bar_length));
                    
                    // Create a simple ASCII bar chart for VRAM utilization
                    if usage_info.total_vram_bytes > 0 {
                        let vram_percent = (usage_info.used_vram_bytes as f32 / usage_info.total_vram_bytes as f32) * 100.0;
                        let bar_length = (vram_percent / 2.0) as usize;
                        let bar = "#".repeat(bar_length);
                        println!("\n{}", "VRAM Utilization:".cyan());
                        println!("0%{:─<50}100%", "");
                        println!("{}{}", bar.yellow(), " ".repeat(50 - bar_length));
                    }
                    
                    // Demonstrate get_usage_by_name
                    println!("\n{}", "Demonstrating get_usage_by_name():".cyan().bold());
                    print!("{}", "Retrieving usage for the same GPU by name... ".blue());
                    io::stdout().flush().unwrap();
                    
                    let by_name_start = Instant::now();
                    match get_usage_by_name(&gpu_info.name) {
                        Ok(usage_by_name) => {
                            let by_name_time = by_name_start.elapsed();
                            println!("{} (took {:.2?})", "Done!".green().bold(), by_name_time);
                            
                            println!("{} {:.1}%", "GPU Utilization:".cyan(), usage_by_name.gpu_usage_percent);
                            println!("{} {} / {}", "VRAM Usage:".cyan(), 
                                    usage_by_name.used_vram.white(), 
                                    usage_by_name.total_vram.white());
                        },
                        Err(e) => {
                            println!("{}: {}", "Failed".red().bold(), e);
                        }
                    }
                },
                Err(e) => {
                    println!("{}: {}", "Failed".red().bold(), e);
                    println!("{}", "Could not retrieve GPU usage information.".yellow());
                }
            }
        },
        Err(e) => {
            println!("{}: {}", "Failed".red().bold(), e);
            println!("{}", "Could not retrieve GPU information.".yellow());
        }
    }
    
    println!();
    println!("{}", "=== End of GPU Usage Information Example ===".bright_green().bold());
    
    Ok(())
}
