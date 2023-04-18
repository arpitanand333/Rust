use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize)]
struct Paragraph{
    name: String
}

#[derive(Serialize, Deserialize)]
struct Article{
    title: String,
    author:String,
    paragraphs: Vec<Paragraph>
}

fn main(){
    let article = Article{
        title: String::from("wtite in json"),
        author: String::from("chemban"),
        paragraphs: vec![
            Paragraph{name: String::from("this is title")},
            Paragraph{name: String::from("this is body")},
            Paragraph{name: String::from("this is end")}
        ]
    };
    let json = serde_json::to_string(&article).unwrap();
    println!("{}",json);
}

