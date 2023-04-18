use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize)]
struct Paragraph{
    name: String
}

#[derive(Serialize, Deserialize)]
struct Article{
    article: String,
    author: String,
    paragraphs: Vec<Paragraph>
}

fn main(){
    let json = r#"{
        "article": "hello there this is a fiest article",
        "author": "Arpit Anand",
        "paragraphs": [
            {
                "name": "build a bulding step by step"
            },
            {
                "name": "come together to be a strong community"
            }
        ]

    }"#;
    let parse: Article = read_from_json(json);
    println!("{}", parse.paragraphs[0].name);
}

fn read_from_json(rawjson: &str) -> Article{
    let parse = serde_json::from_str(rawjson).unwrap();
    return parse
}
