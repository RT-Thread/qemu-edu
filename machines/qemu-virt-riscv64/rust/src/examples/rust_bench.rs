#[no_mangle]
pub extern "C" fn rust_bench_test() -> i32 {
    // Test the print! and println! macros
    println!("Rust component initialized!");    
    let start = time::get_time();
    for i in 0..1 {
        println!("Rust program: Hello, world! {}", i);
    }
    let end = time::get_time();
    println!("Time: {:?}", end   - start);
    0
}