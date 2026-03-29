use crate::actions::Action;
use crate::actions::Action::Exit;
use crate::component::Component;
use KeyCode::{Char, Enter};
use crossterm::event::{KeyCode, KeyModifiers};
use log::info;
use ratatui::Frame;
use ratatui::crossterm::event::Event;
use ratatui::layout::{Constraint, Direction, Layout, Rect, Size};
use ratatui::prelude::{Line, Stylize};
use ratatui::symbols::border;
use ratatui::widgets::{Block, Borders, Paragraph, ScrollbarState};
use tui_input::Input;
use tui_input::backend::crossterm::EventHandler;
use tui_scrollview::{ScrollView, ScrollViewState};

pub struct AppWidget {
    input: Input,
    content: String,
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
                .border_set(border::PLAIN);
            Paragraph::new(par).block(block)
        };

        let x = self.input.visual_cursor();
        frame.set_cursor_position((area.x + (x + 1) as u16, area.y + 1));
        frame.render_widget(input, layout[0]);

        let paragraph = Paragraph::new("aaaaaaaaaaaa\nbbbbbbbbbb\ncccccccccccc")
            .block(Block::bordered().title(" Output ").border_set(border::PLAIN));

        // let mut scroll_view = ScrollView::new(layout[1].as_size());

        info!("{:?}", area);
        info!("{:?}", layout[0]);
        info!("{:?}", layout[1]);

        // frame.render_stateful_widget(scroll_view, layout[1], &mut self.state);
        frame.render_widget(paragraph, layout[1]);

    }
}

impl AppWidget {
    pub fn new(input: &str) -> Self {
        Self {
            input: Input::from(""),
            content: input.to_string(),
            state: ScrollViewState::new(),
        }
    }
}
