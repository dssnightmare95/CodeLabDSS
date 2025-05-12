mod bot_handler;

#[tokio::main]
async fn main() {
    bot_handler::bot_main::run_bot().await;
}