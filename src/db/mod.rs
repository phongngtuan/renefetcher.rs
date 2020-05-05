use postgres::{Client, NoTls};

pub struct Db {
    client: Client,
}

impl Db {
    pub fn create_tables(&mut self) {
        let query = "
        CREATE TABLE IF NOT EXISTS subscriber (
            id       INT PRIMARY KEY,
            username TEXT NOT NULL
        )
        ";
        self.execute_and_print(query);
    }

    pub fn upsert(&mut self, username: &str, chat_id: i32) {
        let query = format!(
            "
        INSERT INTO subscriber (id, username)
        VALUES ({}, '{}')
        ON CONFLICT (id) DO NOTHING;
        ",
            chat_id, username
        );
        self.execute_and_print(&query)
    }

    pub fn delete(&mut self, chat_id: i32) {
        let query = format!(
            "
            DELETE FROM subscriber WHERE id = {}
        ",
            chat_id
        );
        self.execute_and_print(&query)
    }

    fn execute_and_print(&mut self, query: &str) {
        let res = match self.client.batch_execute(&query) {
            Ok(_) => "Ok",
            _ => "Err",
        };
        println!("{} - {}", res, query);
    }
}

impl Db {
    pub fn new(host: &str, port: i32, username: &str, password: &str) -> Db {
        let client = Client::connect(
            &format!(
                "host={} port={} user={} password={}",
                host, port, username, password
            ),
            NoTls,
        )
        .unwrap();
        Db { client }
    }
}
