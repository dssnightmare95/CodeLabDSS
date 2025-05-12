use teloxide::{prelude::*, utils::command::BotCommands};
use teloxide::dispatching::dialogue::{Dialogue, InMemStorage};
use teloxide::types::BotCommand;
use teloxide::dispatching::UpdateHandler;
use teloxide::dispatching::dialogue;
use std::env;
use dotenv::dotenv;

type MyDialogue = Dialogue<State, InMemStorage<State>>;
pub type HandlerResult = Result<(), Box<dyn std::error::Error + Send + Sync>>;

#[derive(Clone, Default, Debug)]
enum State {
    #[default]
    Start,
    WaitingLocation,
    WaitingProvince {
        location: String,
    },
    WaitingCountry {
        location: String,
        province: String,
    }
}

#[derive(BotCommands, Clone)]
#[command(rename_rule = "lowercase", description = "Enable commands in the bot")]
pub enum Command {
    #[command(description = "Start bot 🚀")]
    Start,
    #[command(description = "Get weather forecast 🌤️")]
    GetWeather,
    #[command(description = "Cancel operation ❌")]
    Cancel,
    #[command(description = "Help menu 📜")]
    Help,
}

pub async fn run_bot() {
    dotenv().ok();

    pretty_env_logger::init();
    log::info!("Starting bot...");

    let token = env::var("TELEGRAM_BOT_TOKEN").expect("TELOXIDE_TOKEN no está definida");
    let bot = Bot::new(token);

    let commands = Command::bot_commands()
        .iter()
        .map(|cmd| BotCommand::new(cmd.command.clone(), cmd.description.clone()))
        .collect::<Vec<_>>();

    bot.set_my_commands(commands).send().await.expect("The menu could not be set!");

    Dispatcher::builder(bot, schema())
        .dependencies(dptree::deps![InMemStorage::<State>::new()])
        .enable_ctrlc_handler()
        .build()
        .dispatch()
        .await;
}

fn schema() -> UpdateHandler<Box<dyn std::error::Error + Send + Sync + 'static>> {
    use dptree::case;
    let command_handler = teloxide::filter_command::<Command, _>()
        .branch(
            case![State::Start]
                .branch(case![Command::Start].endpoint(handle_start_command))
                .branch(case![Command::GetWeather].endpoint(handle_get_weather_command))
                .branch(case![Command::Help].endpoint(handle_help_command)),
        )
        .branch(case![Command::Cancel].endpoint(cancel));

    let message_handler = Update::filter_message()
        .branch(command_handler)
        .branch(case![State::WaitingLocation].endpoint(dialogue_handler))
        .branch(case![State::WaitingProvince { location }].endpoint(dialogue_handler))
        .branch(case![State::WaitingCountry { location, province }].endpoint(dialogue_handler));

    dialogue::enter::<Update, InMemStorage<State>, State, _>()
        .branch(message_handler)
}

async fn cancel(bot: Bot, dialogue: MyDialogue, msg: Message) -> HandlerResult {
    bot.send_message(msg.chat.id, "Cancelling the dialogue.").await?;
    dialogue.exit().await?;
    Ok(())
}

async fn handle_start_command(bot: Bot, msg: Message) -> HandlerResult {
    bot.send_message(msg.chat.id, "Welcome to the Weather Bot!").await?;
    Ok(())
}

async fn handle_help_command(bot: Bot, msg: Message) -> HandlerResult {
    bot.send_message(msg.chat.id, Command::descriptions().to_string()).await?;
    Ok(())
}

async fn handle_get_weather_command(bot: Bot, msg: Message, dialogue: MyDialogue) -> HandlerResult {
    bot.send_message(msg.chat.id, "What is your location?").await?;
    dialogue.update(State::WaitingLocation).await?;
    Ok(())
}

async fn dialogue_handler(bot: Bot, msg: Message, dialogue: MyDialogue, state: State) -> HandlerResult {
    match state {
        State::Start => {
            bot.send_message(msg.chat.id, "Welcome to the Weather Bot!").await?;
            dialogue.update(State::Start).await?;
        }
        State::WaitingLocation => {
            if let Some(text) = msg.text() {
                bot.send_message(msg.chat.id, format!("In which province or state is {} located?", text)).await?;
                dialogue.update(State::WaitingProvince { location: text.into() }).await?;
            } else {
                bot.send_message(msg.chat.id, "Please send me a valid name").await?;
            }
        }
        State::WaitingProvince { location } => {
            if let Some(text) = msg.text() {
                bot.send_message(msg.chat.id, format!("In which country is {} ({})?", location, text)).await?;
                dialogue.update(State::WaitingCountry { location, province: text.into() }).await?;
            } else {
                bot.send_message(msg.chat.id, "Please send me a valid name").await?;
                dialogue.update(State::WaitingProvince { location }).await?;
            }
        }
        State::WaitingCountry { location, province } => {
            if let Some(text) = msg.text() {
                bot.send_message(msg.chat.id, format!("Obtaining the climate for {}, {} en {}...", location, province, text)).await?;
                dialogue.exit().await?;
                // Aquí iría la lógica para obtener el clima usando la ubicación, provincia y país.
                // Por ahora, solo enviamos un mensaje de confirmación.
                bot.send_message(msg.chat.id, format!("¡Done! The climate for {}, {} ({}) is... (simulation)", location, province, text)).await?;
                dialogue.exit().await?;
            } else {
                bot.send_message(msg.chat.id, "Please send me a valid name" ).await?;
                dialogue.update(State::WaitingCountry { location, province }).await?;
            }
        }
    }
    Ok(())
}