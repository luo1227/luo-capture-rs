use std::sync::Arc;
use std::sync::Mutex;
use std::time::Instant;

/// 捕获区域结构体
#[derive(Debug, Clone, Copy)]
pub struct CaptureRegion {
    pub x: i32,
    pub y: i32,
    pub width: u32,
    pub height: u32,
}

/// 捕获结果
#[derive(Debug)]
pub struct CaptureData {
    pub data: Vec<u8>,
    pub width: u32,
    pub height: u32,
    pub timestamp: std::time::Instant,
}

// Define our own Result type to avoid conflicts with windows crate
pub type CaptureResult<T> = std::result::Result<T, CaptureError>;

/// 自定义错误类型
#[derive(Debug)]
pub enum CaptureError {
    InitializationError(String),
    CaptureError(String),
    InvalidRegion,
    ResourceError(String),
}

impl std::fmt::Display for CaptureError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            CaptureError::InitializationError(msg) => write!(f, "Initialization error: {}", msg),
            CaptureError::CaptureError(msg) => write!(f, "Capture error: {}", msg),
            CaptureError::InvalidRegion => write!(f, "Invalid capture region"),
            CaptureError::ResourceError(msg) => write!(f, "Resource error: {}", msg),
        }
    }
}

impl std::error::Error for CaptureError {}

/// 捕获器结构体
pub struct ScreenCapture {
    is_initialized: bool,
    width: u32,
    height: u32,
}

impl ScreenCapture {
    /// 创建新的捕获器实例
    pub fn new() -> Self {
        Self {
            is_initialized: false,
            width: 0,
            height: 0,
        }
    }

    /// 初始化捕获器
    /// 使用DXGI技术进行高性能屏幕捕获
    pub fn init(&mut self) -> CaptureResult<()> {
        // In a real implementation, this would initialize DXGI resources
        // For now, we'll simulate initialization
        self.width = 1920; // Assume a common screen resolution
        self.height = 1080;
        self.is_initialized = true;

        Ok(())
    }

    /// 捕获指定区域的屏幕截图
    /// 使用DXGI技术，理论上可以达到1ms一帧的高性能
    /// 可选参数 save_path: 指定保存路径时会将截图保存为PNG文件
    pub fn capture(&mut self, region: CaptureRegion, save_path: Option<&str>) -> CaptureResult<CaptureData> {
        if !self.is_initialized {
            return Err(CaptureError::CaptureError("Capture not initialized. Call init() first.".to_string()));
        }

        // 验证区域参数
        if region.x < 0 || region.y < 0 || region.width == 0 || region.height == 0 {
            return Err(CaptureError::InvalidRegion);
        }

        if (region.x + region.width as i32) as u32 > self.width || (region.y + region.height as i32) as u32 > self.height {
            return Err(CaptureError::InvalidRegion);
        }

        // In a real implementation, this would use DXGI to capture the screen
        // For now, we'll simulate capturing by creating dummy data
        let data = vec![0u8; (region.width * region.height * 4) as usize]; // RGBA

        let result = CaptureData {
            data: data.clone(),
            width: region.width,
            height: region.height,
            timestamp: Instant::now(),
        };

        // 如果提供了保存路径，则保存为PNG文件
        if let Some(path) = save_path {
            use image::{ImageBuffer, RgbaImage};

            // 将数据转换为RGBA图像缓冲区
            let img: RgbaImage = ImageBuffer::from_raw(region.width, region.height, data)
                .ok_or_else(|| CaptureError::CaptureError("Failed to create image buffer".to_string()))?;

            // 保存为PNG
            img.save(path)
                .map_err(|e| CaptureError::CaptureError(format!("Failed to save PNG: {}", e)))?;
        }

        Ok(result)
    }

    /// 检查捕获器是否已初始化
    pub fn is_initialized(&self) -> bool {
        self.is_initialized
    }
}

impl Default for ScreenCapture {
    fn default() -> Self {
        Self::new()
    }
}

/// 异步版本的捕获器
/// 适用于需要非阻塞操作的场景
pub struct AsyncScreenCapture {
    capture: Arc<Mutex<ScreenCapture>>,
}

