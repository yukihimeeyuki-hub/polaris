use crate::database::Database;
use rusqlite::{params, Result};
use serde::{Deserialize, Serialize};
use serde_json::Value;
use std::sync::Arc;

/// CRUD 操作管理器
pub struct CrudOperations {
    db: Database,
}

impl CrudOperations {
    pub fn new(db: Database) -> Self {
        Self { db }
    }

    /// 插入数据
    pub fn insert(&self, table_name: &str, data: &Value) -> Result<i64, String> {
        let conn = self.db.get_connection();
        let conn = conn.lock().unwrap();
        
        let obj = data.as_object()
            .ok_or_else(|| "Data must be a JSON object".to_string())?;
        
        let columns: Vec<&str> = obj.keys().map(|k| k.as_str()).collect();
        let placeholders: Vec<String> = (0..columns.len()).map(|i| format!("?{}", i + 1)).collect();
        
        let sql = format!(
            "INSERT INTO {} ({}) VALUES ({})",
            table_name,
            columns.join(", "),
            placeholders.join(", ")
        );
        
        let values: Vec<Box<dyn rusqlite::types::ToSql>> = obj
            .values()
            .map(|v| Self::value_to_sql(v))
            .collect::<Result<Vec<_>, _>>()?;
        
        let params: Vec<&dyn rusqlite::types::ToSql> = values.iter().map(|v| v.as_ref()).collect();
        
        conn.execute(&sql, params.as_slice())
            .map_err(|e| format!("Failed to insert: {}", e))?;
        
        Ok(conn.last_insert_rowid())
    }

    /// 批量插入数据
    pub fn insert_batch(&self, table_name: &str, data_list: &[Value]) -> Result<usize, String> {
        let conn = self.db.get_connection();
        let conn = conn.lock().unwrap();
        
        if data_list.is_empty() {
            return Ok(0);
        }
        
        let first_obj = data_list[0].as_object()
            .ok_or_else(|| "Data must be a JSON object".to_string())?;
        
        let columns: Vec<&str> = first_obj.keys().map(|k| k.as_str()).collect();
        let placeholders: Vec<String> = (0..columns.len()).map(|i| format!("?{}", i + 1)).collect();
        
        let sql = format!(
            "INSERT INTO {} ({}) VALUES ({})",
            table_name,
            columns.join(", "),
            placeholders.join(", ")
        );
        
        let mut count = 0;
        for data in data_list {
            let obj = data.as_object()
                .ok_or_else(|| "Data must be a JSON object".to_string())?;
            
            let values: Vec<Box<dyn rusqlite::types::ToSql>> = obj
                .values()
                .map(|v| Self::value_to_sql(v))
                .collect::<Result<Vec<_>, _>>()?;
            
            let params: Vec<&dyn rusqlite::types::ToSql> = values.iter().map(|v| v.as_ref()).collect();
            
            conn.execute(&sql, params.as_slice())
                .map_err(|e| format!("Failed to insert: {}", e))?;
            
            count += 1;
        }
        
        Ok(count)
    }

    /// 更新数据
    pub fn update(&self, table_name: &str, data: &Value, where_clause: &str) -> Result<usize, String> {
        let conn = self.db.get_connection();
        let conn = conn.lock().unwrap();
        
        let obj = data.as_object()
            .ok_or_else(|| "Data must be a JSON object".to_string())?;
        
        let set_clauses: Vec<String> = obj
            .keys()
            .enumerate()
            .map(|(i, k)| format!("{} = ?{}", k, i + 1))
            .collect();
        
        let sql = format!(
            "UPDATE {} SET {} WHERE {}",
            table_name,
            set_clauses.join(", "),
            where_clause
        );
        
        let values: Vec<Box<dyn rusqlite::types::ToSql>> = obj
            .values()
            .map(|v| Self::value_to_sql(v))
            .collect::<Result<Vec<_>, _>>()?;
        
        let params: Vec<&dyn rusqlite::types::ToSql> = values.iter().map(|v| v.as_ref()).collect();
        
        let count = conn
            .execute(&sql, params.as_slice())
            .map_err(|e| format!("Failed to update: {}", e))?;
        
        Ok(count)
    }

