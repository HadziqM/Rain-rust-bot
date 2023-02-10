use serenity::all::{CommandInteraction, CommandOptionType, CommandDataOptionValue, ButtonStyle, ChannelId, ComponentInteraction, UserId};
use serenity::builder::{CreateEmbed, CreateEmbedAuthor, CreateActionRow, CreateMessage, EditInteractionResponse, EditMessage};
use serenity::prelude::Context;
use tokio::io::AsyncWriteExt;
use tokio::fs::File;
use crate::{Components,Init,Register,ErrorLog,PgConn};
use crate::reusable::utils::Color;
use std::num::NonZeroU64;
use std::path::Path;
use std::time::SystemTime;

pub struct FileSave{
    pub name:String,
    pub bin:Vec<u8>
}
struct SaveJudge<'a> {
    cmd: &'a CommandInteraction,
    files:Vec<FileSave>
}
impl<'a> SaveJudge<'a> {
    async fn get_save(cmd:&'a CommandInteraction)->SaveJudge<'a>{
        let savelist = ["savedata.bin","partner.bin","decomyset.bin",
            "hunternavi.bin","otomoairou.bin","platebox.bin",
            "platedata.bin","platemyset.bin","rengokudata.bin",
            "savemercenary.bin","skin_hist.bin"];
        let mut container = Vec::new();
        let resolved = &cmd.data.resolved;
        for data in &cmd.data.options{
            if let CommandOptionType::Attachment= &data.kind(){
                if let CommandDataOptionValue::Attachment(att)=data.value{
                    let e = resolved.attachments.get(&att).unwrap().to_owned();
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
        CreateEmbed::new()
        .title("Save Result").description("The savefile result that already filtered by bot and will be judged by admin later, bot will dm you if savefile is approved by admin.\n**NOTES**\n```dont login into the game untill admin approve or disaprove it then bot dm you, while transfer process run and youare on game, the save file will take no effect, also allow dm permission to let bot dm you```").author(CreateEmbedAuthor::new(&self.cmd.user.name).icon_url(self.cmd.user.face())).field("Filtered File(s)",x.as_str(), true).color(Color::Blue.throw())
    }
    async fn save_to_file(&self)->Result<(),tokio::io::Error>{
        let save = Path::new(".").join("save");
        let user = format!("{}.txt",&self.cmd.user.id.to_string());
        let mut direction = File::create(&save.join(&user)).await?;
        let mut leeway = String::new();
        for data in &self.files{
            let name = format!("{}_{}",&self.cmd.user.id.to_string()
                ,data.name.to_owned());
            let mut file = File::create(&save.join(&name)).await?;
            file.write_all(&data.bin.as_slice()).await?;
            leeway.push_str(&name);
            leeway.push(',');
        }
        direction.write_all(&leeway.as_bytes()).await?;
        Ok(())
    }
    fn make_button(&self)->CreateActionRow{
        CreateActionRow::Buttons(
            vec![
            Components::normal_button("approve", 
                &format!("{}_save_a",&self.cmd.user.id.to_string()),
                ButtonStyle::Primary, "👍"),
            Components::normal_button("disaprove", 
                &format!("{}_save_d",&self.cmd.user.id.to_string()),
                ButtonStyle::Danger,"👎")
            ])
    }
}
struct SaveAcknowladge{
    uid:String,
    accept:bool
}
impl SaveAcknowladge{
    fn check(data:Vec<&str>)->Option<SaveAcknowladge>{
        if let Some(x) = data.first(){
            if let Some(y)=data.last(){
                let mut accept = false;
                if y==&"a"{
                    accept=true
                }
                return Some(SaveAcknowladge { uid: x.to_string(), accept });
            }
        }
        None
    }
    async fn get_files(&self)->Result<Vec<FileSave>,tokio::io::Error>{
        let save = Path::new(".").join("save");
        let user = format!("{}.txt",&self.uid);
        let dir = tokio::fs::read_to_string(&save.join(&user)).await?;
        let mut container = Vec::new();
        for path in dir.split(","){
            if path==""{
                continue;
            }
            let bytea = tokio::fs::read(&save.join(path)).await?;
            let mut name = path.split("_").last().unwrap().split(".").next().unwrap();
            if name == "hist"{
                name = "skin_hist"
            }
            container.push(FileSave{bin:bytea,name:name.to_owned()})
        }
        Ok(container)
    }
}


pub async fn run(ctx:&Context,cmd:&CommandInteraction,init:&Init){
    let mut reg = match Register::default(ctx, cmd, &init, "transfer save", false).await {
        Some(r)=>r,
        None=>{return ;}
    };
    if let Err(why)=cmd.defer(&ctx.http).await{
        reg.error.discord_error(why.to_string(), "defering save interaction").await
    };
    let data = SaveJudge::get_save(cmd).await;
    if data.files.len()==0{
        reg.error.change_error("no valid file detected".to_string(), "transfer save","please rename your save file properly and dont send any large file".to_string());
        reg.error.log_slash_defer(cmd, false).await;
        return ;
    }
    if let Err(why)=data.save_to_file().await{
        reg.error.change_error(why.to_string(), "saving file to folder", "windows issue".to_string());
        reg.error.log_slash_defer(cmd, false).await;
        return ;
    }
    match reg.pg.transfer_cd().await{
        Ok(x)=>{
            if x.0{
                if let Err(why)=cmd.edit_response(&ctx.http,EditInteractionResponse::new()
                    .embed(data.make_embed())).await{
                    reg.error.discord_error(why.to_string(), "transfer response").await;
                }
                let ch = ChannelId(NonZeroU64::new(init.log_channel.transfer_channel).unwrap());
                if let Err(why) = ch.send_message(&ctx.http,CreateMessage::new()
                    .content(format!("<@&{}>",init.server_role.judge_role))
                    .embed(data.make_embed()).components(vec![data.make_button()])).await{
                    reg.error.log_error_channel().await;
                    reg.error.change_error(why.to_string(), "send save to judge", "sorry you need to report this so you could reset your cooldown".to_string());
                }
            }else {
                reg.error.change_error("Youare Still On Cooldown".to_string(), "save transfer", format!("You need to wait till <t:{}:R> to be able to attemp transfer save again",x.1));
                reg.error.log_slash_defer(cmd, false).await;
            }
        }
        Err(why)=>{
            reg.error.pgcon_error_defer(why.to_string(), "get transfer cd", cmd).await;
        }
    };
    reg.pg.close().await;
}
pub async fn run_button(data:Vec<&str>,ctx:&Context,cmd:&ComponentInteraction,init:&Init){
    let mut error = ErrorLog::new(ctx, init, &cmd.user).await;
    if let Err(x)=cmd.create_response(&ctx.http, 
        Components::interaction_response("begin lengthy opration, please dont push button for some time", true)).await{
        error.discord_error(x.to_string(), "reply button").await;
    }
    let check = match SaveAcknowladge::check(data){
        Some(x)=>x,
        None=>{
            error.discord_error("something wrong happened".to_string(), "save button").await;
            return ;
        }
    };
    let user = UserId(NonZeroU64::new(check.uid.parse::<u64>().unwrap()).unwrap()).to_user(&ctx.http).await.unwrap();
    if check.accept{
        let files = match check.get_files().await{
            Ok(x)=>x,
            Err(why)=>{
                error.change_error(why.to_string(), "get files", "please report".to_string());
                error.log_error_channel().await;
                return ;
            }
        };
        let mut pg = match PgConn::create(init,check.uid).await{
            Ok(p)=>p,
            Err(why)=>{
                error.pgcon_error_ch(why.to_string(), "save approve").await;
                return ;
            }
        };
        match pg.get_user_data().await{
            Ok(dt)=>{
                for file in &files{
                    if let Err(why) = pg.transfer_file(file, dt.cid).await{
                        error.pgcon_error_ch(why.to_string(), "upload files").await;
                        return pg.close().await;
                    }
                }
            }
            Err(why)=>{
                error.pgcon_error_ch(why.to_string(), "getting user data").await;
                return pg.close().await;
            }
        };
        if let Err(why)=user.direct_message(&ctx.http, CreateMessage::new().content(&format!("your save file is already approved by {} you can login into the game now",cmd.user.name))).await{
            error.change_error(why.to_string(), "sending dm",format!("please mention {} that save file is successfully transfered, bot cant send dm to them, maybe beacause they disable dm",user.to_string()));
            error.log_error_channel().await;
        }
        let mut msg = cmd.message.clone();
        if let Err(why) =msg.edit(&ctx.http, EditMessage::new()
                .content(format!("Approved by {} At <t:{}:F>",cmd.user.name,SystemTime::now()
                    .duration_since(SystemTime::UNIX_EPOCH).unwrap().as_secs())).components(vec![])).await{
            error.change_error(why.to_string(), "erchiving message", "please delete the message manually, the process is already done successfully".to_string());
            error.log_error_channel().await;
        }
        pg.close().await;
    }else {
        if let Err(why)=user.direct_message(&ctx.http,CreateMessage::new().content(&format!("your save file has been rejected by {} you can login into the game now",cmd.user.name))).await{
            error.change_error(why.to_string(), "sending dm",format!("please mention {} that save file is rejected, bot cant send dm to them, maybe beacause they disable dm",user.to_string()));
            error.log_error_channel().await;
        };
        if let Err(why) = cmd.message.delete(&ctx.http).await{
            error.change_error(why.to_string(), "deleting message", "please delete the message manually, the process is already done successfully".to_string());
            error.log_error_channel().await;
        }
    }
}
