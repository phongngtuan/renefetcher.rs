use serde::{Deserialize};
use std::fs;
use std::path::PathBuf;
use reqwest::blocking::Client;
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

pub struct GetUpdates{
    token: String,
    last_offset: i32,
}

impl GetUpdates {
    pub fn run(&mut self) -> Result<Vec<Update>, Box<dyn std::error::Error>> {
        let base_url = "https://api.telegram.org/bot";
        let get_updates_url = format!("{}{}/getUpdates", base_url, self.token);
        let client = Client::new();

        let params = [("offset", self.last_offset.to_string())];
        let resp = client.get(&get_updates_url)
        .form(&params)
        .send()?
        .json::<Response<Update>>()?;
        println!("Updates: {:#?}", resp);
        let last_update_id: i32 = resp.result.iter().map(|u| u.update_id).max().unwrap_or(-1);
        self.last_offset = last_update_id + 1;

        Ok(resp.result)
    }
}

impl Telegram {
    pub fn new(token: &str) -> Self {
        Telegram {
            token: String::from(token)
        }
    }

    pub fn get_updates(&self) -> GetUpdates {
        GetUpdates {
            token: self.token.clone(),
            last_offset: 0,
        }
    }
}