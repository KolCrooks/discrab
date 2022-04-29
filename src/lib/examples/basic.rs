mod cmds;
mod events;

use discrab::Bot;

use cmds::{EchoCmd, PingSlashCmd, TestCmd};
use events::MsgEvent;

use std::env;

macro_rules! register_all {
    ($bot:expr, $( $cmd:expr ),+) => {
        $bot = $bot
            $(.register(std::sync::Arc::new($cmd)).await)+;
    };
}

#[tokio::main]
async fn main() {
    dotenv::from_filename(".local.env").ok();
    let token = env::var("TOKEN").unwrap();

    let mut bot = Bot::new(token);
    bot.settings().set_debug(true);
    let test = TestCmd::new();
    register_all!(bot, test, EchoCmd, PingSlashCmd {a: 0}, MsgEvent);
    bot.listen().await;
}
