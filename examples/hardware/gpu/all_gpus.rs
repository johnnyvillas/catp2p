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

//! Example demonstrating retrieval of information about all GPUs.

use catp2p::hardware::gpu::{get_all_info, TemperatureUnit};
use catp2p::error::Error;
use std::time::Instant;
use std::io::{self, Write};
use colored::*;

fn main() -> Result<(), Error> {
    // Enable colored output
    colored::control::set_override(true);

    println!("{}", "=== All GPUs Information Example ===".bright_green().bold());
    println!();

    println!("{}", "This example retrieves information about all GPUs available in your system.".blue());
    println!();
    
    print!("{}", "Retrieving information for all GPUs... ".blue());
    io::stdout().flush().unwrap();
    
    let all_gpus_start = Instant::now();
    match get_all_info() {
        Ok(gpu_infos) => {
            let all_gpus_time = all_gpus_start.elapsed();
            println!("{} (took {:.2?})", "Done!".green().bold(), all_gpus_time);
            
            println!("{} {}", "Number of GPUs detected:".cyan(), gpu_infos.len().to_string().white().bold());
            
            for (i, gpu_info) in gpu_infos.iter().enumerate() {
                println!("\n{} {}", format!("GPU {}:", i+1).cyan().bold(), gpu_info.name.white().bold());
                println!("{} {}", "Vendor:".cyan(), gpu_info.vendor.white());
                println!("{} {}", "Architecture:".cyan(), gpu_info.architecture.white());
                println!("{} {}", "Driver:".cyan(), gpu_info.driver.white());
                println!("{} {}", "VRAM:".cyan(), gpu_info.vram.white());
                println!("{} {}", "Type:".cyan(), 
                    if gpu_info.is_integrated { "Integrated".white() } else { "Discrete".white() });
                
                // Display temperature if available (in both Celsius and Fahrenheit)
                if let Some(temp) = gpu_info.temperature {
                    println!("{} {:.1}°C / {:.1}°F", "Temperature:".cyan(), 
                             temp, 
                             gpu_info.temperature_in(TemperatureUnit::Fahrenheit).unwrap());
                }
                
                // Display a few key additional properties if available
                if !gpu_info.additional_properties.is_empty() {
                    let important_props = ["Total VRAM", "Free VRAM", "GPU Utilization"];
                    let mut has_props = false;
                    
                    for prop in important_props.iter() {
                        if let Some(value) = gpu_info.additional_properties.get(*prop) {
                            if !has_props {
                                println!("\n{}", "Key Properties:".cyan());
                                has_props = true;
                            }
                            println!("{} {}", format!("{}:", prop).cyan(), value.white());
                        }
                    }
                }
            }
        },
        Err(e) => {
            println!("{}: {}", "Failed".red().bold(), e);
            println!("{}", "Could not retrieve information for all GPUs.".yellow());
        }
    }
    
    println!();
    println!("{}", "=== End of All GPUs Information Example ===".bright_green().bold());
    
    Ok(())
}
