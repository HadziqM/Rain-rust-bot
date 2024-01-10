pub mod user;
pub mod components;
pub mod market;

use serenity::builder::CreateEmbedAuthor;

use crate::{Context, error::MyErr};

pub struct MyContext<'a>(Context<'a>);

impl<'a> std::ops::Deref for MyContext<'a> {
    type Target = Context<'a>;
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl<'a> From<Context<'a>> for MyContext<'a> {
    fn from(value: Context<'a>) -> Self {
        MyContext(value)
    }
}

impl MyContext<'_> {
    pub async fn self_assign_role(&self,role:serenity::all::RoleId) -> Result<(),MyErr> {
        let _ = self.author_member().await
            .ok_or(MyErr::from("User arent discprd server member"))?
            .add_role(**self, role).await;
        Ok(())
    }
    pub fn embed_author(&self) -> CreateEmbedAuthor {
        CreateEmbedAuthor::new(&self.author().name)
            .icon_url(self.author().face())
    }
}

