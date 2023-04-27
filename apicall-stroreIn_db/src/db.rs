use postgres::{Client, NoTls, Error};

fn main() -> Result<(), Error> {
    let mut client = Client::connect("postgresql://postgres:postgres@localhost/library", NoTls)?;
    
    client.batch_execute("
        CREATE TABLE IF NOT EXISTS user (
            id              INTEGER PRIMARY KEY,
            login            VARCHAR NOT NULL
            )
    ")?;

    Ok(())

}