mod values;
mod app;
mod actions;
mod app_widget;
mod component;

use anyhow::Result;
use clap::Parser;
use env_logger::{Builder, Target};
use log::{LevelFilter, error, info};
use std::fs::OpenOptions;
use crate::app::App;
use values::APP_NAME;

#[tokio::main]
async fn main() {
    let file = OpenOptions::new()
        .create(true)
        .append(true)
        .open(format!("/tmp/{APP_NAME}.log"))
        .expect("Failed to open log file");

    // Configure env_logger to write logs to the file
    Builder::new()
        .target(Target::Pipe(Box::new(file)))
        .filter_level(LevelFilter::Debug)
        .init();

    info!("Starting");

    let args = Args::parse();

    info!("{args:?}");

    match run(args).await {
        Ok(()) => {}
        Err(e) => {
            error!("{e}");
        }
    }
}

#[derive(Parser, Debug)]
#[command(version = crate::values::VERSION, about, long_about = None)]
struct Args {

}

async fn run(_args: Args) -> Result<()> {
    let mut terminal = ratatui::init();

    let app = App::new();
    app.run(&mut terminal).await?;

    ratatui::restore();

    Ok(())
}
