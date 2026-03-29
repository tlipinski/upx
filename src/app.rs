use futures_util::FutureExt;
use std::time::Duration;
use crossterm::event::EventStream;
use futures_util::StreamExt;
use crate::actions::Action;
use color_eyre::Result;
use crate::actions::Action::{Init, Tick};
use crate::app_widget::AppWidget;
use crate::component::Component;
use Action::Exit;
use log::{error, info};
use ratatui::DefaultTerminal;
use tokio::sync::mpsc::{Receiver, Sender};
use tokio::time::interval;

pub struct App {
    ui_tx: Sender<Action>,
    ui_rx: Receiver<Action>,
    widget: Box<dyn Component>,
}

impl App {
    pub fn new(input: &str) -> Self {
        let (ui_tx, ui_rx) = tokio::sync::mpsc::channel::<Action>(100);

        App {
            ui_tx,
            ui_rx,
            widget: Box::new(AppWidget::new(input)),
        }
    }

    pub async fn run(mut self, terminal: &mut DefaultTerminal) -> Result<()> {
        let mut event_stream = EventStream::new();
        let mut tick_interval = interval(Duration::from_secs_f64(1.0 / 4.0));

        let mut message_opt = Some(Init);

        'main_loop: loop {
            while let Some(msg) = message_opt {
                match msg {
                    Exit => {
                        info!("Exiting application");
                        break 'main_loop;
                    }
                    _ => message_opt = self.widget.update(&msg),
                }
            }

            terminal.draw(|frame| self.widget.render(frame, frame.area()))?;

            message_opt = tokio::select! {
                maybe_event = event_stream.next().fuse() => match maybe_event {
                    Some(Ok(event)) => self.widget.handle_key_event(&event),
                    Some(Err(err)) => {
                        error!("{err}");
                        return Err(err.into())
                    },
                    None => break (),
                },
                ui = self.ui_rx.recv().fuse() => ui,
                // _ = tick_interval.tick() => Some(Tick),
            };
        }

        Ok(())
    }

}
