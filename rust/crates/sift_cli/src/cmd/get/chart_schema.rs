use std::{io::Write, process::ExitCode};

use anyhow::{Context as AnyhowContext, Result};
use sift_mcp::{DeclarativeService, RestConfig};

use crate::{cli::GetChartSchemaArgs, cmd::Context};

pub async fn run(ctx: Context, args: GetChartSchemaArgs) -> Result<ExitCode> {
    let schema = DeclarativeService::new(
        Some(RestConfig::new(ctx.rest_uri, ctx.api_key)),
        env!("CARGO_PKG_VERSION"),
    )
    .chart_schema_bytes()
    .await?;

    if let Some(path) = args.output {
        tokio::fs::write(&path, schema)
            .await
            .with_context(|| format!("failed to write `{}`", path.display()))?;
        println!("{}", path.display());
    } else {
        std::io::stdout()
            .write_all(&schema)
            .context("failed to write the declarative chart schema")?;
    }

    Ok(ExitCode::SUCCESS)
}

#[cfg(test)]
mod test {
    use std::{fs, net::SocketAddr};

    use axum::{
        Router,
        http::{HeaderMap, StatusCode},
        response::IntoResponse,
        routing::get,
    };
    use tempdir::TempDir;
    use tokio::net::TcpListener;

    use super::run;
    use crate::{cli::GetChartSchemaArgs, cmd::Context};

    const SCHEMA: &[u8] = br#"{
  "$defs": {"axis": {"type": "object"}}
}
"#;

    #[tokio::test]
    async fn writes_the_chart_schema_to_a_file() {
        let router = Router::new()
            .route(
                "/api/packages/v1/schemas/declarative-chart/versions",
                get(|| async { r#"{"versions":[{"version":"0"}]}"# }),
            )
            .route(
                "/download/schemas/declarative-chart/0/sift.declarative.schema.v0.json",
                get(schema),
            );
        let listener = TcpListener::bind("127.0.0.1:0").await.unwrap();
        let address: SocketAddr = listener.local_addr().unwrap();
        let server = tokio::spawn(async move { axum::serve(listener, router).await });
        let directory = TempDir::new("chart-schema").unwrap();
        let path = directory.path().join("chart-schema.json");

        run(
            Context {
                grpc_uri: "unused".to_string(),
                rest_uri: format!("http://{address}"),
                api_key: "test-key".to_string(),
                disable_tls: false,
                app_uri: None,
            },
            GetChartSchemaArgs {
                output: Some(path.clone()),
            },
        )
        .await
        .unwrap();

        assert_eq!(fs::read(path).unwrap(), SCHEMA);
        server.abort();
    }

    async fn schema(headers: HeaderMap) -> impl IntoResponse {
        assert_eq!(headers.get("authorization").unwrap(), "Bearer test-key");
        (StatusCode::OK, SCHEMA)
    }
}
