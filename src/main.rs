mod config;
mod ticket;

use config::Config;
use poise::serenity_prelude as serenity;
use std::sync::Arc;

pub struct Data {
    pub config: Config,
}

type Error = Box<dyn std::error::Error + Send + Sync>;
type Context<'a> = poise::Context<'a, Data, Error>;

///отправка главного меню (команда)
#[poise::command(slash_command, required_permissions = "ADMINISTRATOR")]
async fn send_ticket_panel(ctx: Context<'_>) -> Result<(), Error> {
    let categories = &ctx.data().config.categories;

    let options: Vec<serenity::CreateSelectMenuOption> = categories
        .iter()
        .map(|(key, cat)| {
            serenity::CreateSelectMenuOption::new(&cat.label, key).description(&cat.description)
        })
        .collect();

    let select_menu = serenity::CreateSelectMenu::new(
        "ticket_category_select",
        serenity::CreateSelectMenuKind::String { options },
    )
    .placeholder("Какая категория вас интересует?");

    let components = vec![serenity::CreateActionRow::SelectMenu(select_menu)];

    let embed = serenity::CreateEmbed::new()
        .title("Поддержка пользователей")
        .description("Выберите категорию вашего вопроса из списка ниже, чтобы открыть тикет.")
        .color(0x5865F2);

    ctx.send(
        poise::CreateReply::default()
            .embed(embed)
            .components(components),
    )
    .await?;

    Ok(())
}

#[tokio::main]
async fn main() {
    let token = std::env::var("DISCORD_TOKEN").expect("Не задан DISCORD_TOKEN");
    let config = Config::load().expect("Ошибка чтения config.json");

    let options = poise::FrameworkOptions {
        commands: vec![send_ticket_panel()],
        event_handler: |ctx, event, _framework, data| {
            Box::pin(async move {
                if let serenity::FullEvent::InteractionCreate { interaction } = event {
                    handle_interaction(ctx, interaction, data).await;
                }
                Ok(())
            })
        },
        ..Default::default()
    };

    let framework = poise::Framework::builder()
        .options(options)
        .setup(|ctx, _ready, framework| {
            Box::pin(async move {
                poise::builtins::register_globally(ctx, &framework.options().commands).await?;
                Ok(Data { config })
            })
        })
        .build();

    let client = serenity::ClientBuilder::new(token, serenity::GatewayIntents::non_privileged())
        .framework(framework)
        .await;

    client.unwrap().start().await.unwrap();
}

async fn handle_interaction(
    _ctx: &serenity::Context,
    _interaction: &serenity::Interaction,
    _data: &Data,
) {
    //обработка отображения SelectMenu +отправка формы
}