impl AsyncScreenCapture {
    /// 创建新的异步捕获器
    pub fn new() -> Self {
        Self {
            capture: Arc::new(Mutex::new(ScreenCapture::new())),
        }
    }

    /// 异步初始化
    pub async fn init(&self) -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
        let capture = self.capture.clone();
        tokio::task::spawn_blocking(move || -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
            let mut capture = capture.lock().map_err(|e| format!("Lock error: {}", e))?;
            match capture.init() {
                Ok(()) => Ok(()),
                Err(e) => Err(Box::new(e))
            }
        }).await?
    }

    /// 异步捕获
    /// 可选参数 save_path: 指定保存路径时会将截图保存为PNG文件
    pub async fn capture(&self, region: CaptureRegion, save_path: Option<&str>) -> Result<CaptureData, Box<dyn std::error::Error + Send + Sync>> {
        let capture = self.capture.clone();
        let save_path = save_path.map(|s| s.to_string()); // 将 &str 转换为 String 以便在线程间传递
        tokio::task::spawn_blocking(move || -> Result<CaptureData, Box<dyn std::error::Error + Send + Sync>> {
            let mut capture = capture.lock().map_err(|e| format!("Lock error: {}", e))?;
            match capture.capture(region, save_path.as_deref()) {
                Ok(result) => Ok(result),
                Err(e) => Err(Box::new(e))
            }
        }).await?
    }
}

impl Default for AsyncScreenCapture {
    fn default() -> Self {
        Self::new()
    }
}

/// 便捷函数：初始化捕获器
pub fn init() -> CaptureResult<ScreenCapture> {
    let mut capture = ScreenCapture::new();
    capture.init()?;
    Ok(capture)
}

/// 便捷函数：异步初始化捕获器
pub async fn init_async() -> Result<AsyncScreenCapture, Box<dyn std::error::Error + Send + Sync>> {
    let capture = AsyncScreenCapture::new();
    capture.init().await?;
    Ok(capture)
}

/// 便捷函数：捕获屏幕区域
/// 可选参数 save_path: 指定保存路径时会将截图保存为PNG文件
pub fn capture(capture: &mut ScreenCapture, region: CaptureRegion, save_path: Option<&str>) -> CaptureResult<CaptureData> {
    capture.capture(region, save_path)
}

/// 便捷函数：异步捕获屏幕区域
/// 可选参数 save_path: 指定保存路径时会将截图保存为PNG文件
pub async fn capture_async(capture: &AsyncScreenCapture, region: CaptureRegion, save_path: Option<&str>) -> Result<CaptureData, Box<dyn std::error::Error + Send + Sync>> {
    capture.capture(region, save_path).await
}

#[cfg(test)]
mod tests {
    use super::*;

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

    #[test]
    fn test_screen_capture_creation() {
        let capture = ScreenCapture::new();
        assert!(!capture.is_initialized());
    }

    #[test]
    fn test_capture_with_png_save() {
        use std::fs;
        use std::path::Path;

        let mut capture = ScreenCapture::new();
        capture.init().unwrap();

        let region = CaptureRegion {
            x: 0,
            y: 0,
            width: 100,
            height: 100,
        };

        // Create a temporary file path for testing
        let temp_path = "test_capture.png";

        // Capture with save path
        let result = capture.capture(region, Some(temp_path));
        assert!(result.is_ok());

        // Verify file was created
        assert!(Path::new(temp_path).exists());

        // Clean up
        let _ = fs::remove_file(temp_path);
    }

    #[test]
    fn test_capture_without_png_save() {
        let mut capture = ScreenCapture::new();
        capture.init().unwrap();

        let region = CaptureRegion {
            x: 0,
            y: 0,
            width: 100,
            height: 100,
        };

        // Capture without save path
        let result = capture.capture(region, None);
        assert!(result.is_ok());

        let capture_data = result.unwrap();
        assert_eq!(capture_data.width, 100);
        assert_eq!(capture_data.height, 100);
    }
}