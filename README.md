# luo-capture-rs

A high-performance screen capture module using DXGI technology for Windows platforms.

## Features

- **High Performance**: Optimized for 1ms frame capture (theoretical)
- **DXGI Technology**: Uses DirectX Graphics Infrastructure for efficient screen capture
- **Region Capture**: Capture specific screen regions instead of full screen
- **Synchronous and Asynchronous APIs**: Both sync and async interfaces available
- **Thread-Safe**: Safe to use across multiple threads
- **Error Handling**: Comprehensive error handling with custom error types

## Installation

Add this to your `Cargo.toml`:

```toml
[dependencies]
luo-capture-rs = "0.1.0"
```

## Usage

### Basic Synchronous Capture

```rust
use luo_capture_rs::capture::*;

fn main() {
    // Initialize the capture module
    let mut capture = init().expect("Failed to initialize capture");

    // Define a capture region (x, y, width, height)
    let region = CaptureRegion {
        x: 0,
        y: 0,
        width: 800,
        height: 600,
    };

    // Capture the region
    match capture.capture(region) {
        Ok(capture_data) => {
            println!("Captured {}x{} image with {} bytes of data", 
                     capture_data.width, capture_data.height, capture_data.data.len());
        },
        Err(e) => eprintln!("Capture failed: {}", e),
    }
}
```

### Asynchronous Capture

```rust
use luo_capture_rs::capture::*;

#[tokio::main]
async fn main() {
    // Initialize the capture module asynchronously
    let async_capture = init_async().await.expect("Failed to initialize async capture");

    // Define a capture region
    let region = CaptureRegion {
        x: 0,
        y: 0,
        width: 800,
        height: 600,
    };

    // Capture the region asynchronously
    match async_capture.capture(region).await {
        Ok(capture_data) => {
            println!("Captured {}x{} image with {} bytes of data", 
                     capture_data.width, capture_data.height, capture_data.data.len());
        },
        Err(e) => eprintln!("Capture failed: {}", e),
    }
}
```

### Timing Example

```rust
use luo_capture_rs::capture::*;
use std::time::Instant;

fn main() {
    // Measure initialization time
    let start_time = Instant::now();
    let mut capture = init().expect("Failed to initialize capture");
    let init_duration = start_time.elapsed();
    println!("Initialization took: {:.3}ms", init_duration.as_secs_f64() * 1000.0);

    // Define a capture region
    let region = CaptureRegion {
        x: 0,
        y: 0,
        width: 800,
        height: 600,
    };

    // Measure capture time
    let start_time = Instant::now();
    match capture.capture(region) {
        Ok(capture_data) => {
            let capture_duration = start_time.elapsed();
            println!("Capture took: {:.3}ms", capture_duration.as_secs_f64() * 1000.0);
            println!("Captured {}x{} image with {} bytes of data", 
                     capture_data.width, capture_data.height, capture_data.data.len());
        },
        Err(e) => eprintln!("Capture failed: {}", e),
    }
}
```

## API Overview

### Core Types

- `CaptureRegion`: Defines the screen region to capture (x, y, width, height)
- `CaptureData`: Contains captured image data, dimensions, and timestamp
- `ScreenCapture`: Main capture interface
- `AsyncScreenCapture`: Async wrapper for non-blocking operations

### Main Functions

- `init()`: Initialize the capture module synchronously
- `init_async()`: Initialize the capture module asynchronously
- `capture()`: Capture a screen region synchronously
- `capture_async()`: Capture a screen region asynchronously

## Error Handling

The module provides comprehensive error handling through the `CaptureError` enum:

- `InitializationError`: Errors during initialization
- `CaptureError`: Errors during capture operations
- `InvalidRegion`: Invalid capture region parameters
- `ResourceError`: Resource allocation or management errors

## Performance Notes

- The capture module is designed for high-performance scenarios
- Region-based capture reduces memory usage and processing time
- Asynchronous API allows non-blocking operations
- Proper resource management prevents memory leaks

## Platform Support

Currently supports Windows platforms with DXGI support. The implementation uses DirectX Graphics Infrastructure for optimal performance.

## Contributing

Contributions are welcome! Please feel free to submit a Pull Request. For major changes, please open an issue first to discuss what you would like to change.

## License

This project is licensed under the MIT License - see the LICENSE file for details.

## Development

To run the examples:

```bash
cargo run --example usage
```

To run tests:

```bash
cargo test
```