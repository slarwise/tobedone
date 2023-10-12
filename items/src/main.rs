use axum::{
    extract::Path, http::header, http::HeaderMap, response::IntoResponse, routing::get, Json,
    Router,
};
use redis::Commands;
use serde::Serialize;

#[tokio::main]
async fn main() {
    let app = Router::new().route("/items/:username", get(get_items));

    println!("Listening on localhost:3000");
    axum::Server::bind(&"0.0.0.0:3000".parse().unwrap())
        .serve(app.into_make_service())
        .await
        .unwrap();
}

#[derive(Serialize)]
struct Item {
    value: String,
    id: usize,
}

impl Item {
    fn new(value: &str, id: usize) -> Item {
        Item {
            value: value.to_string(),
            id,
        }
    }
}
async fn get_items(Path(username): Path<String>) -> impl IntoResponse {
    println!("{}", username);
    let client = redis::Client::open("redis://localhost:6379").unwrap();
    let mut con = client.get_connection().unwrap();
    let tasks: Vec<String> = con.smembers("tasks:default").unwrap();
    let mut words: Vec<Item> = vec![];
    for task in tasks {
        match task.split_once(':') {
            Some((id, task)) => {
                let id: usize = id.parse().unwrap();
                words.push(Item::new(&task, id));
            }
            None => panic!("Incorrect schema: database tasks must be on the form `<id>:<task>`"),
        }
    }
    let mut header_map = HeaderMap::new();
    header_map.insert(header::ACCESS_CONTROL_ALLOW_ORIGIN, "*".parse().unwrap());
    (header_map, Json(words))
}
