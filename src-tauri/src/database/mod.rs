use rusqlite::{Connection, Result as SqliteResult, params};
use std::sync::{Arc, Mutex};
use tauri::AppHandle;
use tauri::Manager;

#[derive(Debug, Clone)]
pub struct Database {
    connection: Arc<Mutex<Connection>>,
}

impl Database {
    pub fn new(app_handle: &AppHandle) -> SqliteResult<Self> {
        // 获取应用数据目录
        let app_dir = app_handle.path().app_data_dir().expect("无法获取应用数据目录");
        
        // 确保目录存在
        std::fs::create_dir_all(&app_dir).expect("无法创建数据目录");
        
        // 数据库文件路径
        let db_path = app_dir.join("cursor_pool.db");
        
        // 创建或打开数据库连接
        let connection = Connection::open(db_path)?;
        
        // 初始化表结构
        connection.execute(
            "CREATE TABLE IF NOT EXISTS item (
                key TEXT PRIMARY KEY,
                value TEXT NOT NULL
            )",
            [],
        )?;
        
        connection.execute(
            "CREATE TABLE IF NOT EXISTS account (
                account TEXT PRIMARY KEY,
                userId TEXT NOT NULL,
                cursorToken TEXT NOT NULL
            )",
            [],
        )?;
        
        Ok(Self {
            connection: Arc::new(Mutex::new(connection)),
        })
    }
    
    // item表操作
    
    pub fn set_item(&self, key: &str, value: &str) -> SqliteResult<()> {
        let conn = self.connection.lock().unwrap();
        conn.execute(
            "INSERT OR REPLACE INTO item (key, value) VALUES (?, ?)",
            params![key, value],
        )?;
        Ok(())
    }
    
    pub fn get_item(&self, key: &str) -> SqliteResult<Option<String>> {
        let conn = self.connection.lock().unwrap();
        let mut stmt = conn.prepare("SELECT value FROM item WHERE key = ?")?;
        let mut rows = stmt.query(params![key])?;
        
        if let Some(row) = rows.next()? {
            Ok(Some(row.get(0)?))
        } else {
            Ok(None)
        }
    }
    
    pub fn delete_item(&self, key: &str) -> SqliteResult<()> {
        let conn = self.connection.lock().unwrap();
        conn.execute("DELETE FROM item WHERE key = ?", params![key])?;
        Ok(())
    }
    
    pub fn get_all_items(&self) -> SqliteResult<Vec<(String, String)>> {
        let conn = self.connection.lock().unwrap();
        let mut stmt = conn.prepare("SELECT key, value FROM item")?;
        let rows = stmt.query_map([], |row| {
            Ok((row.get(0)?, row.get(1)?))
        })?;
        
        let mut items = Vec::new();
        for item in rows {
            items.push(item?);
        }
        
        Ok(items)
    }
    
    // account表操作
    
    pub fn add_account(&self, account: &str, user_id: &str, cursor_token: &str) -> SqliteResult<()> {
        let conn = self.connection.lock().unwrap();
        conn.execute(
            "INSERT OR REPLACE INTO account (account, userId, cursorToken) VALUES (?, ?, ?)",
            params![account, user_id, cursor_token],
        )?;
        Ok(())
    }
    
    pub fn get_account(&self, account: &str) -> SqliteResult<Option<(String, String, String)>> {
        let conn = self.connection.lock().unwrap();
        let mut stmt = conn.prepare("SELECT account, userId, cursorToken FROM account WHERE account = ?")?;
        let mut rows = stmt.query(params![account])?;
        
        if let Some(row) = rows.next()? {
            Ok(Some((row.get(0)?, row.get(1)?, row.get(2)?)))
        } else {
            Ok(None)
        }
    }
    
    pub fn delete_account(&self, account: &str) -> SqliteResult<()> {
        let conn = self.connection.lock().unwrap();
        conn.execute("DELETE FROM account WHERE account = ?", params![account])?;
        Ok(())
    }
    
    pub fn get_all_accounts(&self) -> SqliteResult<Vec<(String, String, String)>> {
        let conn = self.connection.lock().unwrap();
        let mut stmt = conn.prepare("SELECT account, userId, cursorToken FROM account")?;
        let rows = stmt.query_map([], |row| {
            Ok((row.get(0)?, row.get(1)?, row.get(2)?))
        })?;
        
        let mut accounts = Vec::new();
        for account in rows {
            accounts.push(account?);
        }
        
        Ok(accounts)
    }
    
    pub fn update_account_token(&self, account: &str, cursor_token: &str) -> SqliteResult<()> {
        let conn = self.connection.lock().unwrap();
        conn.execute(
            "UPDATE account SET cursorToken = ? WHERE account = ?",
            params![cursor_token, account],
        )?;
        Ok(())
    }
} 