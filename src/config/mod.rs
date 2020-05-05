use serde::{Deserialize};

#[derive(Deserialize, Debug)]
pub struct Config {
    pub telegram: TelegramConfig,
    pub db: DbConfig,
}

#[derive(Deserialize, Debug)]
pub struct DbConfig {
    pub host: String,
    pub port: i32,
    pub username: String,
    pub password: String,
}

#[derive(Deserialize, Debug)]
pub struct TelegramConfig {
    pub token: String,
}

impl Config {
    pub fn from_str(s: &str) -> Result<Config, toml::de::Error> {
        toml::from_str(s)
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn bad_config() {
        assert!(Config::from_str("").is_err());
    }

    #[test]
    fn good_config() {
        let config= Config::from_str(r#"
        [db]
        host = "1.2.3.4"
        port = 56789
        username = "test_user"
        password = "test_password"

        [telegram]
        token = "test_token"
        "#);
        println!("{:?}", config);
        assert!(config.is_ok(), "config should be good");
        let config = config.unwrap();
        assert_eq!(config.telegram.token, "test_token")
    }
}