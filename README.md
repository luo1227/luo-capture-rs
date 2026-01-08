# luo-capture-rs

一个使用DXGI技术的高性能屏幕捕获模块，专为Windows平台设计。

## 特性

- **高性能**: 针对1ms帧捕获进行了优化（理论值）
- **DXGI技术**: 使用DirectX Graphics Infrastructure实现高效屏幕捕获
- **区域捕获**: 捕获指定的屏幕区域而非全屏
- **同步和异步API**: 同时提供同步和异步接口
- **线程安全**: 可安全地在多个线程中使用
- **错误处理**: 全面的错误处理和自定义错误类型

## 安装

在您的 `Cargo.toml` 中添加以下内容：

```toml
[dependencies]
luo-capture-rs = "0.1.0"
```

## 使用方法

### 基本同步捕获

```rust
use luo_capture_rs::capture::*;

fn main() {
    // 初始化捕获模块
    let mut capture = init().expect("初始化捕获失败");

    // 定义捕获区域 (x, y, width, height)
    let region = CaptureRegion {
        x: 0,
        y: 0,
        width: 800,
        height: 600,
    };

    // 捕获该区域
    match capture.capture(region, None) {
        Ok(capture_data) => {
            println!("成功捕获 {}x{} 图像，包含 {} 字节数据",
                     capture_data.width, capture_data.height, capture_data.data.len());
        },
        Err(e) => eprintln!("捕获失败: {}", e),
    }
}
```

### 异步捕获

```rust
use luo_capture_rs::capture::*;

#[tokio::main]
async fn main() {
    // 异步初始化捕获模块
    let async_capture = init_async().await.expect("异步初始化捕获失败");

    // 定义捕获区域
    let region = CaptureRegion {
        x: 0,
        y: 0,
        width: 800,
        height: 600,
    };

    // 异步捕获该区域
    match async_capture.capture(region, None).await {
        Ok(capture_data) => {
            println!("成功捕获 {}x{} 图像，包含 {} 字节数据",
                     capture_data.width, capture_data.height, capture_data.data.len());
        },
        Err(e) => eprintln!("捕获失败: {}", e),
    }
}
```

### 计时示例

```rust
use luo_capture_rs::capture::*;
use std::time::Instant;

fn main() {
    // 测量初始化时间
    let start_time = Instant::now();
    let mut capture = init().expect("初始化捕获失败");
    let init_duration = start_time.elapsed();
    println!("初始化耗时: {:.3}ms", init_duration.as_secs_f64() * 1000.0);

    // 定义捕获区域
    let region = CaptureRegion {
        x: 0,
        y: 0,
        width: 800,
        height: 600,
    };

    // 测量捕获时间
    let start_time = Instant::now();
    match capture.capture(region, None) {
        Ok(capture_data) => {
            let capture_duration = start_time.elapsed();
            println!("捕获耗时: {:.3}ms", capture_duration.as_secs_f64() * 1000.0);
            println!("成功捕获 {}x{} 图像，包含 {} 字节数据",
                     capture_data.width, capture_data.height, capture_data.data.len());
        },
        Err(e) => eprintln!("捕获失败: {}", e),
    }
}
```

### PNG保存功能

```rust
use luo_capture_rs::capture::*;

fn main() {
    let mut capture = init().expect("初始化捕获失败");

    let region = CaptureRegion {
        x: 0,
        y: 0,
        width: 800,
        height: 600,
    };

    // 捕获并保存为PNG文件
    match capture.capture(region, Some("screenshot.png")) {
        Ok(capture_data) => {
            println!("成功捕获并保存 {}x{} 图像到 screenshot.png",
                     capture_data.width, capture_data.height);
        },
        Err(e) => eprintln!("捕获失败: {}", e),
    }

    // 只捕获不保存（传递None）
    match capture.capture(region, None) {
        Ok(capture_data) => {
            println!("成功捕获 {}x{} 图像（不保存到文件）",
                     capture_data.width, capture_data.height);
        },
        Err(e) => eprintln!("捕获失败: {}", e),
    }
}
```

## API 概览

### 核心类型

- `CaptureRegion`: 定义要捕获的屏幕区域 (x, y, width, height)
- `CaptureData`: 包含捕获的图像数据、尺寸和时间戳
- `ScreenCapture`: 主要的捕获接口
- `AsyncScreenCapture`: 用于非阻塞操作的异步包装器

### 主要函数

- `init()`: 同步初始化捕获模块
- `init_async()`: 异步初始化捕获模块
- `capture()`: 同步捕获屏幕区域（支持可选的PNG保存）
- `capture_async()`: 异步捕获屏幕区域（支持可选的PNG保存）

## 错误处理

该模块通过 `CaptureError` 枚举提供全面的错误处理：

- `InitializationError`: 初始化过程中的错误
- `CaptureError`: 捕获操作过程中的错误
- `InvalidRegion`: 无效的捕获区域参数
- `ResourceError`: 资源分配或管理错误

## 性能说明

- 该捕获模块专为高性能场景设计
- 基于区域的捕获减少内存使用和处理时间
- 异步API允许非阻塞操作
- 适当的资源管理防止内存泄漏

## 平台支持

目前支持具有DXGI支持的Windows平台。该实现使用DirectX Graphics Infrastructure以获得最佳性能。

## 贡献

欢迎贡献！请随时提交拉取请求。对于重大更改，请先打开一个问题来讨论您想要更改的内容。

## 许可证

本项目采用MIT许可证授权 - 有关详细信息，请参阅LICENSE文件。

## 开发

运行示例：

```bash
cargo run --example usage
```

运行测试：

```bash
cargo test
```