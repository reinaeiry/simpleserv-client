use axum::{
    extract::Json,
    routing::{get, post},
    Router,
};
use serde::{Deserialize, Serialize};
use std::{net::SocketAddr, process::Command};
use sysinfo::{System, RefreshKind, CpuRefreshKind, MemoryRefreshKind};
use tokio::time::{sleep, Duration};

#[tokio::main]
async fn main() {
    let app = Router::new()
        .route("/status", get(get_status))
        .route("/exec", post(run_command))
        .route("/power/shutdown", post(shutdown))
        .route("/power/restart", post(restart));

    let addr = SocketAddr::from(([0, 0, 0, 0], 3030));
    println!("Running on http://{}", addr);

    axum::serve(tokio::net::TcpListener::bind(addr).await.unwrap(), app)
        .await
        .unwrap();
}

#[derive(Serialize)]
struct Status {
    hostname: String,
    uptime: u64,
    total_memory: u64,
    used_memory: u64,
    cpu_usage: f32,
    temperature: Option<f32>,
}

async fn get_status() -> Json<Status> {
    let mut sys = System::new_with_specifics(
        RefreshKind::new()
            .with_memory(MemoryRefreshKind::new())
            .with_cpu(CpuRefreshKind::everything()),
    );

    sys.refresh_memory();
    sys.refresh_cpu();
    sleep(Duration::from_millis(100)).await;
    sys.refresh_cpu();

    let hostname = System::host_name().unwrap_or_else(|| "unknown".to_string());
    let uptime = System::uptime();
    let total_memory = sys.total_memory();
    let used_memory = sys.used_memory();
    let cpu_usage = {
        let cpus = sys.cpus();
        if cpus.is_empty() {
            0.0
        } else {
            cpus.iter().map(|c| c.cpu_usage()).sum::<f32>() / cpus.len() as f32
        }
    };
    let temperature = None; // TODO : Add temperature support for CPU/GPU

    Json(Status {
        hostname,
        uptime,
        total_memory,
        used_memory,
        cpu_usage,
        temperature,
    })
}

#[derive(Deserialize)]
struct CommandInput {
    cmd: String,
}

#[derive(Serialize)]
struct CommandOutput {
    output: String,
}

async fn run_command(Json(input): Json<CommandInput>) -> Json<CommandOutput> {
    let output = Command::new("cmd")
        .args(["/C", &input.cmd])
        .output()
        .expect("failed to run command");

    let stdout = String::from_utf8_lossy(&output.stdout).to_string();
    Json(CommandOutput { output: stdout })
}

async fn shutdown() -> &'static str {
    let _ = Command::new("shutdown")
        .args(["/s", "/t", "0"])
        .spawn();
    "Shutting down..."
}

async fn restart() -> &'static str {
    let _ = Command::new("shutdown")
        .args(["/r", "/t", "0"])
        .spawn();
    "Restarting..."
}
