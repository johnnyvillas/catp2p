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

//! Example demonstrating performance comparison of GPU information methods.

use catp2p::hardware::gpu::{get_info, get_all_info, is_available, get_usage};
use catp2p::error::Error;
use std::time::Instant;
use std::io::{self, Write};
use colored::*;

fn main() -> Result<(), Error> {
    // Enable colored output
    colored::control::set_override(true);

    println!("{}", "=== GPU Information Performance Comparison ===".bright_green().bold());
    println!();

    println!("{}", "This example compares the performance of different methods for retrieving GPU information.".blue());
    println!();
    
    // Measure performance of different methods
    let iterations = 5;
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
    
    // Measure get_all_info performance
    let mut total_time_all_info = std::time::Duration::new(0, 0);
    for i in 1..=iterations {
        print!("{} ", format!("get_all_info() iteration {}/{}...", i, iterations).blue());
        io::stdout().flush().unwrap();
        
        let start = Instant::now();
        match get_all_info() {
            Ok(_) => {
                let elapsed = start.elapsed();
                total_time_all_info += elapsed;
                println!("{} ({:.2?})", "Done".green(), elapsed);
            },
            Err(e) => {
                println!("{}: {}", "Failed".red().bold(), e);
            }
        }
    }
    let avg_time_all_info = total_time_all_info / iterations;
    
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
    println!("{} {:.2?}", "Average time for get_all_info():".cyan(), avg_time_all_info);
    println!("{} {:.2?}", "Average time for is_available():".cyan(), avg_time_available);
    println!("{} {:.2?}", "Average time for get_usage():".cyan(), avg_time_usage);
    
    // Determine the fastest method
    let fastest_method = if avg_time_info <= avg_time_all_info && avg_time_info <= avg_time_available && avg_time_info <= avg_time_usage {
        "get_info()"
    } else if avg_time_all_info <= avg_time_info && avg_time_all_info <= avg_time_available && avg_time_all_info <= avg_time_usage {
        "get_all_info()"
    } else if avg_time_available <= avg_time_info && avg_time_available <= avg_time_all_info && avg_time_available <= avg_time_usage {
        "is_available()"
    } else {
        "get_usage()"
    };
    
    println!("{} {}", "Fastest method:".cyan(), fastest_method.green().bold());
    
    // Recommendations
    println!("\n{}", "Recommendations:".cyan().bold());
    println!("{}", "1. Use is_available() for quick checks before attempting GPU operations.".white());
    println!("{}", "2. Cache GPU information when possible to avoid repeated queries.".white());
    println!("{}", "3. Use get_info() for most cases, and get_all_info() only when you need information about multiple GPUs.".white());
    println!("{}", "4. For real-time monitoring, use get_usage() sparingly to avoid performance impact.".white());
    
    println!();
    println!("{}", "=== End of GPU Information Performance Comparison ===".bright_green().bold());
    
    Ok(())
}
