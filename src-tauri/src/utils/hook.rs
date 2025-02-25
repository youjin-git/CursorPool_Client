use std::collections::HashMap;
use std::fs;
use lazy_static::lazy_static;
use md5::{Md5, Digest};
use regex::Regex;
use crate::utils::paths::AppPaths;
use reqwest;
use std::time::Duration;
use crate::api::client::get_base_url;

lazy_static! {
    static ref MAIN_JS_MD5: HashMap<&'static str, Vec<&'static str>> = {
        
        let mut m = HashMap::new();
        // 来源 https://gist.githubusercontent.com/Angels-Ray/11a0c8990750f4f563292a55c42465f1/raw
        m.insert("16181e0877949fa846669a134783f858", vec!["0.44.11[-5]"]);
        m.insert("1f53d40367d0ac76f3f123c83b901497", vec!["0.45.2~0.45.8[-5]", "0.45.11[-5]"]);
        m.insert("1650464dc26313c87c789da60c0495d0", vec!["0.45.10[-5]"]);
        m.insert("a2b8ba180a037bec28c9f8e06808423e", vec!["0.45.12[-5]"]);
        m.insert("dfe46d46dcf52711f68522afdb985d01", vec!["0.45.14[-5]"]);
        m.insert("723d492726d0cfa5ac2ad0649f499ef5", vec!["0.45.15[-5]"]);
        m.insert("2df7e08131902951452d37fe946b8b8c", vec!["0.46.0[-5]"]);
        m.insert("44fd6c68052686e67c0402f69ae3f1bb", vec!["0.46.2[-5]"]);
        m.insert("2defb2347364e7193306389730012e3f", vec!["0.46.3[-5]"]);
        m.insert("1e5b4beb11a90d79645ad54a6477482d", vec!["0.44.11"]);
        m.insert("217d4ae5933b13b9aae1829750d0b709", vec!["0.45.10"]);
        m.insert("0c22467f3e082760524dda841eeb2ef6", vec!["0.45.11"]);
        m.insert("3540834ce1b6125022a9316375dfdd43", vec!["0.45.12"]);
        m.insert("ed5b92fca478515c138c3df0c8860fe2", vec!["0.45.14"]);
        m.insert("76bddc6605df5d845af68d4959a4f045", vec!["0.45.15"]);
        m.insert("6114002d8e2bb53853f4a49e228e8c74", vec!["0.45.2"]);
        m.insert("fde15c3fe02b6c48a2a8fa788ff3ed2a", vec!["0.45.3"]);
        m.insert("0052f48978fa8e322e2cb7e0c101d6b2", vec!["0.45.4"]);
        m.insert("74ed1a381f4621ccfd35989f322dc8a2", vec!["0.45.5"]);
        m.insert("7f379a12f3341b59c9aecf394818b5ab", vec!["0.45.6"]);
        m.insert("e82b270f8c114247968bb4a04a4f4f72", vec!["0.45.7"]);
        m.insert("352c7f017a7eab95690263a9d83b7832", vec!["0.45.8"]);
        m.insert("a6d83fa177878ff497286d659957d9ab", vec!["0.46.0"]);
        m.insert("95277d19fe0bb4eb8bbb236d5386cd46", vec!["0.46.2"]);
        m.insert("f85470d71eca2f99d3c9c265dfbf5b8f", vec!["0.46.3"]);
        m
    };

    static ref MACHINE_ID_REGEX: Regex = Regex::new(
        r#"async\s+(\w+)\s*\(\)\s*\{\s*return\s+this\.[\w.]+(?:\?\?|\?)\s*this\.([\w.]+)\.machineId\s*\}"#
    ).unwrap();

    static ref MAC_MACHINE_ID_REGEX: Regex = Regex::new(
        r#"async\s+(\w+)\s*\(\)\s*\{\s*return\s+this\.[\w.]+(?:\?\?|\?)\s*this\.([\w.]+)\.macMachineId\s*\}"#
    ).unwrap();
}

const REMOTE_HASH_URL: &str = "/hash";

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

    async fn fetch_remote_hash_map() -> Result<HashMap<String, Vec<String>>, String> {
        let client = reqwest::Client::new();
        let response = client
            .get(format!("{}{}", get_base_url(), REMOTE_HASH_URL))
            .timeout(Duration::from_secs(10))
            .send()
            .await
            .map_err(|e| format!("请求失败: {}", e))?;

        if response.status().is_success() {
            let text = response
                .text()
                .await
                .map_err(|e| format!("读取响应失败: {}", e))?;

            serde_json::from_str(&text)
                .map_err(|e| format!("解析 JSON 失败: {}", e))
        } else {
            Err("云端没有找到哈希数据".to_string())
        }
    }

    // 修改现有的版本检查函数
    pub async fn check_version_compatibility(md5: &str) -> Result<bool, String> {
        // 首先检查本地映射
        if MAIN_JS_MD5.contains_key(md5) {
            return Ok(true);
        }

        // 尝试从远程获取
        match Self::fetch_remote_hash_map().await {
            Ok(remote_map) => {
                if remote_map.contains_key(md5) {
                    return Ok(true);
                }
            }
            Err(e) => {
                println!("获取远程映射失败: {}", e);
            }
        }

        // 如果所有尝试都失败，返回不支持的版本错误
        let versions: Vec<&str> = MAIN_JS_MD5.values().flatten().copied().collect();
        Err(format!(
            "不支持的 Cursor 版本或 main.js 已被修补。\n支持的版本: {}",
            versions.join(", ")
        ))
    }

    /// 修改现有的更新函数
    pub async fn update_main_js_content() -> Result<(), String> {
        let paths = AppPaths::new()?;
        let file_path = &paths.main_js;
        
        // 读取文件内容
        let content = fs::read_to_string(file_path)
            .map_err(|e| format!("读取文件失败: {}", e))?;

        // 计算MD5
        let md5 = Self::calculate_md5_without_last_lines(&content, 5);

        // 检查版本兼容性（包括远程检查）
        if !Self::check_version_compatibility(&md5).await? {
            return Err("版本不兼容".to_string());
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

    #[tokio::test]
    async fn test_update_and_restore() {
        // 先获取原始hash
        let original_hash = Hook::get_main_js_hash()
            .unwrap_or_else(|e| panic!("获取原始hash失败: {}", e));
        println!("原始 main.js 的 MD5: {}", original_hash);

        // 尝试修补
        match Hook::update_main_js_content().await {
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