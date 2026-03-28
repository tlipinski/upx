use crate::actions::Action;
use crate::component::Component;
use crossterm::event::KeyCode;
use ratatui::Frame;
use ratatui::crossterm::event::Event;
use ratatui::layout::{Constraint, Direction, Layout, Rect};
use ratatui::prelude::{Line, Stylize};
use ratatui::symbols::border;
use ratatui::widgets::{Block, Paragraph};
use tui_input::Input;
use tui_input::backend::crossterm::EventHandler;

pub struct AppWidget {
    input: Input,
}

impl Component for AppWidget {
    fn update(&mut self, action: &Action) -> Option<Action> {
        None
    }

    fn handle_key_event(&mut self, event: &Event) -> Option<Action> {
        if let Event::Key(key_event) = event {
            match key_event.code {
                KeyCode::Enter => {
                    todo!()
                }
                _ => {
                    self.input.handle_event(event);
                    None
                }
            }
        } else {
            None
        }
    }

    fn render(&mut self, frame: &mut Frame, area: Rect) {
        let layout = Layout::default()
            .direction(Direction::Vertical)
            .constraints(vec![Constraint::Length(3), Constraint::Fill(1)])
            .split(area);

        let input = {
            let par = Line::from(self.input.value().bold());
            let block = Block::bordered()
                .title(" Command ")
                .border_set(border::PLAIN);
            Paragraph::new(par).block(block)
        };

        let x = self.input.visual_cursor();
        frame.set_cursor_position((area.x + (x + 1) as u16, area.y + 1));

        frame.render_widget(input, layout[0]);
    }
}

impl AppWidget {
    pub fn new() -> Self {
        Self {
            input: Input::new(String::new()),
        }
    }
}
