use crossterm::event::{self, KeyCode};
use ratatui::Frame;
use ratatui::layout::{Constraint, Layout, Rect};
use ratatui::style::{Color, Style, Stylize};
use ratatui::text::{Line, Span};
use ratatui::widgets::{Row, Table, TableState};

use crate::task::Task;

pub struct TasksTable<'a> {
    tasks: &'a Vec<Task>,
}

impl<'a> TasksTable<'a> {
    pub fn new(tasks: &'a Vec<Task>) -> TasksTable<'a> {
        return Self { tasks };
    }

    pub fn draw(&self) {
        let mut table_state = TableState::default();
        table_state.select_first();
        table_state.select_first_column();
        ratatui::run(|terminal| {
            loop {
                terminal
                    .draw(|frame| self.render(frame, &mut table_state))
                    .unwrap();
                if let Some(key) = event::read().unwrap().as_key_press_event() {
                    match key.code {
                        KeyCode::Esc => return,
                        KeyCode::Down => table_state.select_next(),
                        KeyCode::Up => table_state.select_previous(),
                        KeyCode::Right => table_state.select_next_column(),
                        KeyCode::Left => table_state.select_previous_column(),
                        _ => {}
                    }
                }
            }
        })
    }

    /// Render the UI with a table.
    fn render(&self, frame: &mut Frame, table_state: &mut TableState) {
        let layout = Layout::vertical([Constraint::Length(1), Constraint::Fill(1)]).spacing(1);
        let [top, main] = frame.area().layout(&layout);

        let title = Line::from_iter([
            Span::from("Tasks list").bold(),
            Span::from(" (Press 'q' to quit and arrow keys to navigate)"),
        ]);
        frame.render_widget(title.centered(), top);
        self.render_table(frame, main, table_state);
    }

    /// Render a table with some rows and columns.
    pub fn render_table(&self, frame: &mut Frame, area: Rect, table_state: &mut TableState) {
        let header = Row::new(["ID", "Title", "Stage"])
            .style(Style::new().bold())
            .bottom_margin(1);

        let mut rows: Vec<Row> = Vec::new();
        let stages: Vec<String> = self
            .tasks
            .iter()
            .map(|task| task.stage.as_string())
            .collect();

        for (task, stage) in self.tasks.iter().zip(stages.iter()) {
            rows.push(Row::new([
                task.id.as_str(),
                task.title.as_str(),
                stage.as_str(),
            ]));
        }

        let widths = [
            Constraint::Percentage(20),
            Constraint::Percentage(50),
            Constraint::Percentage(30),
        ];
        let table = Table::new(rows, widths)
            .header(header)
            .column_spacing(1)
            .style(Color::White)
            .row_highlight_style(Style::new().on_black().bold())
            .column_highlight_style(Color::Gray)
            .cell_highlight_style(Style::new().reversed().yellow())
            .highlight_symbol("> ");

        frame.render_stateful_widget(table, area, table_state);
    }
}
