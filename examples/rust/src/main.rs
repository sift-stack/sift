use anyhow::Result;
use dotenv::dotenv;
use serde_json;
use std::{env, io::Write, process::ExitCode, str::FromStr};
use tonic::{metadata::MetadataValue, transport::Channel, Request};

/// Sift generated protos
use sift::gen::sift::annotations::v1::{
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
        let annotation_json = serde_json::to_string_pretty(annotation)?;
        writeln!(&mut buffer, "{annotation_json}")?;
    }

    let out = String::from_utf8(buffer)?;

    println!("{out}");

    Ok(())
}
