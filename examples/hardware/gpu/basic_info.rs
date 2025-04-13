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

//! Example demonstrating basic GPU information retrieval.

use catp2p::hardware::gpu::{get_info, TemperatureUnit};
use catp2p::error::Error;
use std::time::Instant;
use std::io::{self, Write};
use colored::*;

fn main() -> Result<(), Error> {
    // Enable colored output
    colored::control::set_override(true);

    println!("{}", "=== Basic GPU Information Example ===".bright_green().bold());
    println!();

    println!("{}", "This example retrieves detailed information about the primary GPU in your system.".blue());
    println!();
    
    print!("{}", "Retrieving primary GPU information... ".blue());
    io::stdout().flush().unwrap();
    
    let primary_start = Instant::now();
    match get_info() {
        Ok(gpu_info) => {
            let primary_time = primary_start.elapsed();
            println!("{} (took {:.2?})", "Done!".green().bold(), primary_time);
            
            println!("\n{}", "Primary GPU Details:".cyan().bold());
            println!("{} {}", "Name:".cyan(), gpu_info.name.white());
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
            
            // Display additional properties if available
            if !gpu_info.additional_properties.is_empty() {
                println!("\n{}", "Additional Properties:".cyan().bold());
                let mut sorted_props: Vec<_> = gpu_info.additional_properties.iter().collect();
                sorted_props.sort_by(|a, b| a.0.cmp(b.0));
                
                for (key, value) in sorted_props {
                    println!("{} {}", format!("{}:", key).cyan(), value.white());
                }
            }
        },
        Err(e) => {
            println!("{}: {}", "Failed".red().bold(), e);
            println!("{}", "Could not retrieve primary GPU information.".yellow());
        }
    }
    
    println!();
    println!("{}", "=== End of Basic GPU Information Example ===".bright_green().bold());
    
    Ok(())
}
