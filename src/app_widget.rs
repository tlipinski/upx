use crate::actions::Action;
use crate::actions::Action::Exit;
use crate::component::Component;
use KeyCode::{Char, Enter};
use crossterm::event::{KeyCode, KeyModifiers};
use ratatui::Frame;
use ratatui::crossterm::event::Event;
use ratatui::layout::{Constraint, Direction, Layout, Rect, Size};
use ratatui::prelude::{Line, Stylize};
use ratatui::symbols::border;
use ratatui::widgets::{Block, Paragraph, ScrollbarState};
use tui_input::Input;
use tui_input::backend::crossterm::EventHandler;
use tui_scrollview::{ScrollView, ScrollViewState};

pub struct AppWidget {
    input: Input,
    state: ScrollViewState,
}

impl Component for AppWidget {
    fn update(&mut self, _action: &Action) -> Option<Action> {
        None
    }

    fn handle_key_event(&mut self, event: &Event) -> Option<Action> {
        if let Event::Key(key_event) = event {
            match (key_event.code, key_event.modifiers) {
                (Char('c'), KeyModifiers::CONTROL) => Some(Exit),
                (Enter, KeyModifiers::NONE) => {
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

        frame.render_widget(input.clone(), layout[0]);

        let mut view = ScrollView::new(layout[1].as_size());
        view.render_widget(Paragraph::new("Test"), layout[1]);

        frame.render_stateful_widget(view, layout[1], &mut self.state);
    }
}

impl AppWidget {
    pub fn new(input: &str) -> Self {
        Self {
            input: Input::from(""),
            state: ScrollViewState::new(),
        }
    }
}
