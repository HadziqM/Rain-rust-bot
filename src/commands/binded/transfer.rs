use serenity::all::*;
use tokio::io::AsyncWriteExt;
use tokio::fs::File;
use crate::{Components,Reg,SlashBundle,ComponentBundle,MyErr,Mytrait,Mybundle};
use crate::reusable::utils::Color;
use std::num::NonZeroU64;
use std::path::Path;
use std::time::{SystemTime, Duration};

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
    async fn save_to_file(&self)->Result<(),MyErr>{
        let save = Path::new(".").join("save");
        let user = format!("{}.txt",&self.cmd.user.id.to_string());
        let mut direction = File::create(&save.join(&user)).await?;
        let mut leeway = String::new();
        for data in &self.files{
            let name = format!("{}_{}",&self.cmd.user.id.to_string()
                ,data.name.to_owned());
            tokio::fs::write(save.join(&name), &data.bin).await?;
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
                &format!("{}*save*a",&self.cmd.user.id.to_string()),
                ButtonStyle::Primary, "👍"),
            Components::normal_button("disaprove", 
                &format!("{}*save*d",&self.cmd.user.id.to_string()),
                ButtonStyle::Danger,"👎")
            ])
    }
}
struct SaveAcknowladge{
    uid:String,
    accept:bool
}
impl SaveAcknowladge{
    fn check(data:Vec<&str>)->Result<SaveAcknowladge,MyErr>{
        let err = MyErr::Custom("cant get the data fom button, please report this".to_owned());
        if let Some(x) = data.first(){
            if let Some(y)=data.last(){
                let mut accept = false;
                if y==&"a"{
                    accept=true
                }
                return Ok(SaveAcknowladge { uid: x.to_string(), accept });
            }
        }
        Err(err)
    }
    async fn get_files(&self)->Result<Vec<FileSave>,MyErr>{
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

#[hertz::hertz_slash_reg(300,true)]
async fn slash(bnd:&SlashBundle<'_>,_reg:&Reg<'_>)->Result<(),MyErr>{
    let data = SaveJudge::get_save(bnd.cmd).await;
    if data.files.len()==0{
        return Err(MyErr::Custom("no matched file detected, please rename your save file properly and dont send any large file".to_string()));
    }
    data.save_to_file().await?;
    let content = EditInteractionResponse::new().embed(data.make_embed());
    let ch = ChannelId(NonZeroU64::new(bnd.init.log_channel.transfer_channel).unwrap());
    Components::edit_adv(bnd, content).await?;
    let msg = ch.send_message(&bnd.ctx.http,CreateMessage::new()
        .content(format!("<@&{}>",bnd.init.server_role.judge_role))
        .embed(data.make_embed()).components(vec![data.make_button()])).await?;
    tokio::time::sleep(Duration::from_secs(10*60)).await;
    auto_accept(msg, SaveAcknowladge { uid:bnd.cmd.user.id.to_string() , accept: true }, bnd, _reg).await?;
    Ok(())
}
async fn auto_accept(msg:Message,ack:SaveAcknowladge,bnd:&SlashBundle<'_>,reg:&Reg<'_>)->Result<(),MyErr>{
    let mut msg = msg.channel_id.message(&bnd.ctx.http, msg.id).await?;
    if msg.components.len() != 0{
        let files = ack.get_files().await?;
        let user = bnd.user();
        for file in &files{
            reg.pg.transfer_file(file, reg.cid).await?;
        }
        user.direct_message(&bnd.ctx.http, CreateMessage::new().content("your save file is already approved automatically by bot,\nyou can login into the game now")).await?;
        msg.edit(&bnd.ctx.http, EditMessage::new()
            .content(format!("Approved automatically by bot At <t:{}:F>",SystemTime::now()
            .duration_since(SystemTime::UNIX_EPOCH).unwrap().as_secs())).components(vec![])).await?;
    }
    Ok(())
}

#[hertz::hertz_button_normal(0,false)]
async fn button(bnd:&ComponentBundle<'_>)->Result<(),MyErr>{
    Components::response(bnd, "begin lengthy opration, please dont push button for some time", true).await?;
    let data:Vec<_> = bnd.cmd.data.custom_id.split("*").collect();
    let check = SaveAcknowladge::check(data)?;
    let user = UserId(NonZeroU64::new(check.uid.parse::<u64>().unwrap()).unwrap()).to_user(&bnd.ctx.http).await.unwrap();
    if check.accept{
        let files = check.get_files().await?;
        let mut reg = Reg::check(bnd, &user).await?;
        for file in &files{
            reg.pg.transfer_file(file, reg.cid).await?;
        }
        user.direct_message(&bnd.ctx.http, CreateMessage::new().content(&format!("your save file is already approved by {} you can login into the game now",bnd.cmd.user.name))).await?;
        let mut msg = bnd.cmd.message.clone();
        msg.edit(&bnd.ctx.http, EditMessage::new()
            .content(format!("Approved by {} At <t:{}:F>",bnd.cmd.user.name,SystemTime::now()
            .duration_since(SystemTime::UNIX_EPOCH).unwrap().as_secs())).components(vec![])).await?;
        reg.pg.close().await;
    }else {
        user.direct_message(&bnd.ctx.http,CreateMessage::new().content(&format!("your save file has been rejected by {} you can login into the game now",bnd.cmd.user.name))).await?;
        bnd.cmd.message.delete(&bnd.ctx.http).await?;
    }
    Ok(())
}
