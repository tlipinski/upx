use crate::actions::Action;
use crate::actions::Action::Init;
use crate::app_widget::AppWidget;
use crate::component::Component;
use crate::input_handler::handle_input_task;
use Action::Exit;
use log::info;
use ratatui::crossterm::event::Event;
use ratatui::layout::{Constraint, Direction, Layout, Rect};
use ratatui::prelude::{Line, Stylize};
use ratatui::symbols::border;
use ratatui::widgets::{Block, Paragraph};
use ratatui::{DefaultTerminal, Frame};
use tokio::sync::mpsc::{Receiver, Sender};
use tui_input::Input;
use tui_input::backend::crossterm::EventHandler;

pub struct App {
    ui_tx: Sender<Action>,
    ui_rx: Receiver<Action>,
    widget: Box<dyn Component>,
}

impl App {
    pub fn new() -> Self {
        let (ui_tx, ui_rx) = tokio::sync::mpsc::channel::<Action>(100);

        App {
            ui_tx,
            ui_rx,
            widget: Box::new(AppWidget::new()),
        }
    }

    pub async fn run(mut self, terminal: &mut DefaultTerminal) -> anyhow::Result<()> {
        tokio::spawn(handle_input_task(self.ui_tx.clone()));

        let mut message_opt = Some(Init);

        'main_loop: loop {
            while let Some(msg) = message_opt {
                match msg {
                    Exit => {
                        info!("Exiting application");
                        break 'main_loop;
                    }
                    _ => message_opt = self.update(&msg),
                }
            }

            terminal.draw(|frame| self.widget.render(frame, frame.area()))?;

            message_opt = self.ui_rx.recv().await;
        }

        Ok(())
    }

    fn update(&mut self, action: &Action) -> Option<Action> {
        match action {
            Action::InputReceived(evt) => {
                self.widget.handle_key_event(evt);
                None
            }
            _ => None,
        }
    }
}
