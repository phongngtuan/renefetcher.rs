use crate::telegram::Update;
enum Command {
    Subscribe,
    Unsubscribe,
}

pub fn process(update: &Update) {
    println!("> Processing {:?}", update);
}