use reqwest;

fn dropbox() {
    let body = reqwest::get("https://www.rust-lang.org")?
    .text()?;

    println!("body = {:?}", body);
}

fn main() {
    dropbox();
}