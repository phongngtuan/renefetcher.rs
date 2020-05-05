use clap::*;
use std::fs;
use renefetcher::api::process;
use renefetcher::config::Config;
use renefetcher::telegram::Telegram;
use std::{thread, time};

fn main() {
    println!("Hello, world!");

    let matches = App::new("renefetcher").version("1.0").arg(
        Arg::with_name("config")
            .short("c")
            .long("config")
            .value_name("FILE")
            .help("Sets a custom config file")
            .takes_value(true),
    ).get_matches();
    let config_path = matches.value_of("config").unwrap();
    let config = Config::from_str(&fs::read_to_string(config_path).unwrap()).unwrap();
    println!("{:?}", config);

    let telegram = Telegram::new(&config.telegram.token);
    let mut updates = telegram.get_updates();

    // loop {
    //     println!("Looping");
    //     match updates.run() {
    //         Ok(resp) => {
    //             // println!("{:#?}", resp);
    //             for update in resp {
    //                 process(&update);
    //             }
    //             let wait_time = time::Duration::from_millis(1000);
    //             thread::sleep(wait_time);
    //         }
    //         _ => {}
    //     };
    // }
}
