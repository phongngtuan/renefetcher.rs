use serde::Deserialize;

use reqwest::blocking::Client;

#[derive(Deserialize, Debug)]
pub struct Update {
    pub update_id: i32,
    pub message: Message,
}

#[derive(Deserialize, Debug)]
pub struct Response<A> {
    ok: bool,
    result: Vec<A>,
}

#[derive(Deserialize, Debug)]
pub struct Message {
    pub message_id: i32,
    pub chat: Chat,
    pub text: String,
    pub from: User,
}

#[derive(Deserialize, Debug)]
pub struct Chat {
    pub id: i32,
    pub username: String,
}

#[derive(Deserialize, Debug)]
pub struct User {
    pub id: i32,
    pub is_bot: bool,
    pub username: String,
}

pub struct Telegram {
    token: String,
}

pub struct GetUpdates {
    token: String,
    last_offset: i32,
}

impl GetUpdates {
    pub fn run(&mut self) -> Result<Vec<Update>, Box<dyn std::error::Error>> {
        let base_url = "https://api.telegram.org/bot";
        let get_updates_url = format!("{}{}/getUpdates", base_url, self.token);
        let client = Client::new();

        let params = [("offset", self.last_offset.to_string())];
        let resp = client
            .get(&get_updates_url)
            .form(&params)
            .send()?
            .json::<Response<Update>>()?;
        println!("Updates: {:#?}", resp);
        let last_update_id: i32 = resp.result.iter().map(|u| u.update_id).max().unwrap_or(-1);
        self.last_offset = last_update_id + 1;

        Ok(resp.result)
    }
}

pub struct SendMessage {
    token: String,
}

impl SendMessage {
    pub fn run(&self, chat_id: i32, text: &str) -> Result<(), Box<dyn std::error::Error>> {
        let base_url = "https://api.telegram.org/bot";
        let url = format!("{}{}/sendMessage", base_url, self.token);

        let client = Client::new();
        let chat_id_param = chat_id.to_string();

        let params: Vec<(&str, &str)> = vec![("chat_id", &chat_id_param), ("text", text)];
        let resp = client.post(&url).form(&params).send()?;
        println!("Updates: {:#?}", resp);

        Ok(())
    }
}

impl Telegram {
    pub fn new(token: &str) -> Self {
        Telegram {
            token: String::from(token),
        }
    }

    pub fn get_updates(&self) -> GetUpdates {
        GetUpdates {
            token: self.token.clone(), //TODO: not clone this
            last_offset: 0,
        }
    }

    pub fn send_message(&self) -> SendMessage {
        SendMessage {
            token: self.token.clone(),
        }
    }
}
