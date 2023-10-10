use axum::{
    extract::Path, http::header, http::HeaderMap, response::IntoResponse, routing::get, Json,
    Router,
};

#[tokio::main]
async fn main() {
    let app = Router::new().route("/items/:username", get(get_items));

    println!("Listening on localhost:3000");
    axum::Server::bind(&"0.0.0.0:3000".parse().unwrap())
        .serve(app.into_make_service())
        .await
        .unwrap();
}

#[derive(serialize)]
struct Item {
    value: String,
    id: u32,
}

impl Item {
    fn new(value: &str, id: u32) -> Item {
        Item {
            value: value.to_string(),
            id,
        }
    }
}
async fn get_items(Path(username): Path<String>) -> impl IntoResponse {
    println!("{}", username);
    // TODO: Query the database for the user's items
    let words: Vec<Item> = vec![
        Item::new("Read book", 1),
        Item::new("Scroll instagram", 2),
        Item::new("Eat food", 3),
        Item::new("Drink water", 4),
        Item::new("Take a nap", 5),
        Item::new("1; DROP TABLE users", 6),
        Item::new("Watch a movie", 7),
    ];
    let mut header_map = HeaderMap::new();
    header_map.insert(header::ACCESS_CONTROL_ALLOW_ORIGIN, "*".parse().unwrap());
    (header_map, Json(words))
}
