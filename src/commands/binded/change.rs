// use crate::reusable::{config::Init, component::error::ErrorLog, postgress::PgConn};
//
// //Waiting For request
// fn modal_register_row(name:&str,pass:bool)->CreateActionRow{
//     let placeholder = match pass {
//         false => "your MHFZ username on launcher".to_owned(),
//         true => "your MHFZ user password (igonore discord warning)".to_owned(),
//     };
//     let mut row = CreateActionRow::default();
//     row.create_input_text(|i|{
//         i.label(name)
//          .custom_id(name)
//          .required(true)
//          .style(serenity::model::prelude::component::InputTextStyle::Short)
//          .placeholder(&placeholder)
//     });
//     row
// }
//
// fn modal_response<'a,'b>(lt:&'a mut CreateInteractionResponse<'b>)->&'a mut CreateInteractionResponse<'b>{
//     lt.kind(serenity::model::prelude::interaction::InteractionResponseType::Modal)
//        .interaction_response_data(|m|{
//             m.components(|c|c.add_action_row(modal_register_row("username",false))
//                .add_action_row(modal_register_row("password", true)))
//                 .custom_id("register_m")
//                 .title("change your main user")
//     })
// }
// pub async fn run(ctx:&Context,cmd:&ApplicationCommandInteraction,init:&Init){
//     let mut err = ErrorLog::new(&ctx, init, &cmd.user).await;
//     let did = cmd.user.id.to_string();
//     match PgConn::create(init, did).await {
//         Ok(mut pg) =>{
//             match pg.get_user_data().await {
//                 Ok(data) => {
//                     if data.cid == 0 && data.rid==0{
//                         err.change_error("you doesnt have recorded account use `/create` to create account".to_string(), "checking user data", "you cant have more than one account sorry".to_string());
//                         err.log_slash(cmd, false).await;
//                         return pg.close().await;
//                     }
//                     pg.close().await;
//                 }
//                 Err(why) => {
//                     err.change_error(why.to_string(), "getting user data", "please report this".to_string());
//                     err.log_slash(cmd, false).await;
//                     return pg.close().await;
//                 }
//             }
//         }
//         Err(why) => {
//             err.pgcon_error(why.to_string(), "create button", cmd).await;
//             return;
//         }
//     };
//     if let Err(why) = cmd.create_interaction_response(&ctx.http, |r|{
//         modal_response(r)
//     }).await{
//         let mut err = ErrorLog::new(&ctx, init, &cmd.user).await;
//         err.change_error(why.to_string(), "register interface button", "failed to response, most likely your registrasion already done, its just discord error".to_string());
//         err.log_error_channel().await;
//     }
// }
