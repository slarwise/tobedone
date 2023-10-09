use axum::{http::header, http::HeaderMap, response::IntoResponse, routing::get, Json, Router};

#[tokio::main]
async fn main() {
    let app = Router::new().route("/items", get(get_items));

    println!("Listening on localhost:3000");
    axum::Server::bind(&"0.0.0.0:3000".parse().unwrap())
        .serve(app.into_make_service())
        .await
        .unwrap();
}

async fn get_items() -> impl IntoResponse {
    let words: Vec<String> = vec![
        String::from("Read book"),
        String::from("Scroll instagram"),
        String::from("Eat food"),
        String::from("Drink water"),
        String::from("Take a nap"),
        String::from("1; DROP TABLE users"),
        String::from("Watch a movie"),
    ];
    let mut header_map = HeaderMap::new();
    header_map.insert(header::ACCESS_CONTROL_ALLOW_ORIGIN, "*".parse().unwrap());
    (header_map, Json(words))
}
