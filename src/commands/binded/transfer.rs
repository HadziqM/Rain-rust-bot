use serenity::{model::prelude::{interaction::application_command::{ApplicationCommandInteraction, CommandDataOptionValue}, component::ButtonStyle}, builder::{CreateEmbed, CreateActionRow}};
use tokio::io::AsyncWriteExt;
use tokio::fs::File;
use crate::Components;

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
