use serde::Deserialize;
use reqwest::Error;
use reqwest::header::USER_AGENT;

#[derive(Deserialize, Debug)]
struct User{
    login: String,
    id: u32
}

#[tokio::main]
async fn main()-> Result<(), Error>{
    let reqest_url = format!("https://api.github.com/repos/{owner}/{repo}/stargazers",
    owner = "rust-lang-nursery",
    repo = "rust-cookbook");
    println!("{}", reqest_url);

    let client = reqwest::Client::new();
    let res = client
    .get(&reqest_url)
    .header(USER_AGENT, "rust web api call demo")
    .send()
    .await?;

    let users: Vec<User> = res.json().await?;
    println!("users {:?} ", users);
    
    Ok(())
}