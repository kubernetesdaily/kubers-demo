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
#[derive(CustomResource, Deserialize, Serialize, Clone, Debug, JsonSchema)]
#[kube(group = "example.com", version = "v1", kind = "Meetup", namespaced)]
pub struct MeetupSpec {
    organizer: String,
    topic: String,
    attendees: Vec<String>,
}

// Main function to create the CRD in the cluster
#[tokio::main]
async fn main() -> Result<(), kube::Error> {
    let client = Client::try_default().await?;

    let crds: Api<CustomResourceDefinition> = Api::all(client);
    let pp = PostParams::default();

    // Define the CRD for our Meetup resource
    let meetup_crd = CustomResourceDefinition {
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
