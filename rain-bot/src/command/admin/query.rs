use crate::all::*;

struct Code {
    code: String,
}
impl Code {
    fn new(cont: String) -> MyResult<Code> {
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
        if code.is_empty() {
            return Err("cant detect code in your message".into());
        }
        Ok(Code {
            code: code[1..].concat(),
        })
    }

    fn from_msg(msg: &Message) -> MyResult<Code> {
        Code::new(msg.content.clone())
    }
}

pub struct Query;
pub struct Execute;

#[async_trait]
impl MessageCommandTrait for Query {
    fn name_msg(&self) -> &'static str {
        "query"
    }

    async fn handle_msg(&self, app: Arc<App>, cmd: Message, ctx: Context) -> MyResult<()> {
        let code = Code::from_msg(&cmd)?;
        let table = app.db.query(&code.code).await?;
        cmd.reply(&ctx.http, table).await?;
        Ok(())
    }
}
#[async_trait]
impl MessageCommandTrait for Execute {
    fn name_msg(&self) -> &'static str {
        "execute"
    }

    async fn handle_msg(&self, app: Arc<App>, cmd: Message, ctx: Context) -> MyResult<()> {
        let code = Code::from_msg(&cmd)?;
        app.db.execute(&code.code).await?;
        cmd.reply(&ctx.http, "success").await?;
        Ok(())
    }
}
