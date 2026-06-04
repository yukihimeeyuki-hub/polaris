use rusqlite::{Connection, Result, Row, ToSql};

pub struct Database {
    conn: Connection,
}

impl Database {
    /// 打开数据库
    pub fn new(db_path: &str) -> Result<Self> {
        let conn = Connection::open(db_path)?;
        Ok(Self { conn })
    }

    // ==========================
    // 表操作
    // ==========================

    /// 创建表
    ///
    /// example:
    /// create_table(
    ///     "users",
    ///     "id INTEGER PRIMARY KEY AUTOINCREMENT,
    ///      name TEXT NOT NULL,
    ///      age INTEGER"
    /// )
    pub fn create_table(
        &self,
        table_name: &str,
        columns: &str,
    ) -> Result<()> {
        let sql = format!(
            "CREATE TABLE IF NOT EXISTS {} ({})",
            table_name, columns
        );

        self.conn.execute(&sql, [])?;
        Ok(())
    }

    /// 删除表
    pub fn drop_table(&self, table_name: &str) -> Result<()> {
        let sql = format!("DROP TABLE IF EXISTS {}", table_name);
        self.conn.execute(&sql, [])?;
        Ok(())
    }

    /// 执行 ALTER TABLE
    pub fn alter_table(&self, sql: &str) -> Result<()> {
        self.conn.execute(sql, [])?;
        Ok(())
    }

    /// 判断表是否存在
    pub fn table_exists(&self, table_name: &str) -> Result<bool> {
        let mut stmt = self.conn.prepare(
            "SELECT COUNT(*) FROM sqlite_master
             WHERE type='table' AND name=?1"
        )?;

        let count: i64 = stmt.query_row([table_name], |row| row.get(0))?;

        Ok(count > 0)
    }

    /// 获取所有表名
    pub fn list_tables(&self) -> Result<Vec<String>> {
        let mut stmt = self.conn.prepare(
            "SELECT name FROM sqlite_master
             WHERE type='table'
             ORDER BY name"
        )?;

        let tables = stmt
            .query_map([], |row| row.get::<_, String>(0))?
            .collect::<Result<Vec<_>, _>>()?;

        Ok(tables)
    }

    // ==========================
    // 数据操作
    // ==========================

    /// 插入数据
    pub fn insert(
        &self,
        table_name: &str,
        columns: &[&str],
        values: &[&dyn ToSql],
    ) -> Result<usize> {
        let cols = columns.join(",");

        let placeholders = (1..=columns.len())
            .map(|i| format!("?{}", i))
            .collect::<Vec<_>>()
            .join(",");

        let sql = format!(
            "INSERT INTO {} ({}) VALUES ({})",
            table_name,
            cols,
            placeholders
        );

        self.conn.execute(&sql, values)
    }

    /// 更新数据
    ///
    /// example:
    /// update(
    ///     "users",
    ///     "name=?1, age=?2",
    ///     &[&"Tom", &18, &1]
    /// )
    pub fn update(
        &self,
        table_name: &str,
        set_clause: &str,
        condition: &str,
        params: &[&dyn ToSql],
    ) -> Result<usize> {
        let sql = format!(
            "UPDATE {} SET {} WHERE {}",
            table_name,
            set_clause,
            condition
        );

        self.conn.execute(&sql, params)
    }

    /// 删除数据
    pub fn delete(
        &self,
        table_name: &str,
        condition: &str,
        params: &[&dyn ToSql],
    ) -> Result<usize> {
        let sql = format!(
            "DELETE FROM {} WHERE {}",
            table_name,
            condition
        );

        self.conn.execute(&sql, params)
    }

    /// 判断数据是否存在
    pub fn data_exists(
        &self,
        table_name: &str,
        condition: &str,
        params: &[&dyn ToSql],
    ) -> Result<bool> {
        let sql = format!(
            "SELECT EXISTS(
                SELECT 1 FROM {}
                WHERE {}
                LIMIT 1
            )",
            table_name,
            condition
        );

        let exists: i32 =
            self.conn.query_row(&sql, params, |row| row.get(0))?;

        Ok(exists == 1)
    }

    /// 查询单条记录
    pub fn query_one<T, F>(
        &self,
        sql: &str,
        params: &[&dyn ToSql],
        mapper: F,
    ) -> Result<T>
    where
        F: FnOnce(&Row) -> Result<T>,
    {
        self.conn.query_row(sql, params, mapper)
    }

    /// 查询多条记录
    pub fn query_all<T, F>(
        &self,
        sql: &str,
        params: &[&dyn ToSql],
        mut mapper: F,
    ) -> Result<Vec<T>>
    where
        F: FnMut(&Row) -> Result<T>,
    {
        let mut stmt = self.conn.prepare(sql)?;

        let rows = stmt.query_map(params, |row| mapper(row))?;

        let mut result = Vec::new();

        for row in rows {
            result.push(row?);
        }

        Ok(result)
    }
}