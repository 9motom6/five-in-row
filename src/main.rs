use reqwest::blocking;
use reqwest::Error;

fn main() -> Result<(), Error> {
    let body = reqwest::blocking::get("https://www.rust-lang.org")?.text()?;


    println!("body = {:?}", body);
    Ok(())
}
