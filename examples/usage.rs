use luo_capture::capture::*;
use std::time::Instant;

#[tokio::main]
async fn main() {
    // Example usage of the capture module with timing

    // Method 1: Synchronous initialization and capture
    println!("Initializing capture module...");
    let start_time = Instant::now();
    let mut screen_capture = init().expect("Failed to initialize capture");
    let init_duration = start_time.elapsed();
    println!("Capture initialized successfully! Duration: {:.3}ms", init_duration.as_secs_f64() * 1000.0);

    // Define a capture region (x, y, width, height)
    let region = CaptureRegion {
        x: 0,
        y: 0,
        width: 800,
        height: 600,
    };

    println!("Capturing region: {:?}", region);
    let start_time = Instant::now();
    match screen_capture.capture(region, None) {
        Ok(capture_data) => {
            let capture_duration = start_time.elapsed();
            println!("Capture successful! Duration: {:.3}ms", capture_duration.as_secs_f64() * 1000.0);
            println!("Width: {}, Height: {}, Data size: {} bytes",
                     capture_data.width, capture_data.height, capture_data.data.len());
        },
        Err(e) => {
            let capture_duration = start_time.elapsed();
            eprintln!("Capture failed after {:.3}ms: {}", capture_duration.as_secs_f64() * 1000.0, e);
        }
    }

    // Method 2: Asynchronous initialization and capture
    println!("\nInitializing async capture module...");
    let start_time = Instant::now();
    let async_capture = init_async().await.expect("Failed to initialize async capture");
    let init_duration = start_time.elapsed();
    println!("Async capture initialized successfully! Duration: {:.3}ms", init_duration.as_secs_f64() * 1000.0);

    println!("Async capturing region: {:?}", region);
    let start_time = Instant::now();
    match async_capture.capture(region, None).await {
        Ok(capture_data) => {
            let capture_duration = start_time.elapsed();
            println!("Async capture successful! Duration: {:.3}ms", capture_duration.as_secs_f64() * 1000.0);
            println!("Width: {}, Height: {}, Data size: {} bytes",
                     capture_data.width, capture_data.height, capture_data.data.len());
        },
        Err(e) => {
            let capture_duration = start_time.elapsed();
            eprintln!("Async capture failed after {:.3}ms: {}", capture_duration.as_secs_f64() * 1000.0, e);
        }
    }

    // Method 3: Using convenience functions
    println!("\nUsing convenience functions...");
    let mut capture_instance = ScreenCapture::new();
    let start_time = Instant::now();
    capture_instance.init().expect("Failed to initialize capture");
    let init_duration = start_time.elapsed();
    println!("Capture instance initialized! Duration: {:.3}ms", init_duration.as_secs_f64() * 1000.0);

    let region2 = CaptureRegion {
        x: 100,
        y: 100,
        width: 400,
        height: 300,
    };

    let start_time = Instant::now();
    match luo_capture::capture(&mut capture_instance, region2, None) {
        Ok(capture_data) => {
            let capture_duration = start_time.elapsed();
            println!("Convenience function capture successful! Duration: {:.3}ms", capture_duration.as_secs_f64() * 1000.0);
            println!("Width: {}, Height: {}, Data size: {} bytes",
                     capture_data.width, capture_data.height, capture_data.data.len());
        },
        Err(e) => {
            let capture_duration = start_time.elapsed();
            eprintln!("Convenience function capture failed after {:.3}ms: {}", capture_duration.as_secs_f64() * 1000.0, e);
        }
    }

    // Method 4: Capture with PNG save functionality
    println!("\nTesting PNG save functionality...");
    let region3 = CaptureRegion {
        x: 50,
        y: 50,
        width: 200,
        height: 150,
    };

    let save_path = "capture_example.png";
    let start_time = Instant::now();
    match screen_capture.capture(region3, Some(save_path)) {
        Ok(capture_data) => {
            let capture_duration = start_time.elapsed();
            println!("Capture with PNG save successful! Duration: {:.3}ms", capture_duration.as_secs_f64() * 1000.0);
            println!("Width: {}, Height: {}, Data size: {} bytes", capture_data.width, capture_data.height, capture_data.data.len());
            println!("PNG saved to: {}", save_path);

            // Check if file exists
            if std::path::Path::new(save_path).exists() {
                println!("✓ PNG file created successfully");
            } else {
                println!("✗ PNG file was not created");
            }
        },
        Err(e) => {
            let capture_duration = start_time.elapsed();
            eprintln!("Capture with PNG save failed after {:.3}ms: {}", capture_duration.as_secs_f64() * 1000.0, e);
        }
    }
}