extern crate reqwest;

fn main() {
    let body = reqwest::get("https://www.rust-lang.org")?.text()?;
    println!("{:?}", body);
}
