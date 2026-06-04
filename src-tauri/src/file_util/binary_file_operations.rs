use std::fs;
use std::path::Path;
use tauri::command;

/// 二进制文件信息
#[derive(serde::Serialize, serde::Deserialize, Debug)]
pub struct BinaryFileInfo {
    pub path: String,
    pub size: u64,
    pub file_type: String,
    pub extension: String,
    pub is_custom_format: bool,
    pub metadata: Option<CustomMetadata>,
}

/// 私有格式元数据
#[derive(serde::Serialize, serde::Deserialize, Debug, Clone)]
pub struct CustomMetadata {
    pub format_name: String,
    pub version: String,
    pub custom_headers: Vec<(String, String)>,
}

/// 文件类型枚举
#[derive(Debug)]
pub enum FileType {
    Image,
    Audio,
    Video,
    Archive,
    Document,
    Executable,
    #[allow(dead_code)]
    Custom(String),
    #[allow(dead_code)]
    Unknown,
}

/// 通过魔数检测文件类型
fn detect_file_type_by_magic(bytes: &[u8]) -> FileType {
    if bytes.len() < 4 {
        return FileType::Unknown;
    }

    match &bytes[0..4] {
        // PNG: 89 50 4E 47
        [0x89, 0x50, 0x4E, 0x47] => FileType::Image,
        // JPEG: FF D8 FF
        [0xFF, 0xD8, 0xFF, _] => FileType::Image,
        // GIF: 47 49 46 38
        [0x47, 0x49, 0x46, 0x38] => FileType::Image,
        // WEBP: 52 49 46 46 (RIFF)
        [0x52, 0x49, 0x46, 0x46] => FileType::Image,
        // PDF: 25 50 44 46
        [0x25, 0x50, 0x44, 0x46] => FileType::Document,
        // ZIP/APK/DOCX: 50 4B 03 04
        [0x50, 0x4B, 0x03, 0x04] => FileType::Archive,
        // MP3 (ID3): 49 44 33
        [0x49, 0x44, 0x33, _] => FileType::Audio,
        // MP4/M4A: 00 00 00 18 66 74 79 70 (ftyp)
        _ if bytes.len() >= 8 && &bytes[4..8] == b"ftyp" => FileType::Video,
        // EXE: 4D 5A
        [0x4D, 0x5A, _, _] => FileType::Executable,
        _ => FileType::Unknown,
    }
}

/// 通过扩展名获取文件类型
#[allow(dead_code)]
fn get_file_type_by_extension(extension: &str) -> FileType {
    match extension.to_lowercase().as_str() {
        "png" | "jpg" | "jpeg" | "gif" | "bmp" | "webp" | "svg" | "ico" => FileType::Image,
        "mp3" | "wav" | "flac" | "aac" | "ogg" | "wma" => FileType::Audio,
        "mp4" | "avi" | "mkv" | "mov" | "wmv" | "flv" | "webm" => FileType::Video,
        "zip" | "rar" | "7z" | "tar" | "gz" | "bz2" => FileType::Archive,
        "pdf" | "doc" | "docx" | "xls" | "xlsx" | "ppt" | "pptx" => FileType::Document,
        "exe" | "dll" | "so" | "dylib" => FileType::Executable,
        _ => FileType::Unknown,
    }
}

/// 获取文件类型字符串
fn file_type_to_string(file_type: &FileType) -> String {
    match file_type {
        FileType::Image => "image".to_string(),
        FileType::Audio => "audio".to_string(),
        FileType::Video => "video".to_string(),
        FileType::Archive => "archive".to_string(),
        FileType::Document => "document".to_string(),
        FileType::Executable => "executable".to_string(),
        FileType::Custom(name) => format!("custom:{}", name),
        FileType::Unknown => "unknown".to_string(),
    }
}

/// 检查是否为私有/自定义格式
fn is_custom_format(extension: &str) -> bool {
    let known_extensions = [
        "png", "jpg", "jpeg", "gif", "bmp", "webp", "svg", "ico",
        "mp3", "wav", "flac", "aac", "ogg", "wma",
        "mp4", "avi", "mkv", "mov", "wmv", "flv", "webm",
        "zip", "rar", "7z", "tar", "gz", "bz2",
        "pdf", "doc", "docx", "xls", "xlsx", "ppt", "pptx",
        "exe", "dll", "so", "dylib",
        "txt", "md", "json", "xml", "csv", "html", "css", "js", "ts",
    ];
    !known_extensions.contains(&extension.to_lowercase().as_str())
}

#[command]
pub fn create_binary_file(path: &str, data_base64: &str) -> Result<String, String> {
    let path_obj = Path::new(path);

    if path_obj.exists() {
        return Err(format!("文件已存在: {}", path));
    }

    if let Some(parent) = path_obj.parent() {
        if !parent.exists() {
            fs::create_dir_all(parent).map_err(|e| format!("创建目录失败: {}", e))?;
        }
    }

    let bytes = base64_decode(data_base64)?;
    fs::write(path_obj, &bytes).map_err(|e| format!("创建文件失败: {}", e))?;
    Ok(format!("二进制文件创建成功: {}", path))
}

