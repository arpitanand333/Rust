use serde::Deserialize;
//use reqwest::Error ;
use reqwest::header::USER_AGENT;
use sqlx::postgres::PgPoolOptions;
use sqlx::{FromRow, Row};

#[derive(Deserialize, Debug, FromRow)]
struct User {
    login: String,
    id: i32,
}

#[tokio::main]
async fn main() -> std::io::Result<()> {
    let pool = PgPoolOptions::new()
        .max_connections(5)
        .connect("postgresql://postgres:postgres@localhost/library")
        .await
        .expect("unable to connect");

    sqlx::query(
        "CREATE TABLE IF NOT EXISTS public.users
                (
                    id integer NOT NULL,
                    login character varying(255) NOT NULL
                );
    ",
    )
    .execute(&pool)
    .await
    .expect("unable to create table");

    sqlx::query("truncate table users")
        .execute(&pool)
        .await
        .expect("unable to create table");

    let reqest_url =
        format!("https://api.github.com/repos/rust-lang-nursery/rust-cookbook/stargazers");
    println!("{}", reqest_url);

    let client = reqwest::Client::new();
    let res = client
        .get(&reqest_url)
        .header(USER_AGENT, "rust web api call demo")
        .send()
        .await
        .unwrap();

    let users: Vec<User> = res.json().await.unwrap();

    for i in 0..users.len() {
        sqlx::query("INSERT INTO users (id, login) VALUES ($1, $2)")
            .bind(users[i].id)
            .bind(users[i].login.clone())
            .execute(&pool)
            .await
            .expect("insert failed");
    }

    let rows: Vec<User> = sqlx::query_as("SELECT id, login FROM users")
        .fetch_all(&pool)
        .await
        .unwrap();

    println!("{:#?}", rows);

    Ok(())
}
