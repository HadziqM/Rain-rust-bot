use crate::reusable::gpt::{chat::CompModel, Gpt};
use crate::{ComponentBundle, Components, ModalBundle, MyErr, Mybundle, Mytrait, SlashBundle};
use serenity::all::*;

#[hertz::hertz_slash_normal(0, true)]
async fn slash(bnd: &SlashBundle<'_>) -> Result<(), MyErr> {
    let mut name = "";
    for data in &bnd.cmd.data.options {
        if let CommandDataOptionValue::SubCommand(_) = &data.value {
            name = &data.name;
        }
    }
    match name {
        "chat" => chat(bnd).await?,
        "image" => image(bnd).await?,
        _ => return Err(MyErr::Custom("we dont have that command".to_owned())),
    }
    Ok(())
}

#[hertz::hertz_button_normal(0, false)]
async fn button(bnd: &ComponentBundle<'_>) -> Result<(), MyErr> {
    let id = get_id(bnd);
    Components::response_adv(bnd, modal_response(&id)).await?;
    let msg = bnd.cmd.message.await_modal_interaction(bnd.ctx);
    while let Some(x) = msg.next().await {
        let z = ModalBundle {
            cmd: &x,
            ctx: bnd.ctx(),
            image: bnd.image,
            init: bnd.init,
            pedia: bnd.pedia,
        };
        modal(&z).await?;
        break;
    }
    bnd.cmd
        .message
        .clone()
        .edit(&bnd.ctx.http, EditMessage::new().components(vec![]))
        .await?;
    Ok(())
}
async fn modal(bnd: &ModalBundle<'_>) -> Result<(), MyErr> {
    Components::response(bnd, "wait for few second or minutes", true).await?;
    let mut ask = String::new();
    for comp in &bnd.cmd.data.components {
        let arow = comp.components.first().unwrap();
        if let ActionRowComponent::InputText(input) = arow {
            ask = input.value.to_owned();
        }
    }
    if &ask != "" {
        let id = get_id(bnd);
        let gpt_s = Gpt::new(bnd.init)?;
        let data = CompModel::retieve(&id).await?;
        let resp = gpt_s.reply_comp(&data, &ask).await?;
        resp.modal_send(bnd, &id).await?;
        Ok(())
    } else {
        Err(MyErr::Custom("no question asked".to_owned()))
    }
}

async fn chat(bnd: &SlashBundle<'_>) -> Result<(), MyErr> {
    let gpt_struct = Gpt::new(bnd.init)?;
    let mut name = "";
    let id = bnd.cmd.id.to_string();
    for i in Components::sub_options(bnd)? {
        if let CommandDataOptionValue::String(x) = &i.value {
            name = x;
        }
    }
    if name == "" {
        return Err(MyErr::Custom("idk".to_owned()));
    } else {
        let resp = gpt_struct.completition(name).await?;
        resp.send(bnd, &id).await?;
    }
    Ok(())
}

async fn image(bnd: &SlashBundle<'_>) -> Result<(), MyErr> {
    let mut prompt = "".to_owned();
    let mut size = "".to_owned();
    let mut n = 1;
    for x in Components::sub_options(bnd)? {
        match &x.value {
            CommandDataOptionValue::String(y) => {
                if x.name == "size" {
                    size = y.to_owned()
                } else {
                    prompt = y.to_owned()
                }
            }
            CommandDataOptionValue::Integer(y) => {
                n = *y as u8;
            }
            _ => {
                continue;
            }
        }
    }
    let gpt = Gpt::new(bnd.init)?;
    let img = gpt.get_image(prompt, n, size).await?;
    img.send(bnd).await?;
    Ok(())
}

fn get_id<T: Mybundle>(bnd: &T) -> String {
    let name = bnd.name();
    name.split("-").nth(1).unwrap().to_string()
}
fn modal_register_row(name: &str) -> CreateActionRow {
    let placeholder = "Your Reply".to_owned();
    CreateActionRow::InputText(
        CreateInputText::new(InputTextStyle::Paragraph, name, name)
            .required(true)
            .placeholder(&placeholder),
    )
}
fn modal_response(id: &str) -> CreateInteractionResponse {
    let name = format!("chat-{id}");
    CreateInteractionResponse::Modal(
        CreateModal::new(name, "Your Chat Response")
            .components(vec![modal_register_row("username")]),
    )
}