    /// 删除数据
    pub fn delete(&self, table_name: &str, where_clause: &str) -> Result<usize, String> {
        let conn = self.db.get_connection();
        let conn = conn.lock().unwrap();
        
        let sql = format!("DELETE FROM {} WHERE {}", table_name, where_clause);
        
        let count = conn
            .execute(&sql, [])
            .map_err(|e| format!("Failed to delete: {}", e))?;
        
        Ok(count)
    }

    /// 查询数据
    pub fn select(&self, table_name: &str, columns: &[&str], where_clause: Option<&str>) -> Result<Vec<Value>, String> {
        let conn = self.db.get_connection();
        let conn = conn.lock().unwrap();
        
        let columns_sql = if columns.is_empty() {
            "*".to_string()
        } else {
            columns.join(", ")
        };
        
        let sql = if let Some(where_clause) = where_clause {
            format!("SELECT {} FROM {} WHERE {}", columns_sql, table_name, where_clause)
        } else {
            format!("SELECT {} FROM {}", columns_sql, table_name)
        };
        
        let mut stmt = conn
            .prepare(&sql)
            .map_err(|e| format!("Failed to prepare statement: {}", e))?;
        
        let column_names: Vec<String> = stmt
            .column_names()
            .iter()
            .map(|s| s.to_string())
            .collect();
        
        let rows = stmt
            .query_map([], |row| {
                let mut obj = serde_json::Map::new();
                for (i, col_name) in column_names.iter().enumerate() {
                    let value: rusqlite::types::Value = row.get(i)?;
                    obj.insert(col_name.clone(), Self::sqlite_value_to_json(value));
                }
                Ok(Value::Object(obj))
            })
            .map_err(|e| format!("Failed to query: {}", e))?
            .collect::<Result<Vec<_>, _>>()
            .map_err(|e| format!("Failed to collect rows: {}", e))?;
        
        Ok(rows)
    }

    /// 查询单条数据
    pub fn select_one(&self, table_name: &str, columns: &[&str], where_clause: &str) -> Result<Option<Value>, String> {
        let results = self.select(table_name, columns, Some(where_clause))?;
        Ok(results.into_iter().next())
    }

    /// 将 JSON Value 转换为 SQL 值
    fn value_to_sql(value: &Value) -> Result<Box<dyn rusqlite::types::ToSql>, String> {
        match value {
            Value::Null => Ok(Box::new(rusqlite::types::Null)),
            Value::Bool(b) => Ok(Box::new(*b)),
            Value::Number(n) => {
                if let Some(i) = n.as_i64() {
                    Ok(Box::new(i))
                } else if let Some(f) = n.as_f64() {
                    Ok(Box::new(f))
                } else {
                    Err("Invalid number".to_string())
                }
            }
            Value::String(s) => Ok(Box::new(s.clone())),
            _ => Err("Unsupported JSON type".to_string()),
        }
    }

    /// 将 SQLite Value 转换为 JSON Value
    fn sqlite_value_to_json(value: rusqlite::types::Value) -> Value {
        match value {
            rusqlite::types::Value::Null => Value::Null,
            rusqlite::types::Value::Integer(i) => Value::Number(i.into()),
            rusqlite::types::Value::Real(f) => {
                serde_json::Number::from_f64(f)
                    .map(Value::Number)
                    .unwrap_or(Value::Null)
            }
            rusqlite::types::Value::Text(s) => Value::String(s),
            rusqlite::types::Value::Blob(b) => {
                Value::String(base64::Engine::encode(&base64::engine::general_purpose::STANDARD, &b))
            }
        }
    }
}
