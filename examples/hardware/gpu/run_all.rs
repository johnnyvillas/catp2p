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

//! Example demonstrating all GPU information features.

use catp2p::hardware::gpu::{get_info, get_all_info, is_available, get_usage, monitor_usage, TemperatureUnit};
use catp2p::error::Error;
use std::time::{Duration, Instant};
use std::io::{self, Write};
use colored::*;

fn main() -> Result<(), Error> {
    // Enable colored output
    colored::control::set_override(true);

    println!("{}", "=== CatP2P GPU Information Examples ===".bright_green().bold());
    println!();

    println!("{}", "This example demonstrates all GPU information features available in CatP2P.".blue());
    println!("{}", "You can also run each example individually using the commands shown below.".blue());
    println!();

    let examples = [
        ("GPU Availability Check", "cargo run --example hardware_gpu_availability"),
        ("Basic GPU Information", "cargo run --example hardware_gpu_basic_info"),
        ("All GPUs Information", "cargo run --example hardware_gpu_all_gpus"),
        ("GPU Usage Information", "cargo run --example hardware_gpu_usage_info"),
        ("GPU Usage Monitoring", "cargo run --example hardware_gpu_monitoring"),
        ("Performance Comparison", "cargo run --example hardware_gpu_performance"),
    ];

    // Display available examples
    println!("{}", "Available Examples:".cyan().bold());
    for (i, (name, command)) in examples.iter().enumerate() {
        println!("{}. {} - Run with: {}", i+1, name.white().bold(), command.green());
    }
    println!();

    // Ask user which example to run
    println!("{}", "Options:".cyan().bold());
    println!("{}. {}", "A".white().bold(), "Run all examples".white());
    for (i, (name, _)) in examples.iter().enumerate() {
        println!("{}. {}", i+1, name.white());
    }
    println!("{}. {}", "Q".white().bold(), "Quit".white());
    println!();

    loop {
        print!("{} ", "Enter your choice (A, 1-6, Q):".cyan());
        io::stdout().flush().unwrap();

        let mut choice = String::new();
        io::stdin().read_line(&mut choice).expect("Failed to read input");
        let choice = choice.trim().to_uppercase();

        match choice.as_str() {
            "A" => {
                run_all_examples()?;
                break;
            },
            "1" => {
                run_availability_example()?;
                break;
            },
            "2" => {
                run_basic_info_example()?;
                break;
            },
            "3" => {
                run_all_gpus_example()?;
                break;
            },
            "4" => {
                run_usage_info_example()?;
                break;
            },
            "5" => {
                run_monitoring_example()?;
                break;
            },
            "6" => {
                run_performance_example()?;
                break;
            },
            "Q" => {
                println!("{}", "Exiting...".yellow());
                break;
            },
            _ => {
                println!("{}", "Invalid choice. Please try again.".red());
            }
        }
    }

    Ok(())
}

fn run_all_examples() -> Result<(), Error> {
    println!("\n{}", "Running all examples...".bright_green().bold());
    
    run_availability_example()?;
    println!("\n{}", "Press Enter to continue...".yellow());
    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("Failed to read input");
    
    run_basic_info_example()?;
    println!("\n{}", "Press Enter to continue...".yellow());
    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("Failed to read input");
    
    run_all_gpus_example()?;
    println!("\n{}", "Press Enter to continue...".yellow());
    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("Failed to read input");
    
    run_usage_info_example()?;
    println!("\n{}", "Press Enter to continue...".yellow());
    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("Failed to read input");
    
    run_monitoring_example()?;
    println!("\n{}", "Press Enter to continue...".yellow());
    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("Failed to read input");
    
    run_performance_example()?;
    
    println!("\n{}", "All examples completed!".bright_green().bold());
    
    Ok(())
}

fn run_availability_example() -> Result<(), Error> {
    println!("\n{}", "=== GPU Availability Check ===".yellow().bold());
    
    let availability_start = Instant::now();
    let gpu_available = is_available();
    let availability_time = availability_start.elapsed();
    
    if gpu_available {
        println!("{} {}", "GPU Status:".cyan(), "Available".green().bold());
    } else {
        println!("{} {}", "GPU Status:".cyan(), "Not Available".red().bold());
        println!("{}", "No compatible GPU was found on your system.".yellow());
    }
    println!("{} {:.2?}", "Time to check availability:".cyan(), availability_time);
    
    Ok(())
}

fn run_basic_info_example() -> Result<(), Error> {
    println!("\n{}", "=== Basic GPU Information ===".yellow().bold());
    
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
    
    Ok(())
}

fn run_all_gpus_example() -> Result<(), Error> {
    println!("\n{}", "=== All GPUs Information ===".yellow().bold());
    
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
    
    Ok(())
}

