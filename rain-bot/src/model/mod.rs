pub mod user;
pub mod components;


use crate::Context;

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
