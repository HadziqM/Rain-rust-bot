use crate::{Components, MsgBundle, MyErr, PgConn};
use serenity::all::*;

struct Code {
    code: String,
}
impl Default for Code {
    fn default() -> Self {
        Code {
            code: String::new(),
        }
    }
}
impl Code {
    fn new(cont: String) -> Result<Code, MyErr> {
        let mut code = Vec::new();
        let mut state = false;
        let vec: Vec<_> = cont.split("\n").collect();
        for x in vec {
            if x.starts_with("```") {
                state = !state;
            }
            if state {
                code.push(x);
            }
        }
        if code.len() == 0 {
            return Err(MyErr::Custom("cant detect code in your message".to_owned()));
        }
        Ok(Code {
            code: code[1..].concat(),
        })
    }
}

#[hertz::hertz_msg(true)]
async fn msg(bnd: &MsgBundle<'_>) -> Result<(), MyErr> {
    let code = Code::new(bnd.msg.content.clone())?;
    let pg = PgConn::create(bnd.init, bnd.msg.author.id.to_string()).await?;
    pg.execute(&code.code).await?;
    Components::msg(bnd, "success").await?;
    Ok(())
}

#[hertz::hertz_msg(true)]
async fn qry(bnd: &MsgBundle<'_>) -> Result<(), MyErr> {
    let code = Code::new(bnd.msg.content.clone())?;
    let pg = PgConn::create(bnd.init, bnd.msg.author.id.to_string()).await?;
    let table = pg.query(&code.code).await?;
    Components::msg(bnd, &table).await?;
    Ok(())
}
