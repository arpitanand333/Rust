use error_chain::error_chain;

error_chain!{
    foreign_links{
        Io(std::io::Error);
        HttpRequest(reqwest::Error);
    }
}

#[tokio::main]

async fn main() -> Result<()>{
    let mut res = reqwest::get("https://httpbin.org/get").await?;
    println!("status{}", res.status());
    println!("body{}", res.text().await?);
    println!("headers{:#?}", res.headers());
    Ok(())
}
