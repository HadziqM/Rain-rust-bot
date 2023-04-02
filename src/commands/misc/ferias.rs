use crate::{MyErr,SlashBundle,Mytrait,Mybundle,Components, material::ItemPedia};
use super::super::admin::market::SubCommand;
use serenity::all::*;

#[hertz::hertz_auto]
async fn auto(bnd:&SlashBundle<'_>)->Result<(),MyErr>{
    for i in &bnd.cmd.data.options{
        if let CommandDataOptionValue::Autocomplete { kind:_, value } = &i.value{
            let data = SubCommand::Item(value.to_owned());
            let auto = data.predict(bnd.pedia)?;
            bnd.cmd.create_response(&bnd.ctx.http, CreateInteractionResponse::Autocomplete(CreateAutocompleteResponse::new().set_choices(auto))).await?;
        }
    }
    Ok(())
}
#[hertz::hertz_slash_normal(0,false)]
async fn slash(bnd:&SlashBundle<'_>)->Result<(),MyErr>{
    for i in &bnd.cmd.data.options{
        if let CommandDataOptionValue::String(x) = &i.value{
            let data:Vec<_> = x.chars().collect();
            let reverse = [data[2],data[3],data[0],data[1]].iter().collect::<String>();
            let auto = format!("https://xl3lackout.github.io/MHFZ-Ferias-English-Project/sozai/sozai.htm?{}",&reverse);
            let y = CreateInteractionResponse::Message(CreateInteractionResponseMessage::new().components(vec![CreateActionRow::Buttons(vec![CreateButton::new_link(&auto).label(ItemPedia::search(7, x).unwrap())])]));
            Components::response_adv(bnd, y).await?
        }
    }
    Ok(())

}
