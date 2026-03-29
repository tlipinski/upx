mod values;
mod app;
mod actions;
mod app_widget;
mod component;

use color_eyre::Result;
use clap::Parser;
use env_logger::{Builder, Target};
use log::{LevelFilter, error, info};
use std::fs::OpenOptions;
use std::io::{stdin, Read};
use std::os::fd::{AsFd, BorrowedFd};
use tokio::io;
use tokio::io::AsyncRead;
use crate::app::App;
use values::APP_NAME;

#[tokio::main]
async fn main() {
    info!("Starting application");

    color_eyre::install().unwrap();

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

    let args = Args::parse();

    info!("{args:?}");

    let mut input = String::new();
    // stdin().read_to_string(&mut input).expect("Failed to read input");

    info!("{input:?}");

    match run(args, &input).await {
        Ok(()) => {
            info!("Exiting application");
        }
        Err(e) => {
            error!("{e}");
        }
    }
}

#[derive(Parser, Debug)]
#[command(version = crate::values::VERSION, about, long_about = None)]
struct Args {

}

async fn run(_args: Args, stdin: &str) -> Result<()> {
    info!("Starting TUI");
    let mut terminal = ratatui::init();

    let app = App::new(stdin);
    app.run(&mut terminal).await?;

    ratatui::restore();

    Ok(())
}
