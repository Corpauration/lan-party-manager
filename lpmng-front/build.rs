use std::path::PathBuf;
use tracing_subscriber::layer::SubscriberExt;
use tracing_subscriber::util::SubscriberInitExt;

fn main() {
    tracing_subscriber::registry()
        .with(tracing_subscriber::fmt::layer())
        .with(if std::env::var("RUST_LOG").is_ok() {
            tracing_subscriber::EnvFilter::from_default_env()
        } else {
            tracing_subscriber::EnvFilter::new("info")
        })
        .init();

    let project = ines_core::project::Project::load_from_path(PathBuf::from(".")).unwrap();
    project.build().unwrap();

    println!("cargo::rerun-if-changed=src");
    println!("cargo::rerun-if-changed=ines.toml");
}