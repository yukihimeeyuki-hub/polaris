use crate::database::{Database, DatabaseManager, TableManager, CrudOperations};
use serde::{Deserialize, Serialize};
use serde_json::Value;
use std::sync::Arc;
use tauri::Manager;

/// 数据库状态
pub struct DatabaseState {
    pub manager: DatabaseManager,
}

/// 创建数据库
#[tauri::command]
pub fn db_create(app: tauri::AppHandle, db_name: String) -> Result<(), String> {
    let state = app.state::<DatabaseState>();
    state.manager.create_database(&db_name)
}

/// 删除数据库
#[tauri::command]
pub fn db_delete(app: tauri::AppHandle, db_name: String) -> Result<(), String> {
    let state = app.state::<DatabaseState>();
    state.manager.delete_database(&db_name)
}

/// 创建表
#[tauri::command]
pub fn db_create_table(
    app: tauri::AppHandle,
    db_name: String,
    table_name: String,
    columns: Vec<(String, String)>,
) -> Result<(), String> {
    let state = app.state::<DatabaseState>();
    let conn = state.manager.get_connection(&db_name)?;
    let db = Database::new(conn);
    let table_mgr = TableManager::new(db);
    
    let columns_ref: Vec<(&str, &str)> = columns
        .iter()
        .map(|(name, col_type)| (name.as_str(), col_type.as_str()))
        .collect();
    
    table_mgr.create_table(&table_name, &columns_ref)
}

/// 删除表
#[tauri::command]
pub fn db_drop_table(
    app: tauri::AppHandle,
    db_name: String,
    table_name: String,
) -> Result<(), String> {
    let state = app.state::<DatabaseState>();
    let conn = state.manager.get_connection(&db_name)?;
    let db = Database::new(conn);
    let table_mgr = TableManager::new(db);
    
    table_mgr.drop_table(&table_name)
}

/// 添加列
#[tauri::command]
pub fn db_add_column(
    app: tauri::AppHandle,
    db_name: String,
    table_name: String,
    column_name: String,
    column_type: String,
) -> Result<(), String> {
    let state = app.state::<DatabaseState>();
    let conn = state.manager.get_connection(&db_name)?;
    let db = Database::new(conn);
    let table_mgr = TableManager::new(db);
    
    table_mgr.add_column(&table_name, &column_name, &column_type)
}

/// 重命名列
#[tauri::command]
pub fn db_rename_column(
    app: tauri::AppHandle,
    db_name: String,
    table_name: String,
    old_name: String,
    new_name: String,
) -> Result<(), String> {
    let state = app.state::<DatabaseState>();
    let conn = state.manager.get_connection(&db_name)?;
    let db = Database::new(conn);
    let table_mgr = TableManager::new(db);
    
    table_mgr.rename_column(&table_name, &old_name, &new_name)
}

/// 列出所有表
#[tauri::command]
pub fn db_list_tables(app: tauri::AppHandle, db_name: String) -> Result<Vec<String>, String> {
    let state = app.state::<DatabaseState>();
    let conn = state.manager.get_connection(&db_name)?;
    let db = Database::new(conn);
    let table_mgr = TableManager::new(db);
    
    table_mgr.list_tables()
}

/// 插入数据
#[tauri::command]
pub fn db_insert(
    app: tauri::AppHandle,
    db_name: String,
    table_name: String,
    data: Value,
) -> Result<i64, String> {
    let state = app.state::<DatabaseState>();
    let conn = state.manager.get_connection(&db_name)?;
    let db = Database::new(conn);
    let crud = CrudOperations::new(db);
    
    crud.insert(&table_name, &data)
}

/// 批量插入数据
#[tauri::command]
pub fn db_insert_batch(
    app: tauri::AppHandle,
    db_name: String,
    table_name: String,
    data_list: Vec<Value>,
) -> Result<usize, String> {
    let state = app.state::<DatabaseState>();
    let conn = state.manager.get_connection(&db_name)?;
    let db = Database::new(conn);
    let crud = CrudOperations::new(db);
    
    crud.insert_batch(&table_name, &data_list)
}

/// 更新数据
#[tauri::command]
pub fn db_update(
    app: tauri::AppHandle,
    db_name: String,
    table_name: String,
    data: Value,
    where_clause: String,
) -> Result<usize, String> {
    let state = app.state::<DatabaseState>();
    let conn = state.manager.get_connection(&db_name)?;
    let db = Database::new(conn);
    let crud = CrudOperations::new(db);
    
    crud.update(&table_name, &data, &where_clause)
}

/// 删除数据
#[tauri::command]
pub fn db_delete_data(
    app: tauri::AppHandle,
    db_name: String,
    table_name: String,
    where_clause: String,
) -> Result<usize, String> {
    let state = app.state::<DatabaseState>();
    let conn = state.manager.get_connection(&db_name)?;
    let db = Database::new(conn);
    let crud = CrudOperations::new(db);
    
    crud.delete(&table_name, &where_clause)
}

/// 查询数据
#[tauri::command]
pub fn db_select(
    app: tauri::AppHandle,
    db_name: String,
    table_name: String,
    columns: Vec<String>,
    where_clause: Option<String>,
) -> Result<Vec<Value>, String> {
    let state = app.state::<DatabaseState>();
    let conn = state.manager.get_connection(&db_name)?;
    let db = Database::new(conn);
    let crud = CrudOperations::new(db);
    
    let columns_ref: Vec<&str> = columns.iter().map(|s| s.as_str()).collect();
    let where_ref = where_clause.as_deref();
    
    crud.select(&table_name, &columns_ref, where_ref)
}

/// 查询单条数据
#[tauri::command]
pub fn db_select_one(
    app: tauri::AppHandle,
    db_name: String,
    table_name: String,
    columns: Vec<String>,
    where_clause: String,
) -> Result<Option<Value>, String> {
    let state = app.state::<DatabaseState>();
    let conn = state.manager.get_connection(&db_name)?;
    let db = Database::new(conn);
    let crud = CrudOperations::new(db);
    
    let columns_ref: Vec<&str> = columns.iter().map(|s| s.as_str()).collect();
    
    crud.select_one(&table_name, &columns_ref, &where_clause)
}

/// 初始化数据库管理器
pub fn init_database_manager() -> DatabaseState {
    DatabaseState {
        manager: DatabaseManager::new(),
    }
}
