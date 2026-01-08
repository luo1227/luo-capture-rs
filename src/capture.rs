use std::time::Instant;
use windows::Win32::Graphics::Direct3D::{D3D_DRIVER_TYPE, D3D_FEATURE_LEVEL};
use windows::Win32::Graphics::Direct3D11::{
    D3D11_CREATE_DEVICE_FLAG, D3D11_MAP, D3D11_MAPPED_SUBRESOURCE, D3D11_SDK_VERSION,
    D3D11_TEXTURE2D_DESC, D3D11_USAGE, D3D11CreateDevice, ID3D11Device, ID3D11DeviceContext,
    ID3D11Texture2D,
};
use windows::Win32::Graphics::Dxgi::Common::DXGI_SAMPLE_DESC;
use windows::Win32::Graphics::Dxgi::{
    DXGI_OUTDUPL_FRAME_INFO, IDXGIAdapter, IDXGIDevice, IDXGIOutput, IDXGIOutput1,
    IDXGIOutputDuplication, IDXGIResource,
};
use windows_core::Interface;

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

// 定义我们自己的Result类型以避免与windows crate冲突
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
            CaptureError::InitializationError(msg) => write!(f, "初始化错误: {}", msg),
            CaptureError::CaptureError(msg) => write!(f, "捕获错误: {}", msg),
            CaptureError::InvalidRegion => write!(f, "无效的捕获区域"),
            CaptureError::ResourceError(msg) => write!(f, "资源错误: {}", msg),
        }
    }
}

impl std::error::Error for CaptureError {}

/// DXGI捕获资源结构体
struct DxgiResources {
    device: ID3D11Device,
    device_context: ID3D11DeviceContext,
    output_duplication: IDXGIOutputDuplication,
}

/// 捕获器结构体
pub struct ScreenCapture {
    is_initialized: bool,
    width: u32,
    height: u32,
    dxgi_resources: Option<DxgiResources>,
}

impl ScreenCapture {
    /// 创建新的捕获器实例
    pub fn new() -> Self {
        Self {
            is_initialized: false,
            width: 0,
            height: 0,
            dxgi_resources: None,
        }
    }

    /// 初始化DXGI资源
    fn initialize_dxgi(&mut self) -> CaptureResult<DxgiResources> {
        // 创建D3D11设备 - 确保支持BGRA格式
        let mut device: Option<ID3D11Device> = None;
        let mut device_context: Option<ID3D11DeviceContext> = None;
        let mut feature_level = D3D_FEATURE_LEVEL::default();

        unsafe {
            D3D11CreateDevice(
                None,
                D3D_DRIVER_TYPE(1), // HARDWARE
                None,
                D3D11_CREATE_DEVICE_FLAG(0x20), // BGRA_SUPPORT
                Some(&[
                    D3D_FEATURE_LEVEL(0xb000), // LEVEL_11_0
                    D3D_FEATURE_LEVEL(0xa000), // LEVEL_10_0
                ]),
                D3D11_SDK_VERSION,
                Some(&mut device),
                Some(&mut feature_level),
                Some(&mut device_context),
            )
            .map_err(|e| {
                CaptureError::InitializationError(format!("创建D3D11设备失败: {:?}", e))
            })?;
        }

        let device = device
            .ok_or_else(|| CaptureError::InitializationError("无法获取D3D11设备".to_string()))?;
        let device_context = device_context.ok_or_else(|| {
            CaptureError::InitializationError("无法获取D3D11设备上下文".to_string())
        })?;

        // 获取DXGI设备
        let dxgi_device: IDXGIDevice = device
            .cast()
            .map_err(|e| CaptureError::InitializationError(format!("获取DXGI设备失败: {:?}", e)))?;

        // 获取DXGI适配器
        let adapter: IDXGIAdapter = unsafe { dxgi_device.GetAdapter() }.map_err(|e| {
            CaptureError::InitializationError(format!("获取DXGI适配器失败: {:?}", e))
        })?;

        // 获取主输出设备（通常是主显示器）
        let output: IDXGIOutput = unsafe { adapter.EnumOutputs(0) }
            .map_err(|e| CaptureError::InitializationError(format!("枚举输出设备失败: {:?}", e)))?;

        // 获取输出描述
        let output_desc = unsafe { output.GetDesc() }
            .map_err(|e| CaptureError::InitializationError(format!("获取输出描述失败: {:?}", e)))?;

        // 转换为IDXGIOutput1以使用DuplicateOutput
        let output1: IDXGIOutput1 = output
            .cast()
            .map_err(|e| CaptureError::InitializationError(format!("转换输出设备失败: {:?}", e)))?;

        // 创建输出复制对象
        let output_duplication: IDXGIOutputDuplication = unsafe {
            output1.DuplicateOutput(&device)
        }
        .map_err(|e| CaptureError::InitializationError(format!("创建输出复制对象失败: {:?}", e)))?;

        // 设置屏幕尺寸
        self.width =
            (output_desc.DesktopCoordinates.right - output_desc.DesktopCoordinates.left) as u32;
        self.height =
            (output_desc.DesktopCoordinates.bottom - output_desc.DesktopCoordinates.top) as u32;

        Ok(DxgiResources {
            device,
            device_context,
            output_duplication,
        })
    }

