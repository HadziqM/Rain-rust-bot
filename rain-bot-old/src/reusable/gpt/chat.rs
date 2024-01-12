use super::{Gpt, MyErr};
use crate::{Components, Init, ModalBundle, Mybundle, Mytrait};
use serde::{Deserialize, Serialize};
use serenity::all::{Message as SerenityMessage, *};

#[derive(Serialize, Deserialize, Clone)]
pub struct Choiches {
    pub message: Message,
}
#[derive(Serialize, Deserialize, Clone)]
pub struct Message {
    pub content: String,
    role: String,
}
#[derive(Serialize, Deserialize, Clone)]
pub struct Completition {
    pub choices: Vec<Choiches>,
}
#[derive(Serialize, Deserialize, Clone)]
pub struct CompletitionData {
    model: String,
    pub messages: Vec<Message>,
}
#[derive(Clone, Serialize, Deserialize)]
pub struct CompModel {
    pub data: CompletitionData,
    pub comp: Completition,
}
#[derive(Serialize, Deserialize)]
pub struct Preserve {
    pub id: String,
    pub comp: CompModel,
}
impl CompletitionData {
    fn new(messages: Vec<Message>) -> Self {
        CompletitionData {
            model: "gpt-3.5-turbo".to_owned(),
            messages,
        }
    }
}
impl Message {
    fn user(content: &str) -> Self {
        Message {
            content: content.to_owned(),
            role: "user".to_owned(),
        }
    }
    fn assistant(content: &str) -> Self {
        Message {
            content: content.to_owned(),
            role: "assistant".to_owned(),
        }
    }
    fn system(content: &str) -> Self {
        Message {
            content: content.to_owned(),
            role: "system".to_owned(),
        }
    }
}