fn run_usage_info_example() -> Result<(), Error> {
    println!("\n{}", "=== GPU Usage Information ===".yellow().bold());
    
    print!("{}", "Retrieving GPU usage information... ".blue());
    io::stdout().flush().unwrap();
    
    let usage_start = Instant::now();
    match get_usage() {
        Ok(usage_info) => {
            let usage_time = usage_start.elapsed();
            println!("{} (took {:.2?})", "Done!".green().bold(), usage_time);
            
            println!("\n{}", "GPU Usage Information:".cyan().bold());
            println!("{} {}", "Name:".cyan(), usage_info.name.white());
            println!("{} {}", "Vendor:".cyan(), usage_info.vendor.white());
            println!("{} {:.1}%", "GPU Utilization:".cyan(), usage_info.gpu_usage_percent);
            println!("{} {} / {}", "VRAM Usage:".cyan(), 
                    usage_info.used_vram.white(), 
                    usage_info.total_vram.white());
            
            // Calculate percentage of VRAM used
            if usage_info.total_vram_bytes > 0 {
                let vram_percent = (usage_info.used_vram_bytes as f32 / usage_info.total_vram_bytes as f32) * 100.0;
                println!("{} {:.1}%", "VRAM Utilization:".cyan(), vram_percent);
            }
        },
        Err(e) => {
            println!("{}: {}", "Failed".red().bold(), e);
            println!("{}", "Could not retrieve GPU usage information.".yellow());
        }
    }
    
    Ok(())
}

fn run_monitoring_example() -> Result<(), Error> {
    println!("\n{}", "=== GPU Usage Monitoring ===".yellow().bold());
    
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
        },
        Err(e) => {
            println!("{}: {}", "Failed".red().bold(), e);
            println!("{}", "Could not monitor GPU usage.".yellow());
        }
    }
    
    Ok(())
}

fn run_performance_example() -> Result<(), Error> {
    println!("\n{}", "=== GPU Information Performance Comparison ===".yellow().bold());
    
    // Measure performance of different methods
    let iterations = 3; // Reduced for the combined example
    println!("{}", format!("Running {} iterations of each method:", iterations).blue());
    
    // Measure get_info performance
    let mut total_time_info = std::time::Duration::new(0, 0);
    for i in 1..=iterations {
        print!("{} ", format!("get_info() iteration {}/{}...", i, iterations).blue());
        io::stdout().flush().unwrap();
        
        let start = Instant::now();
        match get_info() {
            Ok(_) => {
                let elapsed = start.elapsed();
                total_time_info += elapsed;
                println!("{} ({:.2?})", "Done".green(), elapsed);
            },
            Err(e) => {
                println!("{}: {}", "Failed".red().bold(), e);
            }
        }
    }
    let avg_time_info = total_time_info / iterations;
    
    // Measure is_available performance
    let mut total_time_available = std::time::Duration::new(0, 0);
    for i in 1..=iterations {
        print!("{} ", format!("is_available() iteration {}/{}...", i, iterations).blue());
        io::stdout().flush().unwrap();
        
        let start = Instant::now();
        let _ = is_available();
        let elapsed = start.elapsed();
        total_time_available += elapsed;
        println!("{} ({:.2?})", "Done".green(), elapsed);
    }
    let avg_time_available = total_time_available / iterations;
    
    // Measure get_usage performance
    let mut total_time_usage = std::time::Duration::new(0, 0);
    for i in 1..=iterations {
        print!("{} ", format!("get_usage() iteration {}/{}...", i, iterations).blue());
        io::stdout().flush().unwrap();
        
        let start = Instant::now();
        match get_usage() {
            Ok(_) => {
                let elapsed = start.elapsed();
                total_time_usage += elapsed;
                println!("{} ({:.2?})", "Done".green(), elapsed);
            },
            Err(e) => {
                println!("{}: {}", "Failed".red().bold(), e);
            }
        }
    }
    let avg_time_usage = total_time_usage / iterations;
    
    // Print performance summary
    println!("\n{}", "Performance Summary:".cyan().bold());
    println!("{} {:.2?}", "Average time for get_info():".cyan(), avg_time_info);
    println!("{} {:.2?}", "Average time for is_available():".cyan(), avg_time_available);
    println!("{} {:.2?}", "Average time for get_usage():".cyan(), avg_time_usage);
    
    // Determine the fastest method
    let fastest_method = if avg_time_info <= avg_time_available && avg_time_info <= avg_time_usage {
        "get_info()"
    } else if avg_time_available <= avg_time_info && avg_time_available <= avg_time_usage {
        "is_available()"
    } else {
        "get_usage()"
    };
    
    println!("{} {}", "Fastest method:".cyan(), fastest_method.green().bold());
    
    Ok(())
}
