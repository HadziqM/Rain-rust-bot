use serenity::{model::prelude::{interaction::{application_command::{ApplicationCommandInteraction, CommandDataOptionValue}, InteractionResponseType}, component::ButtonStyle, ChannelId}, builder::{CreateEmbed, CreateActionRow}, prelude::Context};
use tokio::io::AsyncWriteExt;
use tokio::fs::File;
use crate::{Components,Init,Register};

struct FileSave{
    name:String,
    bin:Vec<u8>
}
struct SaveJudge<'a> {
    cmd: &'a ApplicationCommandInteraction,
    files:Vec<FileSave>
}
impl<'a> SaveJudge<'a> {
    async fn get_save(cmd:&'a ApplicationCommandInteraction)->SaveJudge<'a>{
        let savelist = ["savedata.bin","partner.bin","decomyset.bin",
            "hunternavi.bin","otomoairou.bin","platebox.bin",
            "platedata.bin","platemyset.bin","rengokudata.bin",
            "savemercenary.bin","skin_hist.bin"];
        let mut container = Vec::new();
        for data in &cmd.data.options{
            if let Some(resolve)= &data.resolved{
                if let CommandDataOptionValue::Attachment(e)=resolve{
                    //only accept specifict filename and less than 5mb file
                    if savelist.contains(&e.filename.as_str())&&e.size<5000000{
                        if let Ok(data)=e.download().await{
                            let file = FileSave{name:e.filename.to_owned(),bin:data};
                            container.push(file)
                        }
                    }
                }
            }
        }
        SaveJudge{cmd,files:container}
    }
    fn make_embed(&self)->CreateEmbed{
        let mut x= "```\n".to_owned();
        for i in &self.files{
            x.push_str(i.name.as_str());
            x.push_str("\n");
        }
        x.push_str("```");
        let mut emb = CreateEmbed::default();
        emb.title("Save Result").description("The savefile result that already filtered by bot and will be judged by admin later, bot will dm you if savefile is approved by admin.\n**NOTES**\n```dont login into the game untill admin approve or disaprove it then bot dm you, while transfer process run and youare on game, the save file will take no effect, also allow dm permission to let bot dm you```").author(|a|a.name(&self.cmd.user.name).icon_url(self.cmd.user.face())).field("Filtered File(s)",x.as_str(), true);
        emb
    }
    async fn save_to_file(&self)->Result<(),tokio::io::Error>{
        let mut direction = File::create(&format!("{}.txt",&self.cmd.user.id.to_string())).await?;
        let mut leeway = String::new();
        for data in &self.files{
            let name = format!("./save/{}_{}",&self.cmd.user.id.to_string()
                ,data.name.to_owned());
            let mut file = File::create(&name).await?;
            file.write_all(&data.bin.as_slice()).await?;
            leeway.push_str(&name);
            leeway.push(',');
        }
        direction.write_all(&leeway.as_bytes()).await?;
        Ok(())
    }
    fn make_button(&self)->CreateActionRow{
        let mut arow = CreateActionRow::default();
        arow.add_button(
            Components::normal_button("approve", 
                &format!("{}_save_a",&self.cmd.user.id.to_string()),
                ButtonStyle::Primary, "👍")
        ).add_button(
            Components::normal_button("disaprove", 
                &format!("{}_save_d",&self.cmd.user.id.to_string()),
                ButtonStyle::Danger,"👎")
            );
        arow
    }
}
async fn run(ctx:&Context,cmd:&ApplicationCommandInteraction,init:&Init){
    let mut reg = match Register::default(ctx, cmd, &init, "transfer save", false).await {
        Some(r)=>r,
        None=>{return ;}
    };
    match reg.pg.transfer_cd().await{
        Ok(x)=>{
            if x.0{
                let data = SaveJudge::get_save(cmd).await;
                if let Err(why)=cmd.create_interaction_response(&ctx.http, |f|{
                    f.kind(InteractionResponseType::ChannelMessageWithSource)
                    .interaction_response_data(|m|m.add_embed(data.make_embed()))
                }).await{
                    reg.error.discord_error(why.to_string(), "transfer response").await;
                }
                match data.save_to_file().await{
                    Ok(_)=>{
                        let ch = ChannelId(init.log_channel.transfer_channel);
                        if let Err(why) = ch.send_message(&ctx.http, |m|{
                            m.set_embed(data.make_embed()).components(|c|c.add_action_row(data.make_button()))
                        }).await{
                            reg.error.change_error(why.to_string(), "send save to judge", "sorry you need to report this so you could reset your cooldown".to_string());
                            reg.error.log_error_channel().await;
                        }
                    }
                    Err(_)=>{
                        reg.error.log_error_channel().await;
                    }
                }

            }else {
                reg.error.change_error("Youare Still On Cooldown".to_string(), "save transfer", format!("You need to wait till <t:{}:R> to be able to attemp transfer save again",x.1));
                reg.error.log_slash(cmd, false).await;
            }
        }
        Err(why)=>{
            reg.error.pgcon_error(why.to_string(), "get transfer cd", cmd).await;
        }
    };
    reg.pg.close().await;
}
