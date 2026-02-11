mod api;
mod local_storage;

use axum::{
    routing::post,
    Router
};

use api::{handle_logs, handle_metrics, handle_traces};
use rootly_lib::{Rootly};
use std::sync::Arc;
use tokio_cron_scheduler::{JobScheduler, Job, JobSchedulerError};

fn args_parser() -> std::collections::HashMap<String, String> {
    let args: Vec<String> = std::env::args().skip(1).collect();
    let mut args_map: std::collections::HashMap<String, String> = std::collections::HashMap::new();
    let empty_config = String::new();
    if !args.is_empty() {
        args_map = args.into_iter().map(|a| {
            if a.starts_with("--"){
                let key_value_split = a.split("=").collect::<Vec<&str>>();
                (key_value_split[0].to_string(), key_value_split.get(1).cloned().unwrap_or("").to_string())
            }
            else {
                (empty_config.clone(), empty_config.clone())
            }
        }).collect();
    }
    args_map
}

#[tokio::main]
async fn main() {
    let args_map = args_parser();
    println!("{:?}", args_map);
    let data_path = args_map.get("--data_path").cloned().unwrap();
    let local_storage = local_storage::LocalStorage::new(&data_path);
    let rootly = Rootly::init(data_path, Box::new(local_storage));
    let rootly_ref = Arc::new(rootly);

    let app = Router::new()
        .route("/v1/traces", post(handle_traces))
        .route("/v1/metrics", post(handle_metrics))
        .route("/v1/logs", post(handle_logs))
        .with_state(rootly_ref);

    let listener = tokio::net::TcpListener::bind("0.0.0.0:4318").await.unwrap();
    println!("OTLP HTTP Server listening on {}", listener.local_addr().unwrap());
    let scheduler = JobScheduler::new().await.unwrap();
    scheduler.add(
        Job::new("1/10 * * * * *", |_uuid, _l| {
            let data_path:String = rootly_ref.get_config().get::<String>("data_path").unwrap_or_default().to_string();
            println!("{}",format!("I run every 10 seconds: {data_path}"));
        }).unwrap()
    ).await.unwrap();
    scheduler.start().await;
    axum::serve(listener, app).await;
}
