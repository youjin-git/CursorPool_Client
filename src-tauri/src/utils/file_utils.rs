use std::fs;
use std::path::Path;
use std::io::Error;

/// 检查文件是否是只读
pub fn is_read_only(path: &Path) -> Result<bool, Error> {
    let metadata = fs::metadata(path)?;
    Ok(metadata.permissions().readonly())
}

/// 设置文件为只读
pub fn set_read_only(path: &Path) -> Result<(), Error> {
    let mut permissions = fs::metadata(path)?.permissions();
    permissions.set_readonly(true);
    fs::set_permissions(path, permissions)?;
    Ok(())
}

/// 取消文件只读属性
pub fn unset_read_only(path: &Path) -> Result<(), Error> {
    let mut permissions = fs::metadata(path)?.permissions();
    permissions.set_readonly(false);
    fs::set_permissions(path, permissions)?;
    Ok(())
}

/// 安全写入文件, 自动处理只读属性
pub fn safe_write(path: &Path, content: &str) -> Result<(), Error> {
    // 检查文件是否存在
    if path.exists() {
        // 如果是只读文件, 先取消只读属性
        if is_read_only(path)? {
            unset_read_only(path)?;
        }
    }

    // 写入文件
    fs::write(path, content)?;

    // 设置文件为只读
    set_read_only(path)?;

    Ok(())
} 