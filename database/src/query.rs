use super::*;
use sqlx::postgres::PgRow;
use sqlx::sqlite::SqliteRow;
use sqlx::{Column, Row, TypeInfo};

pub enum DatabaseType {
    Postgres,
    Sqlite,
}

enum DataTable {
    PostGres(Vec<PgRow>),
    Sqlite(Vec<SqliteRow>),
}

impl Db {
    pub async fn query(&self, db: DatabaseType, qry: &str) -> DbResult<String> {
        let data: DataTable = match db {
            DatabaseType::Postgres => {
                DataTable::PostGres(sqlx::raw_sql(qry).fetch_all(self.post.pool()).await?)
            }
            DatabaseType::Sqlite => {
                DataTable::Sqlite(sqlx::raw_sql(qry).fetch_all(self.lite.pool()).await?)
            }
        };

        row_to_table(data)
    }
    pub async fn execute(&self, db: DatabaseType, qry: &str) -> DbResult<u64> {
        match db {
            DatabaseType::Postgres => {
                let x = sqlx::raw_sql(qry).execute(self.post.pool()).await?;
                return Ok(x.rows_affected());
            }
            DatabaseType::Sqlite => {
                let x = sqlx::raw_sql(qry).execute(self.lite.pool()).await?;
                return Ok(x.rows_affected());
            }
        }
    }
}
fn get_name_type<R: Row>(row: &R) -> DbResult<String> {
    let mut string = vec!["|".to_string()];
    for i in row.columns() {
        let name = i.name();
        string.push(format!("{name}|"));
    }

    // Add the separator row
    string.push("\n|".to_string()); // New line for separator
    for i in row.columns() {
        // Add a separator for each column (e.g., "-" repeated for the name length)
        let separator = "-".repeat(i.name().len());
        string.push(format!("{separator}|"));
    }
    Ok(string.concat())
}

pub fn get_value(row: PgRow) -> DbResult<String> {
    let mut string = vec!["|".to_string()];

    for column in row.columns() {
        let idx = column.ordinal();
        let type_name = column.type_info().name();

        let val = match type_name {
            "INT" | "SERIAL" | "INT4" => row.try_get::<i32, _>(idx).map(|v| v.to_string()),
            "BIGINT" | "BIGSERIAL" | "INT8" => row.try_get::<i64, _>(idx).map(|v| v.to_string()),
            "VARCHAR" | "CHAR" | "TEXT" | "NAME" => row.try_get::<String, _>(idx),
            "BOOL" => row.try_get::<bool, _>(idx).map(|v| v.to_string()),
            "TIMESTAMP" | "TIMESTAMPTZ" => row
                .try_get::<chrono::NaiveDateTime, _>(idx)
                .map(|v| v.to_string()),
            "DATE" => row
                .try_get::<chrono::NaiveDate, _>(idx)
                .map(|v| v.to_string()),
            "TIME" => row
                .try_get::<chrono::NaiveTime, _>(idx)
                .map(|v| v.to_string()),
            _ => Ok(format!("[{}]", type_name)),
        };

        let formatted = match val {
            Ok(v) => v,
            Err(_) => "NULL".to_string(),
        };

        string.push(format!("{formatted}|"));
    }

    Ok(string.concat())
}

pub fn get_value_sqlite(row: SqliteRow) -> DbResult<String> {
    let mut string = vec!["|".to_string()];

    for column in row.columns() {
        let idx = column.ordinal();
        let type_name = column.type_info().name();

        let val = match type_name {
            "INT" | "SERIAL" | "INT4" => row.try_get::<i32, _>(idx).map(|v| v.to_string()),
            "BIGINT" | "BIGSERIAL" | "INT8" => row.try_get::<i64, _>(idx).map(|v| v.to_string()),
            "VARCHAR" | "CHAR" | "TEXT" | "NAME" => row.try_get::<String, _>(idx),
            "BOOL" => row.try_get::<bool, _>(idx).map(|v| v.to_string()),
            "TIMESTAMP" | "TIMESTAMPTZ" => row
                .try_get::<chrono::NaiveDateTime, _>(idx)
                .map(|v| v.to_string()),
            "DATE" => row
                .try_get::<chrono::NaiveDate, _>(idx)
                .map(|v| v.to_string()),
            "TIME" => row
                .try_get::<chrono::NaiveTime, _>(idx)
                .map(|v| v.to_string()),
            _ => Ok(format!("[{}]", type_name)),
        };

        let formatted = match val {
            Ok(v) => v,
            Err(_) => "NULL".to_string(),
        };

        string.push(format!("{formatted}|"));
    }

    Ok(string.concat())
}

fn row_to_table(tb: DataTable) -> DbResult<String> {
    let mut res = vec!["```".to_string()];
    match tb {
        DataTable::Sqlite(row) => {
            let name = match row.first() {
                Some(x) => get_name_type(x)?,
                None => {
                    return Err(" There is no data in your query".into());
                }
            };
            res.push(name);
            for pat in row {
                let data = get_value_sqlite(pat)?;
                res.push("\n".to_string());
                res.push(data)
            }
        }
        DataTable::PostGres(row) => {
            let name = match row.first() {
                Some(x) => get_name_type(x)?,
                None => {
                    return Err(" There is no data in your query".into());
                }
            };
            res.push(name);
            for pat in row {
                let data = get_value(pat)?;
                res.push("\n".to_string());
                res.push(data)
            }
        }
    };
    res.push("```".to_string());
    Ok(res.concat())
}
