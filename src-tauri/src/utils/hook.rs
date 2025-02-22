use std::collections::HashMap;
use std::fs;
use lazy_static::lazy_static;
use md5::{Md5, Digest};
use regex::Regex;
use crate::utils::paths::AppPaths;

lazy_static! {
    static ref MAIN_JS_MD5: HashMap<&'static str, Vec<&'static str>> = {
        
        let mut m = HashMap::new();
        // 0.45.2~0.45.10 来源 https://github.com/Angels-Ray/fake-rosrus/blob/2ef9115e37631df38b8313ef914a2ef736f0d1a0/utils/mainjs.js#L6C1-L16C3
        m.insert("1f53d40367d0ac76f3f123c83b901497", vec!["0.45.2~0.45.8[-5]"]);
        m.insert("1650464dc26313c87c789da60c0495d0", vec!["0.45.10[-5]"]);
        m.insert("1f53d40367d0ac76f3f123c83b901497", vec!["0.45.11[-5]"]);
        m.insert("6114002d8e2bb53853f4a49e228e8c74", vec!["0.45.2"]);
        m.insert("fde15c3fe02b6c48a2a8fa788ff3ed2a", vec!["0.45.3"]);
        m.insert("0052f48978fa8e322e2cb7e0c101d6b2", vec!["0.45.4"]);
        m.insert("74ed1a381f4621ccfd35989f322dc8a2", vec!["0.45.5"]);
        m.insert("e82b270f8c114247968bb4a04a4f4f72", vec!["0.45.7"]);
        m.insert("352c7f017a7eab95690263a9d83b7832", vec!["0.45.8"]);
        m.insert("217d4ae5933b13b9aae1829750d0b709", vec!["0.45.10"]);
        m.insert("0c22467f3e082760524dda841eeb2ef6", vec!["0.45.11"]);
        m
    };

    static ref MACHINE_ID_REGEX: Regex = Regex::new(
        r#"async\s+(\w+)\s*\(\)\s*\{\s*return\s+this\.[\w.]+(?:\?\?|\?)\s*this\.([\w.]+)\.machineId\s*\}"#
    ).unwrap();

    static ref MAC_MACHINE_ID_REGEX: Regex = Regex::new(
        r#"async\s+(\w+)\s*\(\)\s*\{\s*return\s+this\.[\w.]+(?:\?\?|\?)\s*this\.([\w.]+)\.macMachineId\s*\}"#
    ).unwrap();
}

pub struct Hook;

