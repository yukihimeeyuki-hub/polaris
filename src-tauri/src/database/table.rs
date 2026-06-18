use crate::database::Database;
use rusqlite::Result;
use std::sync::Arc;

/// 表管理器
pub struct TableManager {
    db: Database,
}

impl TableManager {
    pub fn new(db: Database) -> Self {
        Self { db }
    }

    /// 创建表
    pub fn create_table(&self, table_name: &str, columns: &[(&str, &str)]) -> Result<(), String> {
        let conn = self.db.get_connection();
        let conn = conn.lock().unwrap();
        
        let columns_sql: Vec<String> = columns
            .iter()
            .map(|(name, col_type)| format!("{} {}", name, col_type))
            .collect();
        
        let sql = format!(
            "CREATE TABLE IF NOT EXISTS {} ({})",
            table_name,
            columns_sql.join(", ")
        );
        
        conn.execute(&sql, [])
            .map_err(|e| format!("Failed to create table: {}", e))?;
        
        Ok(())
    }

    /// 删除表
    pub fn drop_table(&self, table_name: &str) -> Result<(), String> {
        let conn = self.db.get_connection();
        let conn = conn.lock().unwrap();
        
        let sql = format!("DROP TABLE IF EXISTS {}", table_name);
        
        conn.execute(&sql, [])
            .map_err(|e| format!("Failed to drop table: {}", e))?;
        
        Ok(())
    }

    /// 修改表 - 添加列
    pub fn add_column(&self, table_name: &str, column_name: &str, column_type: &str) -> Result<(), String> {
        let conn = self.db.get_connection();
        let conn = conn.lock().unwrap();
        
        let sql = format!(
            "ALTER TABLE {} ADD COLUMN {} {}",
            table_name, column_name, column_type
        );
        
        conn.execute(&sql, [])
            .map_err(|e| format!("Failed to add column: {}", e))?;
        
        Ok(())
    }

    /// 修改表 - 重命名列
    pub fn rename_column(
        &self,
        table_name: &str,
        old_name: &str,
        new_name: &str,
    ) -> Result<(), String> {
        let conn = self.db.get_connection();
        let conn = conn.lock().unwrap();
        
        let sql = format!(
            "ALTER TABLE {} RENAME COLUMN {} TO {}",
            table_name, old_name, new_name
        );
        
        conn.execute(&sql, [])
            .map_err(|e| format!("Failed to rename column: {}", e))?;
        
        Ok(())
    }

    /// 检查表是否存在
    pub fn table_exists(&self, table_name: &str) -> Result<bool, String> {
        let conn = self.db.get_connection();
        let conn = conn.lock().unwrap();
        
        let sql = "SELECT name FROM sqlite_master WHERE type='table' AND name=?";
        let mut stmt = conn
            .prepare(sql)
            .map_err(|e| format!("Failed to prepare statement: {}", e))?;
        
        let exists = stmt
            .exists(rusqlite::params![table_name])
            .map_err(|e| format!("Failed to check table existence: {}", e))?;
        
        Ok(exists)
    }

    /// 获取所有表名
    pub fn list_tables(&self) -> Result<Vec<String>, String> {
        let conn = self.db.get_connection();
        let conn = conn.lock().unwrap();
        
        let sql = "SELECT name FROM sqlite_master WHERE type='table'";
        let mut stmt = conn
            .prepare(sql)
            .map_err(|e| format!("Failed to prepare statement: {}", e))?;
        
        let tables = stmt
            .query_map([], |row| row.get(0))
            .map_err(|e| format!("Failed to query tables: {}", e))?
            .collect::<Result<Vec<String>, _>>()
            .map_err(|e| format!("Failed to collect tables: {}", e))?;
        
        Ok(tables)
    }
}
