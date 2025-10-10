/*
 * Copyright (c) 2006-2024, RT-Thread Development Team
 *
 * SPDX-License-Identifier: Apache-2.0
 *
 * Change Logs:
 * Date           Author       Notes
 * 2025-09-25     foxglove     test vec operations with GlobalAllocator
 */

// Imports are handled by the parent module

#[no_mangle]
pub extern "C" fn rust_vec_demo() {
    println!("\n=== Standard Vec Demo with GlobalAllocator ===");
    
    // Create a Vec with initial capacity
    let mut v: Vec<u32> = Vec::with_capacity(2);
    println!("Created Vec with capacity: {}", v.capacity());
    
    // Push elements
    for i in 1..=5u32 {
        v.push(i);
        println!("push {} -> ok", i);
    }
    
    println!("len={} cap={}", v.len(), v.capacity());
    
    // Print all elements
    for (index, &value) in v.iter().enumerate() {
        println!("v[{}]={}", index, value);
    }
    
    // Test pop operation
    println!("\nTesting pop operations:");
    while let Some(value) = v.pop() {
        println!("popped: {}, remaining len: {}", value, v.len());
    }
    
    // Test Vec methods
    println!("\nTesting Vec methods:");
    v.extend_from_slice(&[10, 20, 30, 40, 50]);
    println!("After extend_from_slice: len={}", v.len());
    
    // Test indexing
    if let Some(&value) = v.get(2) {
        println!("v[2] = {}", value);
    }
    
    // Test clear
    v.clear();
    println!("After clear: len={}, cap={}", v.len(), v.capacity());
    
    println!("Vec demo completed!");
}

#[no_mangle]
pub extern "C" fn rust_vec_advanced_demo() {
    println!("\n=== Advanced Vec Operations Demo ===");
    
    // Test Vec<String> (if we had String support)
    let mut numbers: Vec<i32> = Vec::new();
    
    // Test reserve
    numbers.reserve(10);
    println!("After reserve(10): cap={}", numbers.capacity());
    
    // Fill with data
    for i in 0..10 {
        numbers.push(i * i);
    }
    
    // Test retain
    numbers.retain(|&x| x % 2 == 0);
    println!("After retain (even numbers only): len={}", numbers.len());
    
    for (i, &num) in numbers.iter().enumerate() {
        println!("numbers[{}] = {}", i, num);
    }
    
    // Test insert and remove
    numbers.insert(0, 999);
    println!("After insert(0, 999): first element = {}", numbers[0]);
    
    let removed = numbers.remove(0);
    println!("Removed element: {}", removed);
    
    // Test shrink_to_fit
    numbers.shrink_to_fit();
    println!("After shrink_to_fit: len={}, cap={}", numbers.len(), numbers.capacity());
    
    println!("Advanced Vec demo completed!");
}
