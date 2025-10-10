/*
 * Copyright (c) 2006-2024, RT-Thread Development Team
 *
 * SPDX-License-Identifier: Apache-2.0
 *
 * Change Logs:
 * Date           Author       Notes
 * 2025-01-15     foxglove     semaphore demo
 */

/// Basic semaphore operations demo
#[no_mangle]
pub extern "C" fn rust_semaphore_basic_demo() {
    println!("\n=== Basic Semaphore Demo ===");
    
    // Create a semaphore
    match Semaphore::new_with_name("test_sem") {
        Ok(sem) => {
            println!("Semaphore created successfully");
            
            // Test try_take (should fail initially as semaphore starts with 0)
            match sem.try_take() {
                Ok(_) => println!("try_take: SUCCESS (unexpected)"),
                Err(e) => println!("try_take: FAILED as expected - {:?}", e),
            }
            
            // Release semaphore to make it available
            sem.release();
            println!("Semaphore released");
            
            // Now try_take should succeed
            match sem.try_take() {
                Ok(_) => println!("try_take: SUCCESS after release"),
                Err(e) => println!("try_take: FAILED - {:?}", e),
            }
            
            // Release again for next test
            sem.release();
            
            // Test take with timeout
            match sem.take(100) {
                Ok(_) => println!("take(100): SUCCESS"),
                Err(e) => println!("take(100): FAILED - {:?}", e),
            }
            
            println!("Basic semaphore demo completed!");
        }
        Err(e) => {
            println!("Failed to create semaphore: {:?}", e);
        }
    }
}

/// Producer-Consumer demo using semaphore
#[no_mangle]
pub extern "C" fn rust_semaphore_producer_consumer_demo() {
    println!("\n=== Producer-Consumer Semaphore Demo ===");
    
    // Create semaphores for synchronization
    let empty_sem = match Semaphore::new_with_name("empty") {
        Ok(sem) => {
            // Initialize with buffer size (3 slots)
            sem.release();
            sem.release();
            sem.release();
            Arc::new(sem)
        }
        Err(e) => {
            println!("Failed to create empty semaphore: {:?}", e);
            return;
        }
    };
    
    let full_sem = match Semaphore::new_with_name("full") {
        Ok(sem) => Arc::new(sem),
        Err(e) => {
            println!("Failed to create full semaphore: {:?}", e);
            return;
        }
    };
    
    let mutex_sem = match Semaphore::new_with_name("mutex") {
        Ok(sem) => {
            sem.release(); // Initialize as available
            Arc::new(sem)
        }
        Err(e) => {
            println!("Failed to create mutex semaphore: {:?}", e);
            return;
        }
    };
    
    // Shared buffer counter (simulated)
    static mut BUFFER_COUNT: i32 = 0;
    
    // Producer thread
    let empty_sem_producer = Arc::clone(&empty_sem);
    let full_sem_producer = Arc::clone(&full_sem);
    let mutex_sem_producer = Arc::clone(&mutex_sem);
    
    let producer = Thread::spawn(
        String::from("producer"),
        2048,
        10,
        20,
        move || {
            for i in 1..=5 {
                // Wait for empty slot
                if let Err(e) = empty_sem_producer.take_wait_forever() {
                    println!("[Producer] Failed to wait for empty slot: {:?}", e);
                    continue;
                }
                
                // Enter critical section
                if let Err(e) = mutex_sem_producer.take_wait_forever() {
                    println!("[Producer] Failed to acquire mutex: {:?}", e);
                    continue;
                }
                
                // Produce item
                unsafe {
                    BUFFER_COUNT += 1;
                    println!("[Producer] Produced item {}, buffer count: {}", i, BUFFER_COUNT);
                }
                
                // Exit critical section
                mutex_sem_producer.release();
                
                // Signal full slot
                full_sem_producer.release();
                
                Thread::ms_delay(200);
            }
            println!("[Producer] Finished producing");
        }
    );
    
    // Consumer thread
    let empty_sem_consumer = Arc::clone(&empty_sem);
    let full_sem_consumer = Arc::clone(&full_sem);
    let mutex_sem_consumer = Arc::clone(&mutex_sem);
    
    let consumer = Thread::spawn(
        String::from("consumer"),
        2048,
        10,
        20,
        move || {
            for i in 1..=5 {
                // Wait for full slot
                if let Err(e) = full_sem_consumer.take_wait_forever() {
                    println!("[Consumer] Failed to wait for full slot: {:?}", e);
                    continue;
                }
                
                // Enter critical section
                if let Err(e) = mutex_sem_consumer.take_wait_forever() {
                    println!("[Consumer] Failed to acquire mutex: {:?}", e);
                    continue;
                }
                
                // Consume item
                unsafe {
                    BUFFER_COUNT -= 1;
                    println!("[Consumer] Consumed item {}, buffer count: {}", i, BUFFER_COUNT);
                }
                
                // Exit critical section
                mutex_sem_consumer.release();
                
                // Signal empty slot
                empty_sem_consumer.release();
                
                Thread::ms_delay(300);
            }
            println!("[Consumer] Finished consuming");
        }
    );
    
    // Wait for threads to complete
    Thread::ms_delay(3000);
    
    println!("Producer-Consumer demo completed!");
}

/// Multiple threads synchronization demo
#[no_mangle]
pub extern "C" fn rust_semaphore_multi_thread_demo() {
    println!("\n=== Multi-Thread Semaphore Demo ===");
    
    // Create a semaphore with limited resources (2 permits)
    let resource_sem = match Semaphore::new_with_name("resource") {
        Ok(sem) => {
            // Initialize with 2 permits
            sem.release();
            sem.release();
            Arc::new(sem)
        }
        Err(e) => {
            println!("Failed to create resource semaphore: {:?}", e);
            return;
        }
    };
    
    // Create multiple worker threads
    for worker_id in 1..=4 {
        let sem_clone = Arc::clone(&resource_sem);
        
        let _worker = Thread::spawn(
            format!("worker_{}", worker_id),
            2048,
            12,
            20,
            move || {
                println!("[Worker {}] Trying to acquire resource...", worker_id);
                
                match sem_clone.take_wait_forever() {
                    Ok(_) => {
                        println!("[Worker {}] Acquired resource, working...", worker_id);
                        
                        // Simulate work
                        Thread::ms_delay(500);
                        
                        println!("[Worker {}] Releasing resource", worker_id);
                        sem_clone.release();
                    }
                    Err(e) => {
                        println!("[Worker {}] Failed to acquire resource: {:?}", worker_id, e);
                    }
                }
            }
        );
    }
    
    // Wait for all workers to complete
    Thread::ms_delay(3000);
    
    println!("Multi-thread semaphore demo completed!");
}

/// Comprehensive semaphore test
#[no_mangle]
pub extern "C" fn rust_semaphore_demo_all() {
    println!("\n=== Comprehensive Semaphore Demo ===");
    
    // Run all demos
    rust_semaphore_basic_demo();
    Thread::ms_delay(500);
    
    rust_semaphore_producer_consumer_demo();
    Thread::ms_delay(500);
    
    rust_semaphore_multi_thread_demo();
    
    println!("\n=== All Semaphore Demos Completed ===");
}