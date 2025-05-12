use teloxide::{prelude::*, utils::command::BotCommands};
use teloxide::dispatching::dialogue::{Dialogue, InMemStorage};
use teloxide::filter_command;
use teloxide::types::BotCommand;
use std::env;
use dotenv::dotenv;


//type MyDialogue = Dialogue<State, InMemStorage<State>>;
pub type HandlerResult = Result<(), Box<dyn std::error::Error + Send + Sync>>;

/* #[derive(Clone, Default, Debug)]
enum State {
    #[default]
    WaitingLocation,
    WaitingProvince {
        location: String,
    },
    WaitingCountry {
        location: String,
        province: String,
    }
} */

#[derive(BotCommands, Clone)]
#[command(rename_rule = "lowercase", description = "Enable commands in the bot")]
pub enum Command {
    #[command(description = "Start bot 🚀")]
    Start,
    #[command(description = "Get weather forecast 🌤️")]
    GetWeather,
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

    bot.set_my_commands(commands).send().await.expect("No se pudo establecer el menú");

    let handler = dptree::entry()
        .branch(Update::filter_message().filter_command::<Command>().endpoint(command_handler));


    Dispatcher::builder(bot, handler)
        //.dependencies(dptree::deps![InMemStorage::<State>::new()])
        .enable_ctrlc_handler()
        .build()
        .dispatch()
        .await;
}


async fn command_handler(bot: Bot, msg: Message, cmd: Command) -> HandlerResult {
    match cmd {
        Command::Start => handle_start_command(bot, msg).await,
        Command::GetWeather => handle_get_weather_command(bot, msg).await,
        Command::Help => handle_help_command(bot, msg).await,
    }
}

async fn handle_start_command(bot: Bot, msg: Message) -> HandlerResult {
    bot.send_message(msg.chat.id, "Welcome to the Weather Bot!").await?;
    Ok(())
}

async fn handle_help_command(bot: Bot, msg: Message) -> HandlerResult {
    bot.send_message(msg.chat.id, Command::descriptions().to_string()).await?;
    Ok(())
}

async fn handle_get_weather_command(bot: Bot, msg: Message/*, _dialogue: MyDialogue*/) -> HandlerResult {
    bot.send_message(msg.chat.id, "What is your location?").await?;
    //dialogue.update(State::WaitingLocation).await?;
    Ok(())
}
