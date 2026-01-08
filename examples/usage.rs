use luo_capture::*;
use std::time::Instant;

fn main() {
    // 捕获模块的使用示例，包含计时功能

    // 方法1：同步初始化和捕获
    println!("正在初始化捕获模块...");
    let start_time = Instant::now();
    let mut screen_capture = init().expect("初始化捕获失败");
    let init_duration = start_time.elapsed();
    println!(
        "捕获模块初始化成功！耗时: {:.3}ms",
        init_duration.as_secs_f64() * 1000.0
    );

    // 定义捕获区域 (x, y, width, height)
    let region = CaptureRegion {
        x: 0,
        y: 0,
        width: 800,
        height: 600,
    };

    println!("正在捕获区域: {:?}", region);
    let start_time = Instant::now();
    match screen_capture.capture(region, None) {
        Ok(capture_data) => {
            let capture_duration = start_time.elapsed();
            println!(
                "捕获成功！耗时: {:.3}ms",
                capture_duration.as_secs_f64() * 1000.0
            );
            println!(
                "宽度: {}, 高度: {}, 数据大小: {} 字节",
                capture_data.width,
                capture_data.height,
                capture_data.data.len()
            );
        }
        Err(e) => {
            let capture_duration = start_time.elapsed();
            eprintln!(
                "捕获在 {:.3}ms 后失败: {}",
                capture_duration.as_secs_f64() * 1000.0,
                e
            );
        }
    }

    // 方法2：使用便捷函数
    println!("\n正在使用便捷函数...");
    let mut capture_instance = ScreenCapture::new();
    let start_time = Instant::now();
    capture_instance.init().expect("初始化捕获失败");
    let init_duration = start_time.elapsed();
    println!(
        "捕获实例已初始化！耗时: {:.3}ms",
        init_duration.as_secs_f64() * 1000.0
    );

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
            println!(
                "便捷函数捕获成功！耗时: {:.3}ms",
                capture_duration.as_secs_f64() * 1000.0
            );
            println!(
                "宽度: {}, 高度: {}, 数据大小: {} 字节",
                capture_data.width,
                capture_data.height,
                capture_data.data.len()
            );
        }
        Err(e) => {
            let capture_duration = start_time.elapsed();
            eprintln!(
                "便捷函数捕获在 {:.3}ms 后失败: {}",
                capture_duration.as_secs_f64() * 1000.0,
                e
            );
        }
    }

    // 方法3：测试PNG保存功能
    println!("\n正在测试PNG保存功能...");
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
            println!(
                "带PNG保存的捕获成功！耗时: {:.3}ms",
                capture_duration.as_secs_f64() * 1000.0
            );
            println!(
                "宽度: {}, 高度: {}, 数据大小: {} 字节",
                capture_data.width,
                capture_data.height,
                capture_data.data.len()
            );
            println!("PNG已保存到: {}", save_path);

            // 检查文件是否存在
            if std::path::Path::new(save_path).exists() {
                println!("✓ PNG文件创建成功");
            } else {
                println!("✗ PNG文件未创建");
            }
        }
        Err(e) => {
            let capture_duration = start_time.elapsed();
            eprintln!(
                "带PNG保存的捕获在 {:.3}ms 后失败: {}",
                capture_duration.as_secs_f64() * 1000.0,
                e
            );
        }
    }
}
