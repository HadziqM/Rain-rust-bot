use super::{BitwiseError,PgConn};
use sqlx::postgres::PgRow;
use sqlx::{Row,ValueRef,Column};


impl PgConn<'_> {
    pub async fn query(&self,qry:&str)->Result<String, BitwiseError>{
        let fetch = sqlx::query(qry).fetch_all(&self.pool).await?;
        Ok(row_to_table(fetch))
    }
    pub async fn execute(&self,qry:&str)->Result<(), BitwiseError>{
        sqlx::query(qry).execute(&self.pool).await?;
        Ok(())
    }
}
fn row_to_table(row:Vec<PgRow>)->String{
    let name:String =row.first().unwrap().columns().iter().map(|x|format!("{}\t",x.name())).collect();
    let mut res = vec!["```".to_string(),name];
    while let Some(pat) = row.iter().next() {
        let data:String = pat.columns().iter().map(|x|{
            let value = pat.try_get_raw(x.ordinal()).unwrap();
            match value.is_null(){
                true=>"NULL\t".to_owned(),
                false=> format!("{}\t",value.as_str().unwrap().to_string())
            }
        }).collect();
        res.push(data)
    }
    res.push("```".to_string());
    res.concat()
}