    /// 初始化捕获器
    /// 使用DXGI技术进行高性能屏幕捕获
    pub fn init(&mut self) -> CaptureResult<()> {
        if self.is_initialized {
            return Ok(());
        }

        let dxgi_resources = self.initialize_dxgi()?;
        self.dxgi_resources = Some(dxgi_resources);
        self.is_initialized = true;

        println!("DXGI初始化成功");
        Ok(())
    }

    /// 检查并重新初始化DXGI资源（如果需要）
    fn ensure_dxgi_resources(&mut self) -> CaptureResult<()> {
        if self.dxgi_resources.is_none() {
            self.is_initialized = false;
            self.init()?;
        }
        Ok(())
    }

    /// 执行DXGI屏幕捕获全屏数据
    fn capture_fullscreen(&mut self) -> CaptureResult<Vec<u8>> {
        // 如果DXGI资源不存在或失效，尝试重新初始化
        if self.dxgi_resources.is_none() {
            return Err(CaptureError::CaptureError("DXGI资源未初始化".to_string()));
        }

        let resources = self.dxgi_resources.as_ref().unwrap();

        // 获取桌面帧
        let mut frame_info = DXGI_OUTDUPL_FRAME_INFO::default();
        let mut desktop_resource: Option<IDXGIResource> = None;

        unsafe {
            resources
                .output_duplication
                .AcquireNextFrame(
                    1000, // 超时时间（毫秒）
                    &mut frame_info,
                    &mut desktop_resource,
                )
                .map_err(|e| CaptureError::CaptureError(format!("获取帧失败: {:?}", e)))?;
        }

        // 获取桌面资源（纹理）
        let desktop_resource = desktop_resource.ok_or_else(|| {
            unsafe {
                resources.output_duplication.ReleaseFrame().ok();
            }
            CaptureError::CaptureError("无法获取桌面资源".to_string())
        })?;

        // 转换为纹理
        let texture: ID3D11Texture2D = desktop_resource.cast().map_err(|e| {
            unsafe {
                resources.output_duplication.ReleaseFrame().ok();
            }
            CaptureError::CaptureError(format!("转换纹理失败: {:?}", e))
        })?;

        // 获取纹理描述
        let mut texture_desc = D3D11_TEXTURE2D_DESC::default();
        unsafe { texture.GetDesc(&mut texture_desc) };

        println!(
            "纹理信息 - 格式: {}, 宽度: {}, 高度: {}, 采样数: {}, BindFlags: {}, Usage: {}",
            texture_desc.Format.0,
            texture_desc.Width,
            texture_desc.Height,
            texture_desc.SampleDesc.Count,
            texture_desc.BindFlags,
            texture_desc.Usage.0
        );

        // 处理多重采样纹理（如果需要）
        let source_texture = if texture_desc.SampleDesc.Count > 1 {
            // 对于多重采样，需要resolve到单采样
            return Err(CaptureError::CaptureError(
                "多重采样纹理暂不支持".to_string(),
            ));
        } else {
            texture
        };

        // 创建全屏staging纹理（复用，避免重复创建开销）
        let staging_texture_desc = D3D11_TEXTURE2D_DESC {
            Width: texture_desc.Width,
            Height: texture_desc.Height,
            MipLevels: 1,
            ArraySize: 1,
            Format: texture_desc.Format,
            SampleDesc: DXGI_SAMPLE_DESC {
                Count: 1,
                Quality: 0,
            },
            Usage: D3D11_USAGE(3), // STAGING
            BindFlags: 0,
            CPUAccessFlags: 0x10000 | 0x20000, // READ | WRITE
            MiscFlags: 0,
        };

        let mut staging_texture: Option<ID3D11Texture2D> = None;
        unsafe {
            resources
                .device
                .CreateTexture2D(&staging_texture_desc, None, Some(&mut staging_texture))
                .map_err(|e| {
                    resources.output_duplication.ReleaseFrame().ok();
                    CaptureError::CaptureError(format!("创建staging纹理失败: {:?}", e))
                })?;
        }

        let staging_texture = staging_texture.ok_or_else(|| {
            unsafe {
                resources.output_duplication.ReleaseFrame().ok();
            }
            CaptureError::CaptureError("无法创建staging纹理".to_string())
        })?;

        // 快速全屏复制到staging纹理
        unsafe {
            resources
                .device_context
                .CopyResource(&staging_texture, &source_texture);
        }

        // 映射staging纹理以读取数据
        let mut mapped_resource = D3D11_MAPPED_SUBRESOURCE::default();
        unsafe {
            resources
                .device_context
                .Map(
                    &staging_texture,
                    0,
                    D3D11_MAP(1), // READ
                    0,
                    Some(&mut mapped_resource),
                )
                .map_err(|e| {
                    println!("Map调用失败，错误码: {:?}", e);
                    println!(
                        "纹理格式: {}, CPU访问标志: {}",
                        staging_texture_desc.Format.0, staging_texture_desc.CPUAccessFlags
                    );
                    let _ = resources.output_duplication.ReleaseFrame();
                    CaptureError::CaptureError(format!("映射staging纹理失败: {:?}", e))
                })?;
        }

        // 复制全屏数据
        let row_pitch = mapped_resource.RowPitch as usize;
        let total_size = (texture_desc.Height as usize) * row_pitch;

        let mut fullscreen_data = vec![0u8; total_size];
        unsafe {
            std::ptr::copy_nonoverlapping(
                mapped_resource.pData as *const u8,
                fullscreen_data.as_mut_ptr(),
                total_size,
            );
        }

        // 取消映射
        unsafe {
            resources.device_context.Unmap(&staging_texture, 0);
        }

        // 释放帧
        unsafe {
            resources.output_duplication.ReleaseFrame().ok();
        }

        println!(
            "DXGI全屏捕获成功，屏幕大小: {}x{}, 数据大小: {} bytes",
            texture_desc.Width,
            texture_desc.Height,
            fullscreen_data.len()
        );
        Ok(fullscreen_data)
    }

