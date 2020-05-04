use renefetcher::telegram::Telegram;
use std::{thread, time};

fn main() {
    println!("Hello, world!");
    let telegram = Telegram::new();
    let mut updates = telegram.get_updates();

    loop {
        println!("Looping");
        match updates.run() {
            Ok(resp) => {
                // println!("{:#?}", resp);
                let wait_time = time::Duration::from_millis(1000);
                thread::sleep(wait_time);
            }
            _ => {}
        };
    }
}
