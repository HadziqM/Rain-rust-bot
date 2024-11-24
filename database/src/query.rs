use std::ops::Deref;

use super::*;
use chrono::*;
use sqlx::postgres::PgRow;
use sqlx::{Column, Decode, Postgres, Row, ValueRef};

impl Db {
    pub async fn query(&self, qry: &str) -> DbResult<String> {
        match sqlx::query(qry).fetch_all(self.deref()).await {
            Ok(fetch) => Ok(row_to_table(fetch)?),
            Err(err) => Err(format!(" Query Error: {err}, fix your sql syntax").into()),
        }
    }
    pub async fn execute(&self, qry: &str) -> DbResult<()> {
        match sqlx::query(qry).execute(self.deref()).await {
            Ok(_) => Ok(()),
            Err(err) => Err(format!(" Query Error: {err}, fix your sql syntax").into()),
        }
    }
}
fn get_name_type(row: &PgRow) -> DbResult<String> {
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
fn get_value(row: PgRow) -> DbResult<String> {
    let mut string = vec!["|".to_string()];

    for i in row.columns() {
        let value = row.try_get_raw(i.ordinal())?;
        let name = value.type_info().to_string();

        let val = if value.is_null() {
            "NULL".to_string()
        } else {
            match name.as_str() {
                "INT" | "SERIAL" | "INT4" => <i32 as Decode<Postgres>>::decode(value)
                    .unwrap()
                    .to_string(),
                "BIGINT" | "BIGSERIAL" | "INT8" => <i64 as Decode<Postgres>>::decode(value)
                    .unwrap()
                    .to_string(),
                "VARCHAR" | "CHAR(N)" | "TEXT" | "NAME" => {
                    <&str as Decode<Postgres>>::decode(value)
                        .unwrap()
                        .to_string()
                }
                "BOOL" => <bool as Decode<Postgres>>::decode(value)
                    .unwrap()
                    .to_string(),
                "TIMESTAMPTZ" | "TIMESTAMP" => {
                    let timestamp: NaiveDateTime =
                        <NaiveDateTime as Decode<Postgres>>::decode(value).unwrap();
                    timestamp.to_string()
                }
                "DATE" => {
                    let date: NaiveDate = <NaiveDate as Decode<Postgres>>::decode(value).unwrap();
                    date.to_string()
                }
                "TIME" => {
                    let time: NaiveTime = <NaiveTime as Decode<Postgres>>::decode(value).unwrap();
                    time.to_string()
                }
                _ => format!("[{}]", name),
            }
        };

        string.push(format!("{}|", val));
    }

    Ok(string.concat())
}
fn row_to_table(row: Vec<PgRow>) -> DbResult<String> {
    let name = match row.first() {
        Some(x) => get_name_type(x)?,
        None => {
            return Err(" There is no data in your query".into());
        }
    };
    let mut res = vec!["```".to_string(), name];
    for pat in row {
        let data = get_value(pat)?;
        res.push("\n".to_string());
        res.push(data)
    }
    res.push("```".to_string());
    Ok(res.concat())
}
