# kubers-demo

### Introduction kubers 

##### Install Rust 

```
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
rustc --version
rustc 1.68.0-nightly (b70baa4f9 2022-12-14)

```


##### Install Minikube 

```
curl -LO https://storage.googleapis.com/minikube/releases/latest/minikube-darwin-amd64
sudo install minikube-darwin-amd64 /usr/local/bin/minikube
minikube start 
kubectl get ns 
NAME              STATUS   AGE
default           Active   15s
kube-node-lease   Active   15s
kube-public       Active   15s
kube-system       Active   15s
```


### initialize rust binary using cargo package manager 

```
cargo init 
```

#### update cargo.toml dependencies 

```
[package]
name = "kubers-demo"
version = "0.1.0"
edition = "2021"

# See more keys and their definitions at https://doc.rust-lang.org/cargo/reference/manifest.html

[dependencies]
kube = { version = "0.87.2", features = ["client","runtime", "derive", "rustls-tls"] }
k8s-openapi = { version = "0.20.0", features = ["latest"] }
tokio = { version = "1.0", features = ["full"] }  # Use the latest version
[dev-dependencies]
k8s-openapi = { version = "0.20.0", features = ["latest"] }
async-std = "1.0"  # Use the latest version


```

##### update main.rs 

```
use kube::{Client, Api};
use kube::api::ListParams;
use k8s_openapi::api::core::v1::Pod;
use std::error::Error;

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    // Load the kubeconfig file.
    let config = kube::Config::from_kubeconfig(&kube::config::KubeConfigOptions::default()).await?;
    let client = Client::try_from(config)?;

    // Work with Kubernetes API.
    let pods: Api<Pod> = Api::default_namespaced(client);
    let lp = ListParams::default();

    for p in pods.list(&lp).await? {
        println!("Found Pod: {}", p.metadata.name.unwrap_or_default());
    }

    Ok(())
}



```

#### deploy simple ngnix deployment 
```
kubectl apply -f https://k8s.io/examples/application/deployment.yaml

```

### Cargo build 

```cargo build ```


### Cargo Run 

```cargo run ```

run rust code to talk with kubectl and get upate to using rust kube crate 

```cargo run -- --kubers-demo kubectl -- get po```



### build own CRD with meetup example 

```
cargo init kube-crd 

```

update your ```main.rs``` 

```
use kube::{Api, Client};
use kube::api::PostParams;
use serde::{Serialize, Deserialize};
use k8s_openapi::apiextensions_apiserver::pkg::apis::apiextensions::v1::{
    CustomResourceDefinition,
    CustomResourceDefinitionSpec,
    CustomResourceDefinitionNames,
    CustomResourceDefinitionVersion,
    JSONSchemaProps,
    CustomResourceValidation,
    JSONSchemaPropsOrArray,
};
use k8s_openapi::apimachinery::pkg::apis::meta::v1::ObjectMeta;
use schemars::JsonSchema;
use kube::CustomResource;
use std::collections::BTreeMap;

// Define the spec of our custom resource
#[derive(CustomResource, Deserialize, Serialize, Clone, Debug, JsonSchema)] // This line is a combination of derive macros to add functionality to the MeetupSpec struct. It makes MeetupSpec a Kubernetes custom resource, serializable, cloneable, and debuggable.
#[kube(group = "example.com", version = "v1", kind = "Meetup", namespaced)] // This annotation specifies the API group, version, and kind of the custom resource, and that it is namespaced.
pub struct MeetupSpec {
    organizer: String,
    topic: String,
    attendees: Vec<String>,
}

// Main function to create the CRD in the cluster
#[tokio::main]
async fn main() -> Result<(), kube::Error> { // An asynchronous main function that returns a result type, which on error contains kube::Error.
    let client = Client::try_default().await?; // Attempts to create a default Kubernetes client, which connects to the cluster's API server.

    let crds: Api<CustomResourceDefinition> = Api::all(client); // : Creates an API interface for CustomResourceDefinition objects that can interact with all namespaces.
    let pp = PostParams::default(); //  Initializes default post parameters

    // Define the CRD for our Meetup resource
    let meetup_crd = CustomResourceDefinition { // Defines the CRD for the Meetup resource. It includes metadata (like the name), specifications for versions, names, scope, and the schema for validation.
        metadata: ObjectMeta {
            name: Some("meetups.example.com".to_string()),
            ..Default::default()
        },
        spec: CustomResourceDefinitionSpec {
            group: "example.com".to_string(),
            versions: vec![
                CustomResourceDefinitionVersion {
                    name: "v1".to_string(),
                    served: true,
                    storage: true,
                    schema: Some(CustomResourceValidation {
                        open_api_v3_schema: Some(JSONSchemaProps {
                            type_: Some("object".to_string()),
                            properties: Some({
                                let mut props = BTreeMap::new();
                                props.insert("organizer".to_string(), JSONSchemaProps {
                                    type_: Some("string".to_string()),
                                    ..Default::default()
                                });
                                props.insert("topic".to_string(), JSONSchemaProps {
                                    type_: Some("string".to_string()),
                                    ..Default::default()
                                });
                                props.insert("attendees".to_string(), JSONSchemaProps {
                                    type_: Some("array".to_string()),
                                    items: Some(JSONSchemaPropsOrArray::Schema(Box::new(JSONSchemaProps {
                                        type_: Some("string".to_string()),
                                        ..Default::default()
                                    }))),
                                    ..Default::default()
                                });
                                props
                            }),
                            ..Default::default()
                        }),
                    }),
                    ..Default::default()
                }
            ],
            names: CustomResourceDefinitionNames {
                plural: "meetups".to_string(),
                singular: Some("meetup".to_string()),
                kind: "Meetup".to_string(),
                short_names: Some(vec!["mtup".to_string()]),
                ..Default::default()
            },
            scope: "Namespaced".to_string(),
            ..Default::default()
        },
        status: None,
    };

    // Create the CRD
    crds.create(&pp, &meetup_crd).await?;

    Ok(())
}


```

### build you code 

```
cargo build 
```

### deploy simple kubernetes app 

```

 kubectl apply -f https://k8s.io/examples/application/deployment.yaml

```

### Cargo run 

```
cargo run -- --kubers-demo kubectl -- get po    

```
### get the CRD 

```
kubectl get crd  
```

### deploy own CRD via YAML 

```
apiVersion: example.com/v1
kind: Meetup
metadata:
  name: kubernetes-intro-meetup
  namespace: default
organizer: "sangam"
topic: "Introduction to Kube rs with sangam"
attendees:
  - "abhi"
  - "lavakush"
  - "sangam"
```

```
kubectl apply -f meetup-resource.yaml  
```


```
kubectl get meetups -n default  

```

```
kubectl get meetup kubernetes-intro-meetup -n default -o jsonpath='{.attendees[*]}'

```


### Lets play around kube-open-api 

```
cargo init kube-rs-slack

```

## update main.rs file wih following 

```
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

    let slack_webhook_url = "******"; // Replace with your Slack webhook URL

    tokio::pin!(watcher);
    while let Some(event) = watcher.try_next().await? {
        if let watcher::Event::Applied(pod) = event {
            let pod_name = pod.metadata.name.unwrap_or_default();  
            let message = format!("Pod update: {}", pod_name);
            send_slack_message(slack_webhook_url, &message).await;
        }
    }

    Ok(())
}


```

### create custom app and set channel where you get messages 

lets run the binary ```cargo run``` 

open another terminal do any deployements you get updates in channel 

