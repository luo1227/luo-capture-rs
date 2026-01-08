use luo_capture::*;

/// 测试捕获区域结构体创建
#[test]
fn test_capture_region_creation() {
    let region = CaptureRegion {
        x: 100,
        y: 100,
        width: 800,
        height: 600,
    };
    assert_eq!(region.x, 100);
    assert_eq!(region.y, 100);
    assert_eq!(region.width, 800);
    assert_eq!(region.height, 600);
}

/// 测试屏幕捕获器创建
#[test]
fn test_screen_capture_creation() {
    let capture = ScreenCapture::new();
    assert!(!capture.is_initialized());
}

/// 测试捕获数据结构
#[test]
fn test_capture_data_creation() {
    use std::time::Instant;

    let data = vec![255u8; 400]; // 10x10 RGBA数据
    let capture_data = CaptureData {
        data,
        width: 10,
        height: 10,
        timestamp: Instant::now(),
    };

    assert_eq!(capture_data.width, 10);
    assert_eq!(capture_data.height, 10);
    assert_eq!(capture_data.data.len(), 400);
}

/// 测试无效捕获区域验证
#[test]
fn test_invalid_capture_region() {
    let mut capture = ScreenCapture::new();
    // 不初始化，直接测试区域验证

    let invalid_region = CaptureRegion {
        x: -1,
        y: 0,
        width: 100,
        height: 100,
    };

    // 由于捕获器未初始化，应该返回错误
    let result = capture.capture(invalid_region, None);
    assert!(result.is_err());
}

/// 测试错误类型显示
#[test]
fn test_error_display() {
    let init_error = CaptureError::InitializationError("测试错误".to_string());
    assert_eq!(format!("{}", init_error), "初始化错误: 测试错误");

    let capture_error = CaptureError::CaptureError("捕获失败".to_string());
    assert_eq!(format!("{}", capture_error), "捕获错误: 捕获失败");

    let invalid_region = CaptureError::InvalidRegion;
    assert_eq!(format!("{}", invalid_region), "无效的捕获区域");
}

/// 测试便捷函数
#[test]
fn test_convenience_functions() {
    // 测试ScreenCapture::new()
    let capture = ScreenCapture::new();
    assert!(!capture.is_initialized());

    // 测试ScreenCapture::default()
    let default_capture = ScreenCapture::default();
    assert!(!default_capture.is_initialized());
}
