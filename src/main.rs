use axum::{http::StatusCode, routing, Json, Router};
use serde::Serialize;

#[derive(Serialize)]
struct Todo {
    id: i32,
    name: String,
}

async fn get_todos() -> (StatusCode, Json<Vec<Todo>>) {
    let todos = vec![
        Todo {
            id: 1,
            name: String::from("Do something A"),
        },
        Todo {
            id: 2,
            name: String::from("Do Something B"),
        },
    ];
    return (StatusCode::OK, Json(todos));
}

#[tokio::main]
async fn main() {
    let app = Router::new().route("/todos", routing::get(get_todos));

    let listener = tokio::net::TcpListener::bind("0.0.0.0:8080").await.unwrap();
    axum::serve(listener, app).await.unwrap();
}
