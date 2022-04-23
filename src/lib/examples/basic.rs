mod cmds;
mod events;

use discrab::Bot;

use cmds::{EchoCmd, PingSlashCmd};
use events::MsgEvent;

use std::env;


#[tokio::main]
async fn main() {
    dotenv::from_filename(".local.env").ok();
    let token = env::var("TOKEN").unwrap();

    let mut bot = Bot::new(token);
    bot.settings().set_debug(true);
    bot.register_all(vec![&PingSlashCmd {a: 0}, &EchoCmd, &MsgEvent]).listen().await;
}
