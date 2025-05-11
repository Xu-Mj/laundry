use lazy_static::lazy_static;
use network_interface::{NetworkInterface, NetworkInterfaceConfig};
use serde::{Deserialize, Serialize};
use std::process::Command;
use std::sync::{Arc, Mutex};
use std::time::Instant;
use std::time::{Duration, SystemTime};
use tracing::{debug, info, warn};
use uuid::Uuid;

// 缓存过期时间（24小时）
const CACHE_EXPIRY_DURATION: Duration = Duration::from_secs(24 * 60 * 60);

// 缓存设备信息，避免频繁获取
lazy_static! {
    static ref DEVICE_INFO_CACHE: Arc<Mutex<Option<(DeviceInfo, SystemTime)>>> =
        Arc::new(Mutex::new(None));
    static ref HARDWARE_FINGERPRINT_CACHE: Arc<Mutex<Option<(String, SystemTime)>>> =
        Arc::new(Mutex::new(None));
    static ref MAC_ADDRESS_CACHE: Arc<Mutex<Option<(Option<String>, SystemTime)>>> =
        Arc::new(Mutex::new(None));
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct DeviceInfo {
    pub device_id: String,
    pub device_name: String,
    pub device_type: String,
    pub mac_address: Option<String>,
    pub hardware_fingerprint: String,
}

impl DeviceInfo {
    pub fn get() -> Self {
        info!("开始获取设备信息");
        let start = Instant::now();

        // 检查缓存中是否有有效的设备信息
        if let Some((cached_info, cache_time)) = DEVICE_INFO_CACHE.lock().unwrap().as_ref() {
            if SystemTime::now()
                .duration_since(*cache_time)
                .unwrap_or(Duration::from_secs(0))
                < CACHE_EXPIRY_DURATION
            {
                debug!("使用缓存的设备信息，跳过获取过程");
                return cached_info.clone();
            }
        }

        let mac_start = Instant::now();
        let mac_address = get_mac_address();
        debug!("获取MAC地址耗时: {:?}", mac_start.elapsed());

        let id_start = Instant::now();
        let device_id = get_or_create_device_id(&mac_address);
        debug!("获取设备ID耗时: {:?}", id_start.elapsed());

        let name_start = Instant::now();
        let device_name = get_device_name();
        debug!("获取设备名称耗时: {:?}", name_start.elapsed());

        let type_start = Instant::now();
        let device_type = get_device_type();
        debug!("获取设备类型耗时: {:?}", type_start.elapsed());

        let fingerprint_start = Instant::now();
        let hardware_fingerprint = generate_hardware_fingerprint();
        debug!("生成硬件指纹耗时: {:?}", fingerprint_start.elapsed());

        let device_info = Self {
            device_id,
            device_name,
            device_type,
            mac_address,
            hardware_fingerprint,
        };

        // 更新缓存
        *DEVICE_INFO_CACHE.lock().unwrap() = Some((device_info.clone(), SystemTime::now()));

        info!("设备信息获取完成，总耗时: {:?}", start.elapsed());
        debug!("设备信息: {:?}", device_info);
        device_info
    }
}

// 获取或创建设备ID，优先使用MAC地址，无法获取时使用UUID
fn get_or_create_device_id(mac: &Option<String>) -> String {
    debug!("开始获取或创建设备ID");
    let start = Instant::now();

    // 优先使用MAC地址作为设备ID
    if let Some(mac) = mac {
        // 移除MAC地址中的冒号，使其更适合作为ID
        let mac_id = mac.replace(":", "").replace("-", "");
        debug!("使用MAC地址创建设备ID，耗时: {:?}", start.elapsed());
        return mac_id;
    }

    // 如果无法获取MAC地址，则使用UUID作为备选方案
    let uuid = Uuid::new_v4().to_string();
    debug!("使用UUID创建设备ID，耗时: {:?}", start.elapsed());
    uuid
}

// 获取设备名称
fn get_device_name() -> String {
    debug!("开始获取设备名称");
    let start = Instant::now();

    let hostname = hostname::get()
        .map(|h| h.to_string_lossy().to_string())
        .unwrap_or_else(|_| {
            warn!("无法获取主机名，使用默认值");
            String::from("Unknown")
        });

    debug!("获取设备名称完成，耗时: {:?}", start.elapsed());
    hostname
}

// 获取设备类型（Windows, macOS, Linux等）
fn get_device_type() -> String {
    debug!("开始获取设备类型");
    let start = Instant::now();

    let os = std::env::consts::OS.to_string();
    let arch = std::env::consts::ARCH.to_string();

    let device_type = format!("{}-{}", os, arch);
    debug!("获取设备类型完成，耗时: {:?}", start.elapsed());
    device_type
}

// 获取MAC地址 - 使用network-interface库替代PowerShell命令
fn get_mac_address() -> Option<String> {
    debug!("开始获取MAC地址");
    let start = Instant::now();

    // 检查缓存中是否有有效的MAC地址
    if let Some((cached_mac, cache_time)) = MAC_ADDRESS_CACHE.lock().unwrap().as_ref() {
        if SystemTime::now()
            .duration_since(*cache_time)
            .unwrap_or(Duration::from_secs(0))
            < CACHE_EXPIRY_DURATION
        {
            debug!("使用缓存的MAC地址，跳过获取过程");
            return cached_mac.clone();
        }
    }

    let result = get_mac_address_internal();

    // 更新缓存
    *MAC_ADDRESS_CACHE.lock().unwrap() = Some((result.clone(), SystemTime::now()));

    debug!("获取MAC地址完成，总耗时: {:?}", start.elapsed());
    result
}

// 内部函数，实际获取MAC地址的逻辑
fn get_mac_address_internal() -> Option<String> {
    let interfaces = match NetworkInterface::show() {
        Ok(interfaces) => interfaces,
        Err(e) => {
            warn!("获取网络接口失败: {:?}", e);
            return None;
        }
    };

    // 查找活跃的网络接口
    for interface in interfaces {
        // 跳过回环接口
        if interface.name.to_lowercase().contains("loopback")
            || interface.name.to_lowercase() == "lo"
        {
            continue;
        }

        // 检查接口是否有MAC地址
        if let Some(mac) = interface.mac_addr {
            if !mac.is_empty() {
                return Some(mac);
            }
        }
    }

    // 如果使用network-interface库失败，尝试使用备用方法
    #[cfg(target_os = "windows")]
    {
        // 在Windows上使用ipconfig命令作为备用方法
        debug!("使用备用方法获取MAC地址");
        let output = Command::new("powershell")
            .args(["-Command", "Get-NetAdapter | Where-Object Status -eq 'Up' | Select-Object -First 1 -ExpandProperty MacAddress"])
            .output()
            .ok()?;

        if output.status.success() {
            let mac = String::from_utf8_lossy(&output.stdout).trim().to_string();
            if !mac.is_empty() {
                return Some(mac);
            }
        }
    }

    None
}

// 生成硬件指纹 - 优化版本
fn generate_hardware_fingerprint() -> String {
    info!("开始生成硬件指纹");
    let start = Instant::now();

    // 检查缓存中是否有有效的硬件指纹
    if let Some((cached_fingerprint, cache_time)) =
        HARDWARE_FINGERPRINT_CACHE.lock().unwrap().as_ref()
    {
        if SystemTime::now()
            .duration_since(*cache_time)
            .unwrap_or(Duration::from_secs(0))
            < CACHE_EXPIRY_DURATION
        {
            debug!("使用缓存的硬件指纹，跳过生成过程");
            return cached_fingerprint.clone();
        }
    }

    let fingerprint = generate_hardware_fingerprint_internal();

    // 更新缓存
    *HARDWARE_FINGERPRINT_CACHE.lock().unwrap() = Some((fingerprint.clone(), SystemTime::now()));

    info!("硬件指纹生成完成，总耗时: {:?}", start.elapsed());
    fingerprint
}

// 内部函数，实际生成硬件指纹的逻辑
fn generate_hardware_fingerprint_internal() -> String {
    let mut fingerprint = String::new();

    // 添加CPU信息 - 使用更高效的方法
    #[cfg(target_os = "windows")]
    {
        debug!("开始获取CPU信息");
        let cpu_start = Instant::now();

        // 使用更简单的命令获取CPU ID
        let output = Command::new("powershell")
            .args([
                "-Command",
                "(Get-CimInstance -Class Win32_Processor).ProcessorId",
            ])
            .output();

        if let Ok(output) = output {
            if output.status.success() {
                let cpu_id = String::from_utf8_lossy(&output.stdout).trim().to_string();
                if !cpu_id.is_empty() {
                    fingerprint.push_str(&format!("CPU:{}", cpu_id));
                    debug!("成功获取CPU信息，耗时: {:?}", cpu_start.elapsed());
                }
            }
        }

        // 如果没有获取到CPU信息，添加设备名称和随机数
        if fingerprint.is_empty() {
            let device_name = get_device_name();
            let random_id = Uuid::new_v4().to_string();
            fingerprint = format!("FALLBACK:{}|{}", device_name, random_id);
            debug!("使用备用方案生成硬件指纹");
        }
    }

    // 非Windows平台使用备用方案
    #[cfg(not(target_os = "windows"))]
    {
        let device_name = get_device_name();
        let random_id = Uuid::new_v4().to_string();
        fingerprint = format!("FALLBACK:{}|{}", device_name, random_id);
    }

    fingerprint
}
