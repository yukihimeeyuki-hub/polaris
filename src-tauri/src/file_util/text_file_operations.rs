use std::fs;
use std::path::Path;
use tauri::command;

#[command]
pub fn create_text_file(path: &str, content: &str) -> Result<String, String> {
    let path_obj = Path::new(path);

    if path_obj.exists() {
        return Err(format!("文件已存在: {}", path));
    }

    if let Some(parent) = path_obj.parent() {
        if !parent.exists() {
            fs::create_dir_all(parent).map_err(|e| format!("创建目录失败: {}", e))?;
        }
    }

    fs::write(path_obj, content).map_err(|e| format!("创建文件失败: {}", e))?;
    Ok(format!("文件创建成功: {}", path))
}

#[command]
pub fn delete_text_file(path: &str) -> Result<String, String> {
    let path_obj = Path::new(path);

    if !path_obj.exists() {
        return Err(format!("文件不存在: {}", path));
    }

    if !path_obj.is_file() {
        return Err(format!("路径不是文件: {}", path));
    }

    fs::remove_file(path_obj).map_err(|e| format!("删除文件失败: {}", e))?;
    Ok(format!("文件删除成功: {}", path))
}

#[command]
pub fn read_text_file(path: &str) -> Result<String, String> {
    let path_obj = Path::new(path);

    if !path_obj.exists() {
        return Err(format!("文件不存在: {}", path));
    }

    if !path_obj.is_file() {
        return Err(format!("路径不是文件: {}", path));
    }

    fs::read_to_string(path_obj).map_err(|e| format!("读取文件失败: {}", e))
}

#[command]
pub fn save_text_file(path: &str, content: &str) -> Result<String, String> {
    let path_obj = Path::new(path);

    if !path_obj.exists() {
        return Err(format!("文件不存在: {}", path));
    }

    if !path_obj.is_file() {
        return Err(format!("路径不是文件: {}", path));
    }

    fs::write(path_obj, content).map_err(|e| format!("保存文件失败: {}", e))?;
    Ok(format!("文件保存成功: {}", path))
}

#[command]
pub fn edit_text_file(path: &str, start_line: usize, end_line: usize, new_content: &str) -> Result<String, String> {
    let path_obj = Path::new(path);

    if !path_obj.exists() {
        return Err(format!("文件不存在: {}", path));
    }

    if !path_obj.is_file() {
        return Err(format!("路径不是文件: {}", path));
    }

    let content = fs::read_to_string(path_obj).map_err(|e| format!("读取文件失败: {}", e))?;
    let lines: Vec<&str> = content.lines().collect();
    let total_lines = lines.len();

    if start_line == 0 || end_line == 0 || start_line > total_lines || end_line > total_lines || start_line > end_line {
        return Err(format!("行号无效: 起始行={}, 结束行={}, 总行数={}", start_line, end_line, total_lines));
    }

    let start_idx = start_line - 1;
    let end_idx = end_line - 1;

    let mut new_lines: Vec<String> = lines.iter().take(start_idx).map(|s| s.to_string()).collect();

    new_lines.push(new_content.to_string());

    new_lines.extend(lines.iter().skip(end_idx + 1).map(|s| s.to_string()));

    let final_content = new_lines.join("\n");

    fs::write(path_obj, final_content).map_err(|e| format!("保存文件失败: {}", e))?;
    Ok(format!("文件编辑成功: {}", path))
}
