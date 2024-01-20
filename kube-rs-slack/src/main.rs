use kube::{Client, Api};
use kube::runtime::watcher;
use k8s_openapi::api::core::v1::Pod;
use tokio;
use reqwest;
use serde_json::json;
use futures_util::TryStreamExt;

async fn send_slack_message(webhook_url: &str, message: &str) {
    let client = reqwest::Client::new();
    if let Err(e) = client.post(webhook_url)
        .json(&json!({ "text": message }))
        .send()
        .await {
        eprintln!("Failed to send message to Slack: {}", e);
    }
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let client = Client::try_default().await?;
    let pods: Api<Pod> = Api::all(client.clone());

    let watcher = watcher(pods, Default::default());

    let slack_webhook_url = "https://hooks.slack.com/services/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"; // Replace with your Slack webhook URL

    tokio::pin!(watcher);
    while let Some(event) = watcher.try_next().await? {
        if let watcher::Event::Applied(pod) = event {
            let pod_name = pod.metadata.name.unwrap_or_default();  // Corrected way to get name
            let message = format!("Pod update: {}", pod_name);
            send_slack_message(slack_webhook_url, &message).await;
        }
    }

    Ok(())
}
