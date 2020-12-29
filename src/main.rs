use reqwest::blocking;
use reqwest::blocking::Response;
use reqwest::Error;
use serde::Serialize;
use std::convert::TryInto;

#[derive(Serialize)]
struct User {
    nickname: String,
    email: String,
}

fn main() -> Result<(), Error> {
    create_user();
    Ok(())
}

fn create_user() -> Result<(), Error> {
    let user: User = User { nickname: String::from("9motom6"), email: String::from("9motom6@gmail.com") };
    let client = reqwest::blocking::Client::new();

    let res: String = client.post("https://piskvorky.jobs.cz/api/v1/user")
        .body(serde_json::to_string(&user).unwrap())
        .send()?
        .text()?;
    println!("{}", res);

    Ok(())
}