#[command]
pub fn delete_binary_file(path: &str) -> Result<String, String> {
    let path_obj = Path::new(path);

    if !path_obj.exists() {
        return Err(format!("文件不存在: {}", path));
    }

    if !path_obj.is_file() {
        return Err(format!("路径不是文件: {}", path));
    }

    fs::remove_file(path_obj).map_err(|e| format!("删除文件失败: {}", e))?;
    Ok(format!("二进制文件删除成功: {}", path))
}

#[command]
pub fn read_binary_file(path: &str) -> Result<BinaryFileInfo, String> {
    let path_obj = Path::new(path);

    if !path_obj.exists() {
        return Err(format!("文件不存在: {}", path));
    }

    if !path_obj.is_file() {
        return Err(format!("路径不是文件: {}", path));
    }

    let metadata = fs::metadata(path_obj).map_err(|e| format!("获取文件元数据失败: {}", e))?;
    let size = metadata.len();
    let extension = path_obj.extension()
        .and_then(|e| e.to_str())
        .unwrap_or("")
        .to_string();

    let bytes = fs::read(path_obj).map_err(|e| format!("读取文件失败: {}", e))?;
    let file_type = detect_file_type_by_magic(&bytes);
    let file_type_str = file_type_to_string(&file_type);

    let is_custom = is_custom_format(&extension);

    let custom_metadata = if is_custom {
        Some(CustomMetadata {
            format_name: format!("custom_{}", extension),
            version: "1.0".to_string(),
            custom_headers: vec![],
        })
    } else {
        None
    };

    Ok(BinaryFileInfo {
        path: path.to_string(),
        size,
        file_type: file_type_str,
        extension,
        is_custom_format: is_custom,
        metadata: custom_metadata,
    })
}

#[command]
pub fn read_binary_file_data(path: &str) -> Result<String, String> {
    let path_obj = Path::new(path);

    if !path_obj.exists() {
        return Err(format!("文件不存在: {}", path));
    }

    if !path_obj.is_file() {
        return Err(format!("路径不是文件: {}", path));
    }

    let bytes = fs::read(path_obj).map_err(|e| format!("读取文件失败: {}", e))?;
    Ok(base64_encode(&bytes))
}

#[command]
pub fn save_binary_file(path: &str, data_base64: &str) -> Result<String, String> {
    let path_obj = Path::new(path);

    if !path_obj.exists() {
        return Err(format!("文件不存在: {}", path));
    }

    if !path_obj.is_file() {
        return Err(format!("路径不是文件: {}", path));
    }

    let bytes = base64_decode(data_base64)?;
    fs::write(path_obj, &bytes).map_err(|e| format!("保存文件失败: {}", e))?;
    Ok(format!("二进制文件保存成功: {}", path))
}

#[command]
pub fn create_custom_binary_file(
    path: &str,
    data_base64: &str,
    format_name: &str,
    version: &str,
    custom_headers: Vec<(String, String)>
) -> Result<String, String> {
    let path_obj = Path::new(path);

    if path_obj.exists() {
        return Err(format!("文件已存在: {}", path));
    }

    if let Some(parent) = path_obj.parent() {
        if !parent.exists() {
            fs::create_dir_all(parent).map_err(|e| format!("创建目录失败: {}", e))?;
        }
    }

    let mut file_data = Vec::new();

    let magic = format!("{}:{}", format_name, version);
    let magic_bytes = magic.as_bytes();
    let magic_len = magic_bytes.len() as u32;

    file_data.extend_from_slice(&magic_len.to_le_bytes());
    file_data.extend_from_slice(magic_bytes);

    for (key, value) in &custom_headers {
        let header = format!("{}={}", key, value);
        let header_bytes = header.as_bytes();
        let header_len = header_bytes.len() as u32;
        file_data.extend_from_slice(&header_len.to_le_bytes());
        file_data.extend_from_slice(header_bytes);
    }

    file_data.extend_from_slice(&[0u8; 4]);

    let data_bytes = base64_decode(data_base64)?;
    file_data.extend_from_slice(&data_bytes);

    fs::write(path_obj, &file_data).map_err(|e| format!("创建私有格式文件失败: {}", e))?;
    Ok(format!("私有格式文件创建成功: {}", path))
}

