use super::{Db, PgCustomError};
use sqlx::postgres::PgRow;
use sqlx::{Column, Decode, Postgres, Row, ValueRef};

impl Db {
    pub async fn query(&self, qry: &str) -> Result<String, PgCustomError> {
        let fetch = sqlx::query(qry).fetch_all(&**self).await?;
        Ok(row_to_table(fetch)?)
    }
    pub async fn execute(&self, qry: &str) -> Result<(), PgCustomError> {
        sqlx::query(qry).execute(&**self).await?;
        Ok(())
    }
}
fn get_name_type(row: &PgRow) -> Result<String, PgCustomError> {
    let mut string = Vec::new();
    for i in row.columns() {
        let name = i.name();
        let typ = row.try_get_raw(i.ordinal())?;
        string.push("\t".to_string());
        string.push(format!("{}({})", name, typ.type_info().to_string()));
    }
    Ok(string[1..].concat())
}
fn get_value(row: PgRow) -> Result<String, PgCustomError> {
    let mut string = Vec::new();
    for i in row.columns() {
        let value = row.try_get_raw(i.ordinal())?;
        let nam = value.type_info().to_string();
        let name = nam.as_str();
        let i32clust = vec!["INT", "SERIAL", "INT4"];
        let i64clust = vec!["BIGINT", "BIGSERIAL", "INT8"];
        let strclust = vec!["VARCHAR", "CHAR(N)", "TEXT", "NAME"];
        let boolclust = "BOOL";
        let val;
        if value.is_null() {
            val = "NULL".to_string();
        } else {
            if i32clust.contains(&name) {
                val = <i32 as Decode<Postgres>>::decode(value)
                    .unwrap()
                    .to_string();
            } else if i64clust.contains(&name) {
                val = <i64 as Decode<Postgres>>::decode(value)
                    .unwrap()
                    .to_string();
            } else if strclust.contains(&name) {
                val = <&str as Decode<Postgres>>::decode(value)
                    .unwrap()
                    .to_string();
            } else if name == boolclust {
                val = <bool as Decode<Postgres>>::decode(value)
                    .unwrap()
                    .to_string();
            } else if name == "TIMESTAMPTZ" {
                val = format!(
                    "<t:{}:R>",
                    <i64 as Decode<Postgres>>::decode(value)
                        .unwrap()
                        .to_string()
                );
            } else if name == "BYTEA" {
                val = "[bytea]".to_owned();
            } else {
                let raw = <&[u8] as Decode<Postgres>>::decode(value).unwrap();
                val = format!("{:?}", raw);
            }
        }
        string.push("\t".to_string());
        string.push(val);
    }
    Ok(string[1..].concat())
}
fn row_to_table(row: Vec<PgRow>) -> Result<String, PgCustomError> {
    let name = match row.first() {
        Some(x) => get_name_type(x)?,
        None => {
            return Err(PgCustomError::from("no row in the table"));
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
