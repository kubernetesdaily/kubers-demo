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

