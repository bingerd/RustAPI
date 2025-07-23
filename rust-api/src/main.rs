use clap::Parser;
use std::{net::SocketAddr, sync::Arc};
use app::{create_app, AppState};
use tch::CModule;

mod app;

#[derive(Parser, Debug)]
#[command(author, version, about)]
pub struct Args {
    #[arg(long, default_value = "0.0.0.0")]
    pub host: String,

    #[arg(long, default_value_t = 5000)]
    pub port: u16,

    #[arg(long, default_value_t = false)]
    pub debug: bool,
}

#[tokio::main]
async fn main() {
    let args = Args::parse();

    if args.debug {
        println!("Debug mode enabled");
    }

    println!("Starting server on {}:{}", args.host, args.port);

    let addr = SocketAddr::new(args.host.parse().unwrap(), args.port);

    // Load models here and inject them into app state
    let model = CModule::load("scripted_model.pt").expect("Failed to load model");
    // let lookup = CModule::load("scripted_lookup.pt").expect("Failed to load lookup"); // Impossible because it is a custom Python class.
    let shared_state = Arc::new(AppState { model });

    let app = create_app(shared_state);

    axum_server::bind(addr)
        .serve(app.into_make_service())
        .await
        .unwrap();

}