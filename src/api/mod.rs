use crate::telegram::Update;
pub enum Command {
    Subscribe { username: String, chat_id: i32 }, //TODO: maybe use reference
    Unsubscribe { chat_id: i32 },
}

pub fn process(update: &Update) -> Option<Command> {
    println!("> Processing {:?}", update);
    match update.message.text.as_str() {
        "/start" => Some(Command::Subscribe {username: update.message.chat.username.clone(), chat_id: update.message.chat.id}),
        "/stop"  => Some(Command::Unsubscribe {chat_id: update.message.chat.id}),
        _        => None
    }
}