#[command]
pub fn read_custom_binary_file(path: &str) -> Result<BinaryFileInfo, String> {
    let path_obj = Path::new(path);

    if !path_obj.exists() {
        return Err(format!("文件不存在: {}", path));
    }

    if !path_obj.is_file() {
        return Err(format!("路径不是文件: {}", path));
    }

    let metadata = fs::metadata(path_obj).map_err(|e| format!("获取文件元数据失败: {}", e))?;
    let size = metadata.len();
    let extension = path_obj.extension()
        .and_then(|e| e.to_str())
        .unwrap_or("")
        .to_string();

    let bytes = fs::read(path_obj).map_err(|e| format!("读取文件失败: {}", e))?;

    if bytes.len() < 4 {
        return Err("文件格式无效".to_string());
    }

    let magic_len = u32::from_le_bytes([bytes[0], bytes[1], bytes[2], bytes[3]]) as usize;

    if bytes.len() < 4 + magic_len {
        return Err("文件格式无效：魔数长度不匹配".to_string());
    }

    let magic = String::from_utf8_lossy(&bytes[4..4 + magic_len]).to_string();
    let parts: Vec<&str> = magic.splitn(2, ':').collect();
    let format_name = parts[0].to_string();
    let version = if parts.len() > 1 { parts[1].to_string() } else { "unknown".to_string() };

    let mut offset = 4 + magic_len;
    let mut custom_headers = Vec::new();

    while offset + 4 <= bytes.len() {
        let header_len = u32::from_le_bytes([
            bytes[offset],
            bytes[offset + 1],
            bytes[offset + 2],
            bytes[offset + 3],
        ]) as usize;

        if header_len == 0 {
            break;
        }

        if offset + 4 + header_len > bytes.len() {
            break;
        }

        let header = String::from_utf8_lossy(&bytes[offset + 4..offset + 4 + header_len]).to_string();
        if let Some((key, value)) = header.split_once('=') {
            custom_headers.push((key.to_string(), value.to_string()));
        }

        offset += 4 + header_len;
    }

    Ok(BinaryFileInfo {
        path: path.to_string(),
        size,
        file_type: format!("custom:{}", format_name),
        extension,
        is_custom_format: true,
        metadata: Some(CustomMetadata {
            format_name,
            version,
            custom_headers,
        }),
    })
}

#[command]
pub fn extract_custom_binary_data(path: &str) -> Result<String, String> {
    let path_obj = Path::new(path);

    if !path_obj.exists() {
        return Err(format!("文件不存在: {}", path));
    }

    if !path_obj.is_file() {
        return Err(format!("路径不是文件: {}", path));
    }

    let bytes = fs::read(path_obj).map_err(|e| format!("读取文件失败: {}", e))?;

    if bytes.len() < 4 {
        return Err("文件格式无效".to_string());
    }

    let magic_len = u32::from_le_bytes([bytes[0], bytes[1], bytes[2], bytes[3]]) as usize;

    if bytes.len() < 4 + magic_len {
        return Err("文件格式无效：魔数长度不匹配".to_string());
    }

    let mut offset = 4 + magic_len;

    while offset + 4 <= bytes.len() {
        let header_len = u32::from_le_bytes([
            bytes[offset],
            bytes[offset + 1],
            bytes[offset + 2],
            bytes[offset + 3],
        ]) as usize;

        if header_len == 0 {
            break;
        }

        if offset + 4 + header_len > bytes.len() {
            break;
        }

        offset += 4 + header_len;
    }

    let data_bytes = &bytes[offset..];
    Ok(base64_encode(data_bytes))
}

fn base64_encode(bytes: &[u8]) -> String {
    const CHARS: &[u8] = b"ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789+/";
    let mut result = String::with_capacity((bytes.len() + 2) / 3 * 4);
    let chunks = bytes.chunks(3);
    for chunk in chunks {
        let b0 = chunk[0] as u32;
        let b1 = if chunk.len() > 1 { chunk[1] as u32 } else { 0 };
        let b2 = if chunk.len() > 2 { chunk[2] as u32 } else { 0 };
        let triple = (b0 << 16) | (b1 << 8) | b2;
        result.push(CHARS[((triple >> 18) & 0x3F) as usize] as char);
        result.push(CHARS[((triple >> 12) & 0x3F) as usize] as char);
        if chunk.len() > 1 {
            result.push(CHARS[((triple >> 6) & 0x3F) as usize] as char);
        } else {
            result.push('=');
        }
        if chunk.len() > 2 {
            result.push(CHARS[(triple & 0x3F) as usize] as char);
        } else {
            result.push('=');
        }
    }
    result
}

fn base64_decode(input: &str) -> Result<Vec<u8>, String> {
    let input = input.trim_end_matches('=');
    let mut result = Vec::new();
    let chars: Vec<u8> = input.bytes().collect();

    for chunk in chars.chunks(4) {
        let mut vals = [0u32; 4];
        for (i, &c) in chunk.iter().enumerate() {
            vals[i] = match c {
                b'A'..=b'Z' => (c - b'A') as u32,
                b'a'..=b'z' => (c - b'a' + 26) as u32,
                b'0'..=b'9' => (c - b'0' + 52) as u32,
                b'+' => 62,
                b'/' => 63,
                _ => return Err(format!("无效的 base64 字符: {}", c as char)),
            };
        }

        let triple = (vals[0] << 18) | (vals[1] << 12) | (vals[2] << 6) | vals[3];
        result.push(((triple >> 16) & 0xFF) as u8);
        if chunk.len() > 2 {
            result.push(((triple >> 8) & 0xFF) as u8);
        }
        if chunk.len() > 3 {
            result.push((triple & 0xFF) as u8);
        }
    }

    Ok(result)
}
