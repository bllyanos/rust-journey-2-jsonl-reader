use jsonl_reader::JsonlReader;
use serde::{Deserialize, Serialize};

mod jsonl_reader;

#[derive(Debug, Serialize, Deserialize)]
struct User {
    name: String,
}

fn main() {
    let path = String::from("./data.jsonl");
    let reader: JsonlReader<User> = JsonlReader::new(path);
    let mut users: Vec<Option<User>> = vec![];

    for i in reader.iter() {
        match i {
            Ok(result) => {
                println!("{:?}", &result);
                users.push(Some(result));
            }
            Err(error) => {
                eprintln!("{error}");
                users.push(None)
            }
        }
    }
    println!("{}", serde_json::to_string_pretty(&users).unwrap());
}
