// use serenity::{prelude::Context, model::prelude::interaction::application_command::ApplicationCommandInteraction};
// use sqlx::Pool;
//
// use crate::reusable::config::Init;
//
// use super::{super::postgress, error:: error_rply};
//
//


// pub async fn binding_check(ctx:&Context,cmd:&ApplicationCommandInteraction,init:&Init)->Option<i32>{
//     let id = cmd.user.id.to_string();
//     match user_check(&id,init).await {
//         Ok(d) =>{
//             if d.cid != 0 {
//                 return Some(d.cid);
//             }else if d.rid != 0{
//                 //perform bind
//                 return None;
//             }
//             error_rply(ctx, "no error msg", "checking user data", "You Are Not Registered (create account) or Binded yet try to bind your account to discord or create account if dont have yet", cmd,init).await;
//             None
//         }
//         Err(why) =>{
//             error_rply(ctx, why.to_string().as_str(), "checking user user data", "connection to database timed out, just try again when traffic is more stable", cmd,init).await;
//             None
//         }
//     }
// }
