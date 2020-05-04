use serde::{Deserialize};
use std::fs;
use std::path::PathBuf;
use dirs;

#[derive(Deserialize, Debug)]
pub struct Update {
    update_id: i32,
    message: Message,
}

#[derive(Deserialize, Debug)]
pub struct Response<A> {
    ok: bool,
    result: Vec<A>,
}

#[derive(Deserialize, Debug)]
pub struct Message {
    message_id: i32,
    chat: Chat,
    text: String,
    from: User,
}

#[derive(Deserialize, Debug)]
pub struct Chat {
    id: i32,
    username: String,
}

#[derive(Deserialize, Debug)]
pub struct User {
    id: i32,
    is_bot: bool,
    username: String,
}

pub struct Telegram {
    token: String
}

impl Telegram {
    pub fn new() -> Self {
        let mut token_file = dirs::home_dir().unwrap_or(PathBuf::new());
        token_file.push(".telegram_token");
        println!("{:?}", token_file);
        let token = fs::read_to_string(token_file).unwrap();
        Telegram {
            token
        }
    }

    pub async fn get_update(&self) -> Result<Update, Box<dyn std::error::Error>> {
        let base_url = "https://api.telegram.org/bot";
        let get_updates_url = format!("{}{}/getUpdates", base_url, self.token);
        let mut resp = reqwest::get(&get_updates_url)
        .await?
        .json::<Response<Update>>()
        .await?;
        Ok(resp.result.remove(0))
    }

    pub async fn set_webhook(&self) -> Result<(), Box<dyn std::error::Error>> {
        let base_url = "https://api.telegram.org/bot";
        let get_updates_url = format!("{}{}/setWebhook", base_url, self.token);
        let resp = reqwest::get(&get_updates_url) .await?.text().await?;
        println!("{}", resp);
        Ok(())
    }
}