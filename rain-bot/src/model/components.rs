use serenity::{
    all::{ButtonStyle, ReactionType},
    builder::{CreateButton, CreateInteractionResponse, CreateInteractionResponseMessage},
};

pub struct Components;

impl Components {
    pub fn normal_button(
        name: &str,
        custom_id: &str,
        style: ButtonStyle,
        emoji: &str,
    ) -> CreateButton {
        let mut b = CreateButton::new(custom_id).label(name).style(style);
        if let Ok(emj) = emoji.parse::<ReactionType>() {
            b = b.emoji(emj);
        }
        b
    }
    pub fn interaction_response(content: &str, ephemeral: bool) -> CreateInteractionResponse {
        CreateInteractionResponse::Message(
            CreateInteractionResponseMessage::new()
                .content(content)
                .ephemeral(ephemeral),
        )
    }
}
