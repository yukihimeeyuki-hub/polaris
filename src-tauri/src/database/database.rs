use rusqlite::{Connection, Result};
use std::path::PathBuf;
use std::sync::{Arc, Mutex};

/// 数据库管理器
pub struct DatabaseManager {
    connections: Arc<Mutex<std::collections::HashMap<String, Arc<Mutex<Connection>>>>>,
}

impl DatabaseManager {
    pub fn new() -> Self {
        Self {
            connections: Arc::new(Mutex::new(std::collections::HashMap::new())),
        }
    }

    /// 创建或打开数据库
    pub fn create_database(&self, db_name: &str) -> Result<(), String> {
        let db_path = self.get_db_path(db_name);
        
        let conn = Connection::open(&db_path)
            .map_err(|e| format!("Failed to open database: {}", e))?;
        
        let mut connections = self.connections.lock().unwrap();
        connections.insert(db_name.to_string(), Arc::new(Mutex::new(conn)));
        
        Ok(())
    }

    /// 删除数据库
    pub fn delete_database(&self, db_name: &str) -> Result<(), String> {
        // 先关闭连接
        let mut connections = self.connections.lock().unwrap();
        connections.remove(db_name);
        
        // 删除文件
        let db_path = self.get_db_path(db_name);
        if db_path.exists() {
            std::fs::remove_file(&db_path)
                .map_err(|e| format!("Failed to delete database: {}", e))?;
        }
        
        Ok(())
    }

    /// 获取数据库连接
    pub fn get_connection(&self, db_name: &str) -> Result<Arc<Mutex<Connection>>, String> {
        let connections = self.connections.lock().unwrap();
        connections
            .get(db_name)
            .cloned()
            .ok_or_else(|| format!("Database '{}' not found", db_name))
    }

    /// 获取数据库文件路径
    fn get_db_path(&self, db_name: &str) -> PathBuf {
        let mut path = std::env::current_dir().unwrap_or_else(|_| PathBuf::from("."));
        path.push("databases");
        std::fs::create_dir_all(&path).ok();
        path.push(format!("{}.db", db_name));
        path
    }
}

impl Default for DatabaseManager {
    fn default() -> Self {
        Self::new()
    }
}

/// 数据库包装器（用于单个数据库操作）
pub struct Database {
    conn: Arc<Mutex<Connection>>,
}

impl Database {
    pub fn new(conn: Arc<Mutex<Connection>>) -> Self {
        Self { conn }
    }

    pub fn get_connection(&self) -> Arc<Mutex<Connection>> {
        self.conn.clone()
    }
}