impl Hook {
    pub fn machine_id_regex() -> &'static Regex {
        &MACHINE_ID_REGEX
    }

    pub fn mac_machine_id_regex() -> &'static Regex {
        &MAC_MACHINE_ID_REGEX
    }

    pub fn main_js_md5() -> &'static HashMap<&'static str, Vec<&'static str>> {
        &MAIN_JS_MD5
    }

    /// 计算文件内容的MD5（排除最后几行）
    fn calculate_md5_without_last_lines(content: &str, line_count_to_remove: usize) -> String {
        // 1. 按行分割, 保留空行
        let lines: Vec<&str> = content.split('\n').collect();
        
        // 2. 直接计算要保留的行数
        let keep_lines = if lines.len() > line_count_to_remove {
            lines.len() - line_count_to_remove
        } else {
            0
        };

        // 3. 获取需要的行并保持原始换行符
        let content = if keep_lines > 0 {
            lines[..keep_lines].join("\n")
        } else {
            String::new()
        };

        // 5. 计算 MD5
        let mut hasher = Md5::new();
        hasher.update(content.as_bytes());
        let result = format!("{:x}", hasher.finalize());
        
        result
    }

    /// 获取当前系统 main.js 的 MD5
    pub fn get_main_js_hash() -> Result<String, String> {
        let paths = AppPaths::new()?;
        let content = fs::read_to_string(&paths.main_js)
            .map_err(|e| format!("读取 main.js 失败: {}", e))?;
        Ok(Self::calculate_md5_without_last_lines(&content, 5))
    }

    /// 修补 main.js 文件内容
    pub fn update_main_js_content() -> Result<(), String> {
        let paths = AppPaths::new()?;
        let file_path = &paths.main_js;
        
        // 读取文件内容
        let content = fs::read_to_string(file_path)
            .map_err(|e| format!("读取文件失败: {}", e))?;

        // 计算MD5
        let md5 = Self::calculate_md5_without_last_lines(&content, 5);

        // 检查版本兼容性
        if !MAIN_JS_MD5.contains_key(md5.as_str()) {
            let versions: Vec<&str> = MAIN_JS_MD5.values().flatten().copied().collect();
            return Err(format!(
                "不支持的 Cursor 版本或 main.js 已被修补。\n支持的版本: {}",
                versions.join(", ")
            ));
        }

        // 创建备份
        let backup_path = file_path.with_extension("js.backup");
        fs::write(&backup_path, &content)
            .map_err(|e| format!("创建备份失败: {}", e))?;

        // 替换内容
        let mut modified_content = content.clone();

        // 替换 machineId
        modified_content = MACHINE_ID_REGEX.replace_all(&modified_content, |caps: &regex::Captures| {
            format!("async {}() {{ return this.{}.machineId }}", &caps[1], &caps[2])
        }).to_string();

        // 替换 macMachineId
        modified_content = MAC_MACHINE_ID_REGEX.replace_all(&modified_content, |caps: &regex::Captures| {
            format!("async {}() {{ return this.{}.macMachineId }}", &caps[1], &caps[2])
        }).to_string();

        // 写入修改后的内容
        fs::write(file_path, modified_content)
            .map_err(|e| format!("写入文件失败: {}", e))?;

        Ok(())
    }

    /// 从备份恢复 main.js
    pub fn restore_from_backup() -> Result<(), String> {
        let paths = AppPaths::new()?;
        let file_path = &paths.main_js;
        let backup_path = file_path.with_extension("js.backup");

        if !backup_path.exists() {
            return Err("备份文件不存在".to_string());
        }

        let backup_content = fs::read_to_string(&backup_path)
            .map_err(|e| format!("读取备份文件失败: {}", e))?;

        fs::write(file_path, backup_content)
            .map_err(|e| format!("恢复文件失败: {}", e))?;

        fs::remove_file(backup_path)
            .map_err(|e| format!("删除备份文件失败: {}", e))?;

        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_get_main_js_hash() {
        let paths = AppPaths::new().unwrap();
        let content = fs::read_to_string(&paths.main_js)
            .unwrap_or_else(|e| panic!("读取 main.js 失败: {}", e));

        println!("原始文件的最后几行:");
        let lines: Vec<&str> = content.split('\n').collect();
        for line in lines.iter().rev().take(6) {
            println!("{}", line);
        }

        // 获取完整内容的hash
        let full_hash = {
            let mut hasher = Md5::new();
            hasher.update(content.as_bytes());
            format!("{:x}", hasher.finalize())
        };

        // 获取去掉最后5行的hash
        let trimmed_hash = Hook::calculate_md5_without_last_lines(&content, 5);

        println!("\n完整文件的 MD5: {}", full_hash);
        println!("去掉最后5行的 MD5: {}", trimmed_hash);
        println!("是否为支持的版本: {}", MAIN_JS_MD5.contains_key(trimmed_hash.as_str()));

        assert_ne!(full_hash, trimmed_hash, "去掉最后5行的hash应该与完整文件hash不同");
    }

    #[test]
    fn test_update_and_restore() {
        // 先获取原始hash
        let original_hash = Hook::get_main_js_hash()
            .unwrap_or_else(|e| panic!("获取原始hash失败: {}", e));
        println!("原始 main.js 的 MD5: {}", original_hash);

        // 尝试修补
        match Hook::update_main_js_content() {
            Ok(_) => {
                println!("成功修补 main.js");
                // 获取修补后的hash
                let modified_hash = Hook::get_main_js_hash().unwrap();
                println!("修补后的 MD5: {}", modified_hash);
                assert_ne!(original_hash, modified_hash, "修补后的hash应该与原始hash不同");
            },
            Err(e) => println!("修补失败: {}", e),
        }

        // 等待一秒, 确保文件操作完成
        std::thread::sleep(std::time::Duration::from_secs(1));

        // 从备份恢复
        match Hook::restore_from_backup() {
            Ok(_) => {
                println!("成功恢复 main.js");
                // 验证恢复后的hash是否与原始hash相同
                let restored_hash = Hook::get_main_js_hash().unwrap();
                println!("恢复后的 MD5: {}", restored_hash);
                assert_eq!(original_hash, restored_hash, "恢复后的hash应该与原始hash相同");
            },
            Err(e) => println!("恢复失败: {}", e),
        }
    }

    #[test]
    fn test_regex_patterns() {
        let paths = AppPaths::new().unwrap();
        let content = fs::read_to_string(&paths.main_js).unwrap();

        // 测试是否能匹配到相关模式
        let machine_id_matches = MACHINE_ID_REGEX.find_iter(&content).count();
        let mac_machine_id_matches = MAC_MACHINE_ID_REGEX.find_iter(&content).count();

        println!("找到 machineId 匹配数: {}", machine_id_matches);
        println!("找到 macMachineId 匹配数: {}", mac_machine_id_matches);

        // 确保至少找到一个匹配
        assert!(machine_id_matches > 0, "应该至少找到一个 machineId 匹配");
        assert!(mac_machine_id_matches > 0, "应该至少找到一个 macMachineId 匹配");
    }
} 