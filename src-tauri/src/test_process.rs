use std::process::Command;
use std::thread;
use std::time::Duration;

const MAX_ATTEMPTS: i32 = 3;
const RETRY_DELAY: Duration = Duration::from_secs(2);

pub struct ProcessManager;

impl ProcessManager {
    pub fn new() -> Self {
        ProcessManager
    }

    /// 检查Cursor进程是否正在运行
    pub fn is_cursor_running(&self) -> bool {
        if let Ok(processes) = self.get_cursor_processes() {
            !processes.is_empty()
        } else {
            false
        }
    }

    /// 终止所有Cursor进程
    pub fn kill_cursor_processes(&self) -> Result<(), String> {
        for attempt in 1..=MAX_ATTEMPTS {
            let processes = self.get_cursor_processes()
                .map_err(|e| format!("获取进程列表失败: {}", e))?;

            if processes.is_empty() {
                return Ok(());
            }

            // Windows下先尝试优雅关闭
            #[cfg(target_os = "windows")]
            {
                for pid in &processes {
                    let _ = Command::new("taskkill")
                        .args(&["/PID", pid])
                        .output();
                    thread::sleep(Duration::from_millis(500));
                }
            }

            // 强制结束剩余进程
            if let Ok(remaining) = self.get_cursor_processes() {
                for pid in remaining {
                    let _ = self.kill_process(&pid);
                }
            }

            thread::sleep(RETRY_DELAY);

            // 检查是否还有进程存在
            if let Ok(remaining) = self.get_cursor_processes() {
                if remaining.is_empty() {
                    return Ok(());
                }
            }

            if attempt == MAX_ATTEMPTS {
                return Err("达到最大重试次数,仍无法终止所有Cursor进程".to_string());
            }
        }

        Ok(())
    }

    /// 获取所有Cursor进程的PID
    fn get_cursor_processes(&self) -> Result<Vec<String>, String> {
        let (cmd, args) = match std::env::consts::OS {
            "windows" => ("tasklist", vec!["/FO", "CSV", "/NH"]),
            "macos" => ("ps", vec!["-ax"]),
            "linux" => ("ps", vec!["-A"]),
            _ => return Err("不支持的操作系统".to_string()),
        };

        let output = Command::new(cmd)
            .args(&args)
            .output()
            .map_err(|e| format!("执行命令失败: {}", e))?;

        let output_str = String::from_utf8_lossy(&output.stdout);
        Ok(self.parse_process_list(&output_str))
    }

    /// 解析进程列表输出
    fn parse_process_list(&self, output: &str) -> Vec<String> {
        let mut processes = Vec::new();
        
        for line in output.lines() {
            let lower_line = line.to_lowercase();
            
            // 跳过自身进程
            if lower_line.contains("cursor-pool") {
                continue;
            }

            // 检查是否为Cursor进程
            if let Some(pid) = self.find_cursor_process(line, &lower_line) {
                processes.push(pid);
            }
        }

        processes
    }

    /// 查找Cursor进程并返回PID
    fn find_cursor_process(&self, line: &str, lower_line: &str) -> Option<String> {
        let patterns = [
            "cursor.exe",
            "cursor ",
            "cursor",
            "*cursor*",
        ];

        for pattern in &patterns {
            if self.match_pattern(lower_line, &pattern.to_lowercase()) {
                return self.extract_pid(line);
            }
        }
        None
    }

    /// 模式匹配
    fn match_pattern(&self, line: &str, pattern: &str) -> bool {
        if pattern.starts_with('*') && pattern.ends_with('*') {
            let search = &pattern[1..pattern.len() - 1];
            line.contains(search)
        } else if pattern.starts_with('*') {
            line.ends_with(&pattern[1..])
        } else if pattern.ends_with('*') {
            line.starts_with(&pattern[..pattern.len() - 1])
        } else {
            line == pattern
        }
    }

    /// 提取PID
    fn extract_pid(&self, line: &str) -> Option<String> {
        match std::env::consts::OS {
            "windows" => {
                let parts: Vec<&str> = line.split(',').collect();
                if parts.len() >= 2 {
                    Some(parts[1].trim_matches('"').to_string())
                } else {
                    None
                }
            }
            "macos" | "linux" => {
                let parts: Vec<&str> = line.split_whitespace().collect();
                if !parts.is_empty() {
                    Some(parts[0].to_string())
                } else {
                    None
                }
            }
            _ => None,
        }
    }

    /// 终止进程
    fn kill_process(&self, pid: &str) -> Result<(), String> {
        let (cmd, args) = match std::env::consts::OS {
            "windows" => ("taskkill", vec!["/F", "/PID", pid]),
            "macos" | "linux" => ("kill", vec!["-9", pid]),
            _ => return Err("不支持的操作系统".to_string()),
        };

        Command::new(cmd)
            .args(&args)
            .output()
            .map_err(|e| format!("终止进程失败: {}", e))?;

        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_process_manager() {
        let manager = ProcessManager::new();
        
        // 测试检测Cursor进程
        println!("Cursor是否运行: {}", manager.is_cursor_running());

        // 测试获取进程列表
        if let Ok(processes) = manager.get_cursor_processes() {
            println!("找到的Cursor进程: {:?}", processes);
        }

        // 测试终止进程
        match manager.kill_cursor_processes() {
            Ok(_) => println!("成功终止所有Cursor进程"),
            Err(e) => println!("终止进程失败: {}", e),
        }
    }

    #[test]
    fn test_pattern_matching() {
        let manager = ProcessManager::new();
        
        assert!(manager.match_pattern("cursor", "cursor"));
        assert!(manager.match_pattern("cursor.exe", "*cursor*"));
        assert!(manager.match_pattern("cursor process", "*cursor*"));
        assert!(manager.match_pattern("cursor.exe", "cursor*"));
        assert!(!manager.match_pattern("other process", "*cursor*"));
    }
}