impl CompModel {
    pub async fn delete(
        id: &str,
        mut msg: SerenityMessage,
        ctx: &Context,
        confirm: bool,
    ) -> Result<(), MyErr> {
        if confirm {
            let mut chat = crate::CHAT.lock().await;
            chat.remove(id);
            msg.edit(&ctx.http, EditMessage::new().components(vec![]))
                .await?;
        }
        Ok(())
    }
    pub async fn cached(&self, id: &str) {
        let mut chat = crate::CHAT.lock().await;
        match chat.get_mut(id) {
            Some(x) => {
                *x = self.clone();
            }
            None => {
                chat.insert(id.to_owned(), self.clone());
            }
        }
    }
    pub async fn retieve(id: &str) -> Result<Self, MyErr> {
        let chat = crate::CHAT.lock().await;
        match chat.get(id) {
            Some(x) => {
                return Ok(x.to_owned());
            }
            None => {
                return Err(MyErr::Custom("the chat hystory is erased".to_owned()));
            }
        }
    }
    pub async fn timeout(
        id: &str,
        ctx: &Context,
        msg: SerenityMessage,
        init: &Init,
    ) -> Result<(), MyErr> {
        let stream = ComponentInteractionCollector::new(ctx)
            .custom_ids(vec![format!("chat-{}", &id)])
            .timeout(std::time::Duration::from_secs(init.chat_gpt.timeout as u64));
        let mut conf = true;
        while let Some(_) = stream.next().await {
            conf = false;
            break;
        }
        Self::delete(id, msg, ctx, conf).await
    }
    pub fn button(id: &str) -> Vec<CreateActionRow> {
        let button =
            Components::normal_button("Reply", &format!("chat-{}", id), ButtonStyle::Primary, "🤨");
        vec![CreateActionRow::Buttons(vec![button])]
    }
    pub async fn send<T: Mybundle>(&self, bnd: &T, id: &str) -> Result<(), MyErr> {
        let embed = self.embed(bnd);
        if embed.len() > 10 {
            Components::edit_adv(
                bnd,
                EditInteractionResponse::new()
                    .components(Self::button(id))
                    .embeds(embed[..9].to_vec()),
            )
            .await?;
        } else {
            Components::edit_adv(
                bnd,
                EditInteractionResponse::new()
                    .components(Self::button(id))
                    .embeds(embed),
            )
            .await?;
        }
        self.cached(id).await;
        if bnd.init().chat_gpt.timeout != 0 {
            let new_id = id.to_owned();
            let new_ctx = bnd.ctx().clone();
            let msg = bnd.cmd().get_msg(bnd.ctx()).await?;
            let init = bnd.init().clone();
            tokio::spawn(async move {
                let _ = Self::timeout(&new_id, &new_ctx, msg, &init).await;
            });
        }
        Ok(())
    }
    pub async fn modal_send(&self, bnd: &ModalBundle<'_>, id: &str) -> Result<(), MyErr> {
        let embed = self.embed(bnd);
        let msg;
        if embed.len() > 10 {
            msg = bnd
                .cmd
                .create_followup(
                    &bnd.ctx.http,
                    CreateInteractionResponseFollowup::new()
                        .add_embeds(embed[..9].to_vec())
                        .components(CompModel::button(&id)),
                )
                .await?;
        } else {
            msg = bnd
                .cmd
                .create_followup(
                    &bnd.ctx.http,
                    CreateInteractionResponseFollowup::new()
                        .add_embeds(embed)
                        .components(CompModel::button(&id)),
                )
                .await?;
        }
        self.cached(id).await;
        if bnd.init.chat_gpt.timeout != 0 {
            let new_id = id.to_owned();
            let new_ctx = bnd.ctx().clone();
            let init = bnd.init().clone();
            tokio::spawn(async move {
                let _ = Self::timeout(&new_id, &new_ctx, msg, &init).await;
            });
        }
        Ok(())
    }
    pub fn embed<T: Mybundle>(&self, bnd: &T) -> Vec<CreateEmbed> {
        let user = bnd.user();
        let rply = &self.comp.choices[0].message.content;
        let mut x = vec![rply.to_owned()];
        if rply.len() > 1000 {
            x = pretty_format(rply);
        }
        let mut y = vec![CreateEmbed::new()
            .title("Chat Gpt Prompt")
            .color(Color::GOLD)
            .author(CreateEmbedAuthor::new(&user.name).icon_url(user.face()))
            .field(
                "Question",
                &self.data.messages.last().unwrap().content,
                false,
            )
            .field("Answer", &x[0], false)];
        if x.len() > 1 {
            for z in x[1..].to_vec() {
                y.push(CreateEmbed::new().color(Color::GOLD).description(z))
            }
        }
        y
    }
}
fn pretty_format<'a>(data: &str) -> Vec<String> {
    let x = data.split("```").collect::<Vec<_>>();
    if x.len() == 1 {
        return x[0].split("\n\n").map(|x| x.to_owned()).collect();
    }
    let mut y = vec![];
    let mut splited = true;
    for i in x {
        if splited {
            y.append(&mut i.split("\n\n").map(|x| x.to_owned()).collect::<Vec<_>>());
        } else {
            y.push(format!("```{i}```"))
        }
        splited = !splited;
    }
    y.iter()
        .filter(|&x| x != "")
        .map(|y| y.to_owned())
        .collect::<Vec<_>>()
}
impl Gpt {
    async fn comp(&self, data: &CompletitionData) -> Result<Completition, MyErr> {
        let url = "https://api.openai.com/v1/chat/completions";
        let client = reqwest::Client::new();
        let resp = client
            .post(url)
            .headers(self.head.to_owned())
            .json(&data)
            .send()
            .await?
            .text()
            .await?;
        let comp = match serde_json::from_str::<Completition>(&resp) {
            Ok(x) => x,
            Err(_) => {
                return Err(MyErr::Custom(format!(
                    "getting invalid response with the parsed data:\n\n{}",
                    &resp
                )));
            }
        };
        Ok(comp)
    }
    pub async fn completition(&self, ask: &str) -> Result<CompModel, MyErr> {
        let data = CompletitionData::new(vec![
            Message::system("Youare Self important assistant, if you ever dont know or understand the question asked by user,
                never reveal yourself as AI or use any `as an AI model` word, just make up the answer or feign ignorance,
                also please reply with sarcasm if you asked by narcistic person"),
            Message::user(ask)]);
        let comp = self.comp(&data).await?;
        Ok(CompModel { data, comp })
    }
    pub async fn reply_comp(&self, data: &CompModel, ask: &str) -> Result<CompModel, MyErr> {
        let mut n = data.clone();
        n.data
            .messages
            .push(Message::assistant(&data.comp.choices[0].message.content));
        n.data.messages.push(Message::user(ask));
        let comp = self.comp(&n.data).await?;
        Ok(CompModel { data: n.data, comp })
    }
}
// #[cfg(test)]
// mod testing{
//     use super::*;
//
//     #[tokio::test]
//     #[ignore = "tested"]
//     async fn chat_gpt() {
//         let gpt = Gpt::new().unwrap();
//         let ask = gpt.completition("what is rust programing language?").await.unwrap();
//         println!("rust is : {}\n",ask.comp.choices[0].message.content);
//         let reply = gpt.reply_comp(&ask, "hello world example?").await.unwrap();
//         println!("example : {}",reply.comp.choices[0].message.content)
//     }
// }
