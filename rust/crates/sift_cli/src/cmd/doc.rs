use anyhow::{Context, Result};
use axum::Router;
use include_dir::{Dir, include_dir};
use std::process::ExitCode;
use tokio::net::TcpListener;
use tower_serve_static::ServeDir;

use crate::cli::DocArgs;

static STATIC_ASSETS: Dir<'_> = include_dir!("$CARGO_MANIFEST_DIR/assets/docs/book");

pub async fn serve(args: DocArgs) -> Result<ExitCode> {
    let DocArgs { addr } = args;

    let docs = ServeDir::new(&STATIC_ASSETS);

    let router = Router::new().fallback_service(docs);

    let ln = TcpListener::bind(addr)
        .await
        .context("failed to bind socket address")?;

    println!("documentation available at tcp://{addr}");

    axum::serve(ln, router)
        .await
        .context("server exited due to error")?;

    Ok(ExitCode::SUCCESS)
}
