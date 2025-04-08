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

//! Benchmarks for task execution.

use catp2p::tasks::{Task, TaskResources, TaskResourceType, TaskStatus};
use criterion::{black_box, criterion_group, criterion_main, Criterion};
use std::time::Duration;

fn create_test_task() -> Task {
    Task {
        id: "test-task-1".to_string(),
        name: "Test Task".to_string(),
        status: TaskStatus::Pending,
        resources: TaskResources {
            resource_type: TaskResourceType::CPU,
            cpu_cores: Some(1),
            memory_bytes: Some(1024 * 1024), // 1 MB
            gpu_memory_bytes: None,
        },
        priority: 1,
        created_at: 0,
        started_at: None,
        completed_at: None,
        result: None,
        error: None,
    }
}

fn cpu_intensive_task(iterations: u64) -> u64 {
    let mut sum = 0;
    for i in 0..iterations {
        sum = sum.wrapping_add(i);
        // Add some complexity to prevent optimization
        if i % 1000 == 0 {
            sum = sum.wrapping_mul(i);
        }
    }
    sum
}

fn benchmark_cpu_task(c: &mut Criterion) {
    let mut group = c.benchmark_group("CPU Tasks");
    group.measurement_time(Duration::from_secs(10));
    
    for size in [1_000, 10_000, 100_000, 1_000_000].iter() {
        group.bench_function(format!("cpu_task_{}", size), |b| {
            b.iter(|| cpu_intensive_task(black_box(*size)))
        });
    }
    
    group.finish();
}

fn memory_intensive_task(size_mb: usize) -> Vec<u8> {
    let size = size_mb * 1024 * 1024;
    let mut data = Vec::with_capacity(size);
    for i in 0..size {
        data.push((i % 256) as u8);
    }
    data
}

fn benchmark_memory_task(c: &mut Criterion) {
    let mut group = c.benchmark_group("Memory Tasks");
    group.measurement_time(Duration::from_secs(10));
    
    for size in [1, 10, 50].iter() {
        group.bench_function(format!("memory_task_{}mb", size), |b| {
            b.iter(|| {
                let data = memory_intensive_task(black_box(*size));
                black_box(data.len())
            })
        });
    }
    
    group.finish();
}

criterion_group!(benches, benchmark_cpu_task, benchmark_memory_task);
criterion_main!(benches);
