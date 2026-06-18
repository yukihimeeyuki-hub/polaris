import { invoke } from '@tauri-apps/api/core'

export const db = {
  /** 创建数据库 */
  async create(dbName: string): Promise<void> {
    return invoke('db_create', { dbName })
  },

  /** 删除数据库 */
  async delete(dbName: string): Promise<void> {
    return invoke('db_delete', { dbName })
  },

  /** 创建表 */
  async createTable(
    dbName: string,
    tableName: string,
    columns: [string, string][]
  ): Promise<void> {
    return invoke('db_create_table', { dbName, tableName, columns })
  },

  /** 删除表 */
  async dropTable(dbName: string, tableName: string): Promise<void> {
    return invoke('db_drop_table', { dbName, tableName })
  },

  /** 添加列 */
  async addColumn(
    dbName: string,
    tableName: string,
    columnName: string,
    columnType: string
  ): Promise<void> {
    return invoke('db_add_column', { dbName, tableName, columnName, columnType })
  },

  /** 重命名列 */
  async renameColumn(
    dbName: string,
    tableName: string,
    oldName: string,
    newName: string
  ): Promise<void> {
    return invoke('db_rename_column', { dbName, tableName, oldName, newName })
  },

  /** 列出所有表 */
  async listTables(dbName: string): Promise<string[]> {
    return invoke('db_list_tables', { dbName })
  },

  /** 插入数据 */
  async insert(dbName: string, tableName: string, data: Record<string, unknown>): Promise<number> {
    return invoke('db_insert', { dbName, tableName, data })
  },

  /** 批量插入数据 */
  async insertBatch(
    dbName: string,
    tableName: string,
    dataList: Record<string, unknown>[]
  ): Promise<number> {
    return invoke('db_insert_batch', { dbName, tableName, dataList })
  },

  /** 更新数据 */
  async update(
    dbName: string,
    tableName: string,
    data: Record<string, unknown>,
    whereClause: string
  ): Promise<number> {
    return invoke('db_update', { dbName, tableName, data, whereClause })
  },

  /** 删除数据 */
  async deleteData(dbName: string, tableName: string, whereClause: string): Promise<number> {
    return invoke('db_delete_data', { dbName, tableName, whereClause })
  },

  /** 查询数据 */
  async select(
    dbName: string,
    tableName: string,
    columns: string[] = [],
    whereClause?: string
  ): Promise<Record<string, unknown>[]> {
    return invoke('db_select', { dbName, tableName, columns, whereClause })
  },

  /** 查询单条数据 */
  async selectOne(
    dbName: string,
    tableName: string,
    columns: string[] = [],
    whereClause: string
  ): Promise<Record<string, unknown> | null> {
    return invoke('db_select_one', { dbName, tableName, columns, whereClause })
  },
}
