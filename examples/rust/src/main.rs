use anyhow::Result;
use dotenv::dotenv;
use indoc::writedoc;
use std::{env, io::Write, process::ExitCode, str::FromStr};
use tonic::{metadata::MetadataValue, transport::Channel, Request};

/// Sift generated protos
mod gen;
use gen::annotations::{
    annotation_service_client::AnnotationServiceClient, ListAnnotationsRequest,
    ListAnnotationsResponse,
};

#[tokio::main]
async fn main() -> ExitCode {
    if let Err(err) = run().await {
        eprintln!("{err}");
        return ExitCode::FAILURE;
    }
    ExitCode::SUCCESS
}

#[derive(Debug, thiserror::Error)]
enum Error<'a> {
    #[error("Argument cannot be blank")]
    BlankArgument,
    #[error("Expected a name to be provided")]
    MissingArgument,
    #[error("Missing '{0}' in .env")]
    MissingEnvVar(&'a str),
}

async fn run() -> Result<()> {
    dotenv().ok();

    let auth_token = env::var("SIFT_API_KEY")
        .map_err(|_| Error::MissingEnvVar("SIFT_API_KEY"))
        .map(|s| format!("Bearer {s}"))?;

    let base_uri = env::var("BASE_URI")
        .map_err(|_| Error::MissingEnvVar("BASE_URI"))
        .map(Box::new)
        .map(Box::leak)?;

    let name = match env::args().nth(1) {
        Some(n) if n.is_empty() => return Err(Error::MissingArgument.into()),
        Some(n) => n,
        None => return Err(Error::BlankArgument.into()),
    };

    let channel = Channel::from_static(base_uri).connect().await?;

    let auth_token = MetadataValue::from_str(&auth_token)?;

    let mut annotations_client =
        AnnotationServiceClient::with_interceptor(channel, move |mut req: Request<()>| {
            req.metadata_mut()
                .insert("authorization", auth_token.clone());
            Ok(req)
        });

    let list_annotations_req = ListAnnotationsRequest {
        filter: format!("name.matches(\"(?i){name}\")"),
        page_size: 10,
        page_token: "".to_string(),
    };

    let ListAnnotationsResponse { annotations, .. } = annotations_client
        .list_annotations(list_annotations_req)
        .await?
        .into_inner();

    if annotations.is_empty() {
        println!("No annotations found whose name matches '{name}'")
    }

    let mut buffer = Vec::new();

    for annotation in &annotations {
        writedoc! {
            buffer,
            "
            Annotation ID: {}
                Name: {}
                Description: {}
                State: {}
                Type: {}
                Created at: {}
                Modified at: {}
                Created by rule condition ID: {}

            ",
            annotation.annotation_id,
            annotation.name,
            annotation.description,
            annotation.state().as_str_name(),
            annotation.annotation_type().as_str_name(),
            annotation.created_date.as_ref().map(|t| t.to_string()).unwrap_or_else(String::new),
            annotation.modified_date.as_ref().map(|t| t.to_string()).unwrap_or_else(String::new),
            annotation.created_by_condition_id.as_deref().unwrap_or(""),
        }?;
    }

    let out = String::from_utf8(buffer)?;

    println!("{out}");

    Ok(())
}
