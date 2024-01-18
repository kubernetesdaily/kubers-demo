use kube::{Client, Api}; // Imports the Client and Api types from the kube library for interacting with the Kubernetes API
use kube::api::ListParams; // Imports the ListParams type for constructing list parameters for API calls.
use k8s_openapi::api::core::v1::Pod; // Imports the Pod type from the k8s_openapi library, representing a Kubernetes Pod resource.
use std::error::Error; // Imports the Error trait for handling errors

#[tokio::main] // Marks the main function as asynchronous, using the tokio runtime for handling concurrent operations.
async fn main() -> Result<(), Box<dyn Error>> { // Defines an asynchronous main function that returns a Result type, either Ok(()) on success or an Error if something goes wrong.
    // Load the kubeconfig file.
    let config = kube::Config::from_kubeconfig(&kube::config::KubeConfigOptions::default()).await?; // Loads the Kubernetes configuration from the default kubeconfig file.
    let client = Client::try_from(config)?; // Creates a Client instance from the loaded configuration for interacting with the Kubernetes API.

    // Work with Kubernetes API.
    let pods: Api<Pod> = Api::default_namespaced(client); // Creates an Api instance for working with Pods in the default namespace.
    let lp = ListParams::default(); // Creates default list parameters for listing Pods

    for p in pods.list(&lp).await? { // terates through the list of Pods returned by the API call. 
        println!("Found Pod: {}", p.metadata.name.unwrap_or_default()); // Prints the name of each Pod, handling potential None values
    }

    Ok(()) // Returns Ok to indicate successful execution
}
