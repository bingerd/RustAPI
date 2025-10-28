use clap::Parser;
use std::{net::SocketAddr, sync::Arc};
use app::{create_app, SessionPool};
use anyhow::Result;
use ort::execution_providers::coreml::CoreMLExecutionProvider;
use ort::session::builder::SessionBuilder;
use ort::execution_providers::ExecutionProviderDispatch;

mod app;

/// Command-line arguments
#[derive(Parser, Debug)]
#[command(author, version, about)]
pub struct Args {
    #[arg(long, default_value = "0.0.0.0")]
    pub host: String,

    #[arg(long, default_value_t = 5000)]
    pub port: u16,

    #[arg(long, default_value_t = false)]
    pub debug: bool,

    #[arg(long, default_value_t = 4)]
    pub sessions: usize, // Number of ONNX sessions for the pool
}

#[tokio::main]
async fn main() -> Result<()> {
    let args = Args::parse();

    if args.debug {
        println!("Debug mode enabled");
    }

    println!("Starting server on {}:{}", args.host, args.port);
    let addr = SocketAddr::new(args.host.parse()?, args.port);

    // --- Create ONNX Runtime session pool ---
    let mut session_instances = Vec::with_capacity(args.sessions);
    let coreml: ExecutionProviderDispatch = CoreMLExecutionProvider::default().into();
    for _ in 0..args.sessions {
        let session = SessionBuilder::new()?
            .with_execution_providers(vec![coreml.clone()])?
            .commit_from_file("models/model.onnx")?;
        session_instances.push(session);
    }

    let pool = Arc::new(SessionPool::new(session_instances));

    // --- Create Axum app ---
    let app = create_app(pool);

    // --- Start server ---
    axum_server::bind(addr)
        .serve(app.into_make_service())
        .await?;

    Ok(())
}
