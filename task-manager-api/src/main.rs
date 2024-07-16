use axum::http::StatusCode;
use axum::{routing::get, Json, Router};

use seybio_task_manager::{Collection, Task, TaskCollection};

#[tokio::main]
async fn main() {
    // initialize tracing
    tracing_subscriber::fmt::init();

    // build our application with a route
    let app = Router::new()
        // `GET /` goes to `root`
        .route("/health", get(root))
        .route("/tasks", get(get_tasks));


    // run our app with hyper, listening globally on port 3000
    let listener = tokio::net::TcpListener::bind("0.0.0.0:3001").await.unwrap();
    axum::serve(listener, app).await.unwrap();
}

// basic handler that responds with a static string
async fn root() -> &'static str {
    "it works"
}

async fn get_tasks() -> (StatusCode, Json<Vec<Task>>) {
    let mut collection = TaskCollection::new();
    let task = Task::new("test");
    collection.add_task(task);
    (StatusCode::OK, Json(collection.tasks))
}
