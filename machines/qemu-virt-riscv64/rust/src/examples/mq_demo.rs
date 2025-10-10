/// Test message structure
#[derive(Debug, Clone, PartialEq)]
struct TestMessage {
    id: u32,
    data: [u8; 16],
    timestamp: u32,
}

impl TestMessage {
    fn new(id: u32, data: &str) -> Self {
        let mut msg_data = [0u8; 16];
        let bytes = data.as_bytes();
        let len = bytes.len().min(16);
        msg_data[..len].copy_from_slice(&bytes[..len]);
        
        Self {
            id,
            data: msg_data,
            timestamp: 0, // In real scenario, would use system tick
        }
    }
}

/// Test basic queue operations
fn test_basic_queue_operations() {
    println!("=== Testing Basic Queue Operations ===");
    
    // Create a queue for u32 values
    let queue = Queue::<u32>::new(5).expect("Failed to create queue");
    
    // Test try_send (non-blocking)
    println!("Testing try_send...");
    for i in 0..5 {
        match queue.try_send(i * 10) {
            Ok(()) => println!("Successfully sent: {}", i * 10),
            Err((err, value)) => println!("Failed to send {}: {:?}", value, err),
        }
    }
    
    // Try to send one more (should fail - queue full)
    match queue.try_send(999) {
        Ok(()) => println!("Unexpected success sending to full queue"),
        Err((err, value)) => println!("Expected failure sending {} to full queue: {:?}", value, err),
    }
    
    // Test try_recv (non-blocking)
    println!("Testing try_recv...");
    for i in 0..5 {
        match queue.try_recv() {
            Ok(value) => println!("Successfully received: {}", value),
            Err(err) => println!("Failed to receive: {:?}", err),
        }
    }
    
    // Try to receive one more (should fail - queue empty)
    match queue.try_recv() {
        Ok(value) => println!("Unexpected success receiving from empty queue: {}", value),
        Err(err) => println!("Expected failure receiving from empty queue: {:?}", err),
    }
    
    println!("Basic queue operations test completed.\n");
}

/// Test queue with timeout operations
fn test_timeout_operations() {
    println!("=== Testing Timeout Operations ===");
    
    let queue = Queue::<i32>::new_with_name("timeout_queue", 3)
        .expect("Failed to create named queue");
    
    // Test send with timeout
    println!("Testing send with timeout...");
    for i in 0..3 {
        match queue.send(i, 100) { // 100 tick timeout
            Ok(()) => println!("Successfully sent with timeout: {}", i),
            Err((err, value)) => println!("Failed to send {} with timeout: {:?}", value, err),
        }
    }
    
    // Test recv with timeout
    println!("Testing recv with timeout...");
    for i in 0..3 {
        match queue.recv(100) { // 100 tick timeout
            Ok(value) => println!("Successfully received with timeout: {}", value),
            Err(err) => println!("Failed to receive with timeout: {:?}", err),
        }
    }
    
    // Test timeout on empty queue
    println!("Testing timeout on empty queue...");
    match queue.recv(50) { // Short timeout
        Ok(value) => println!("Unexpected success: {}", value),
        Err(err) => println!("Expected timeout error: {:?}", err),
    }
    
    println!("Timeout operations test completed.\n");
}

/// Test queue capacity and edge cases
fn test_queue_edge_cases() {
    println!("=== Testing Queue Edge Cases ===");
    
    // Test with minimum capacity
    println!("Testing queue with capacity 1...");
    let small_queue = Queue::<u8>::new(1).expect("Failed to create small queue");
    
    // Fill the queue
    match small_queue.try_send(42) {
        Ok(()) => println!("Successfully filled single-item queue"),
        Err((err, value)) => println!("Failed to fill queue: {:?}", err),
    }
    
    // Try to overfill
    match small_queue.try_send(99) {
        Ok(()) => println!("Unexpected success overfilling queue"),
        Err((err, value)) => println!("Expected failure overfilling queue: {:?}", err),
    }
    
    // Empty the queue
    match small_queue.try_recv() {
        Ok(value) => println!("Successfully emptied queue, got: {}", value),
        Err(err) => println!("Failed to empty queue: {:?}", err),
    }
    
    // Try to over-empty
    match small_queue.try_recv() {
        Ok(value) => println!("Unexpected success over-emptying queue: {}", value),
        Err(err) => println!("Expected failure over-emptying queue: {:?}", err),
    }
    
    println!("Edge cases test completed.\n");
}

/// Main demo function
pub fn mq_demo() {
    println!("Starting Message Queue Demo");
    println!("============================");
    
    test_basic_queue_operations();
    test_timeout_operations();
    test_queue_edge_cases();
    
    println!("Message Queue Demo completed successfully!");
    println!("All Queue<T> interfaces have been tested.");
}

/// Entry point for the message queue demo
#[no_mangle]
pub extern "C" fn rust_mq_demo() {
    mq_demo();
}