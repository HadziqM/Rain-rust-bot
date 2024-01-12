use super::{BitwiseError, PgConn};
use sqlx::postgres::PgRow;
use sqlx::{Column, Decode, Postgres, Row, ValueRef};

impl PgConn<'_> {
    pub async fn query(&self, qry: &str) -> Result<String, BitwiseError> {
        let fetch = sqlx::query(qry).fetch_all(&self.pool).await?;
        Ok(row_to_table(fetch)?)
    }
    pub async fn execute(&self, qry: &str) -> Result<(), BitwiseError> {
        sqlx::query(qry).execute(&self.pool).await?;
        Ok(())
    }
}
fn get_name_type(row: &PgRow) -> Result<String, BitwiseError> {
    let mut string = Vec::new();
    for i in row.columns() {
        let name = i.name();
        let typ = row.try_get_raw(i.ordinal())?;
        string.push("\t".to_string());
        string.push(format!("{}({})", name, typ.type_info().to_string()));
    }
    Ok(string[1..].concat())
}
fn get_value(row: PgRow) -> Result<String, BitwiseError> {
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
fn row_to_table(row: Vec<PgRow>) -> Result<String, BitwiseError> {
    let name = match row.first() {
        Some(x) => get_name_type(x)?,
        None => {
            return Err(BitwiseError::NoItem);
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
