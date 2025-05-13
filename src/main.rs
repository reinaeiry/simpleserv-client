use axum::{
    extract::{Query, Json},
    response::Html,
    routing::{get, post},
    Router,
};
use std::{collections::HashMap, net::SocketAddr};

#[tokio::main]
async fn main() {
    let app = Router::new()
        .route("/", get(dashboard))
        .route("/status", get(fetch_status))
        .route("/exec", get(exec_command))
        .route("/power/shutdown", post(shutdown))
        .route("/power/restart", post(restart));

    let addr = SocketAddr::from(([0, 0, 0, 0], 6969));
    println!("Control panel running at http://{}", addr);

    axum::serve(tokio::net::TcpListener::bind(addr).await.unwrap(), app)
        .await
        .unwrap();
}

async fn dashboard() -> Html<&'static str> {
    Html(include_str!("dashboard.html"))
}

async fn fetch_status() -> Html<String> {
    let Ok(response) = reqwest::get("http://0.0.0.0:3030/status").await else {
        return Html("{\"error\":\"Failed to reach client\"}".to_string());
    };
    let Ok(body) = response.text().await else {
        return Html("{\"error\":\"Invalid response\"}".to_string());
    };
    Html(body)
}

async fn exec_command(Query(params): Query<HashMap<String, String>>) -> Html<String> {
    if let Some(cmd) = params.get("cmd") {
        let res = reqwest::Client::new()
            .post("http://0.0.0.0:3030/exec")
            .json(&serde_json::json!({ "cmd": cmd }))
            .send()
            .await;

        match res {
            Ok(response) => match response.json::<HashMap<String, String>>().await {
                Ok(json) => Html(json.get("output").cloned().unwrap_or("No output".to_string())),
                Err(_) => Html("Failed to parse command output.".to_string()),
            },
            Err(_) => Html("Failed to send command.".to_string()),
        }
    } else {
        Html("No command provided".to_string())
    }
}

async fn shutdown() -> Html<String> {
    let _ = reqwest::Client::new()
        .post("http://0.0.0.0:3030/power/shutdown")
        .send()
        .await;

    Html("Shutdown signal sent.".to_string())
}

async fn restart() -> Html<String> {
    let _ = reqwest::Client::new()
        .post("http://0.0.0.0:3030/power/restart")
        .send()
        .await;

    Html("Restart signal sent.".to_string())
}