    /// 捕获指定区域的屏幕截图
    /// 使用DXGI技术，理论上可以达到1ms一帧的高性能
    /// 可选参数 save_path: 指定保存路径时会将截图保存为PNG文件
    pub fn capture(
        &mut self,
        region: CaptureRegion,
        save_path: Option<&str>,
    ) -> CaptureResult<CaptureData> {
        // 确保DXGI资源可用
        self.ensure_dxgi_resources()?;

        // 验证区域参数
        if region.x < 0 || region.y < 0 || region.width == 0 || region.height == 0 {
            return Err(CaptureError::InvalidRegion);
        }

        if (region.x + region.width as i32) as u32 > self.width
            || (region.y + region.height as i32) as u32 > self.height
        {
            return Err(CaptureError::InvalidRegion);
        }

        // 使用DXGI捕获全屏，失败时尝试重新初始化
        let fullscreen_data = match self.capture_fullscreen() {
            Ok(data) => data,
            Err(e) => {
                // 如果DXGI捕获失败，尝试重新初始化
                println!("DXGI捕获失败，尝试重新初始化: {}", e);
                self.dxgi_resources = None;
                self.ensure_dxgi_resources()?;
                self.capture_fullscreen()?
            }
        };

        // 从全屏数据中提取指定区域的数据
        // DXGI格式通常是BGRA，每个像素4字节
        let bytes_per_pixel = 4; // BGRA格式
        let screen_row_pitch = self.width as usize * bytes_per_pixel; // 全屏一行数据的字节数

        // 计算区域数据的总大小
        let region_data_size = region.width as usize * region.height as usize * bytes_per_pixel;
        let mut region_data = vec![0u8; region_data_size];

        // 从全屏数据中复制指定区域
        for y in 0..region.height as usize {
            let src_row_start = ((region.y as usize + y) * screen_row_pitch)
                + (region.x as usize * bytes_per_pixel);
            let dst_row_start = y * (region.width as usize * bytes_per_pixel);
            let row_size = region.width as usize * bytes_per_pixel;

            // 确保不越界
            if src_row_start + row_size <= fullscreen_data.len() {
                region_data[dst_row_start..dst_row_start + row_size]
                    .copy_from_slice(&fullscreen_data[src_row_start..src_row_start + row_size]);
            }
        }

        // 如果提供了保存路径，则保存为PNG文件
        if let Some(path) = save_path {
            use image::{ImageBuffer, RgbaImage};

            // 将BGRA数据转换为RGBA图像缓冲区
            // DXGI的BGRA格式需要转换为image crate的RGBA格式
            let mut rgba_data = vec![0u8; region_data_size];
            for i in 0..region_data_size / 4 {
                let bgra_start = i * 4;
                let rgba_start = i * 4;
                // BGRA -> RGBA: B,G,R,A -> R,G,B,A
                rgba_data[rgba_start] = region_data[bgra_start + 2]; // R <- B
                rgba_data[rgba_start + 1] = region_data[bgra_start + 1]; // G <- G
                rgba_data[rgba_start + 2] = region_data[bgra_start]; // B <- R
                rgba_data[rgba_start + 3] = region_data[bgra_start + 3]; // A <- A
            }

            let img: RgbaImage = ImageBuffer::from_raw(region.width, region.height, rgba_data)
                .ok_or_else(|| CaptureError::CaptureError("创建图像缓冲区失败".to_string()))?;

            // 保存为PNG
            img.save(path)
                .map_err(|e| CaptureError::CaptureError(format!("保存PNG文件失败: {}", e)))?;
        }

        let result = CaptureData {
            data: region_data,
            width: region.width,
            height: region.height,
            timestamp: Instant::now(),
        };

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

impl Drop for ScreenCapture {
    fn drop(&mut self) {
        // DXGI资源会在离开作用域时自动释放
        // GDI资源在每次捕获后都会被清理
    }
}

/// 便捷函数：初始化捕获器
pub fn init() -> CaptureResult<ScreenCapture> {
    let mut capture = ScreenCapture::new();
    capture.init()?;
    Ok(capture)
}

/// 便捷函数：捕获屏幕区域
/// 可选参数 save_path: 指定保存路径时会将截图保存为PNG文件
pub fn capture(
    capture: &mut ScreenCapture,
    region: CaptureRegion,
    save_path: Option<&str>,
) -> CaptureResult<CaptureData> {
    capture.capture(region, save_path)
}
