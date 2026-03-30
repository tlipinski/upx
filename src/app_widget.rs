use crate::actions::Action;
use crate::actions::Action::Exit;
use KeyCode::{Char, Down, Enter, PageDown, PageUp, Up};
use crossterm::event::{KeyCode, KeyModifiers};
use log::{debug, info};
use ratatui::Frame;
use ratatui::crossterm::event::Event;
use ratatui::layout::{Constraint, Direction, Layout, Rect, Size};
use ratatui::prelude::StatefulWidget;
use ratatui::prelude::{Line, Stylize};
use ratatui::symbols::border;
use ratatui::widgets::{Block, Borders, Paragraph, ScrollbarState};
use tui_input::Input;
use tui_input::backend::crossterm::EventHandler;
use tui_scrollview::{ScrollView, ScrollViewState, ScrollbarVisibility};

pub struct AppWidget {
    input: Input,
    content: String,
    state: ScrollViewState,
}

impl AppWidget {
    pub fn update(&mut self, _action: &Action) -> Option<Action> {
        None
    }

    pub fn handle_key_event(&mut self, event: &Event) -> Option<Action> {
        if let Event::Key(key_event) = event {
            match (key_event.code, key_event.modifiers) {
                (Char('c'), KeyModifiers::CONTROL) => Some(Exit),
                (Enter, KeyModifiers::NONE) => {
                    todo!()
                }
                (Down, KeyModifiers::NONE) => {
                    self.state.scroll_down();
                    None
                }
                (PageDown, KeyModifiers::NONE) | (Char('d'), KeyModifiers::CONTROL) => {
                    self.state.scroll_page_down();
                    None
                }
                (Up, KeyModifiers::NONE) => {
                    self.state.scroll_up();
                    None
                }
                (PageUp, KeyModifiers::NONE) | (Char('u'), KeyModifiers::CONTROL) => {
                    self.state.scroll_page_up();
                    None
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

    pub fn render(&mut self, frame: &mut Frame, area: Rect) {
        let layout = Layout::default()
            .direction(Direction::Vertical)
            .constraints(vec![Constraint::Length(3), Constraint::Fill(1)])
            .split(area);

        let input = {
            let par = Line::from(self.input.value().bold());
            let block = Block::bordered().border_set(border::PLAIN);
            Paragraph::new(par).block(block)
        };

        let x = self.input.visual_cursor();
        frame.set_cursor_position((area.x + (x + 1) as u16, area.y + 1));
        frame.render_widget(input, layout[0]);

        let lines = self.content.lines().collect::<Vec<_>>().iter().len();
        debug!("lines: {}", lines);
        let content_size = Size::new(100, lines as u16);
        let mut scroll_view = ScrollView::new(content_size);

        let line_numbers = (1..=lines).map(|i| format!("{:>3}\n", i)).collect::<String>();

        // the layout doesn't have to be hardcoded like this, this is just an example
        scroll_view.render_widget(Paragraph::new(line_numbers), Rect::new(0, 0, 5, 200));
        scroll_view.render_widget(
            Paragraph::new(self.content.clone()),
            Rect::new(5, 0, 95, 200),
        );

        scroll_view.render(layout[1], frame.buffer_mut(), &mut self.state);
    }

    pub fn new(input: &str) -> Self {
        Self {
            input: Input::from(""),
            content: input.to_string(),
            state: ScrollViewState::new(),
        }
    }
}
