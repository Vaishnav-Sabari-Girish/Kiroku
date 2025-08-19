mod expr;
mod parser;
mod eval;
mod truth_table;

use ratatui::{
    backend::CrosstermBackend,
    layout::{Constraint, Direction, Layout},
    style::{Color, Style},
    text::Text,
    widgets::{Block, Borders, Paragraph},
    Terminal,
};
use crossterm::{
    event::{self, Event, KeyCode},
    terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
    execute,
};
use parser::parse_expr;
use truth_table::truth_table;
use std::io;

fn expr_input() -> Result<String, io::Error> {
    //Set up the terminal
    enable_raw_mode()?;
    let mut stdout = io::stdout();
    execute!(stdout, EnterAlternateScreen)?;
    let backend = CrosstermBackend::new(stdout);
    let mut terminal = Terminal::new(backend)?;

    //Init input state
    let mut input = String::new();

    loop {
        terminal.draw(|f| {
            let size = f.area();

            //Create a centered Layout
            let chunks = Layout::default()
                .direction(Direction::Vertical)
                .constraints([
                    Constraint::Percentage(40),   //Top Padding
                    Constraint::Length(3),        //Input field Height
                    Constraint::Percentage(40),  //Bottom Padding
                ])
                .split(size);

            let input_area = Layout::default()
                .direction(Direction::Horizontal)
                .constraints([
                    Constraint::Percentage(20),   //Left Padding
                    Constraint::Percentage(60),   // Input field width
                    Constraint::Percentage(20),   //Right Padding
                ])
                .split(chunks[1])[1];

            //Create Input field

            let block = Block::default()
                .borders(Borders::ALL)
                .style(Style::default().fg(Color::White))
                .title("Expression");

            let paragraph = Paragraph::new(Text::from(input.as_str()))
                .block(block)
                .style(Style::default().fg(Color::White));

            f.render_widget(paragraph, input_area);

        })?;

        //Handle Input

        if let Event::Key(key) = event::read()? {
            match key.code {
                KeyCode::Char(c) => input.push(c),
                KeyCode::Backspace => { input.pop(); },
                KeyCode::Enter => break,
                _ => {}
            }
        }
    }

    //Clean up the terminal
    disable_raw_mode()?;
    execute!(terminal.backend_mut(), LeaveAlternateScreen)?;
    terminal.show_cursor()?;

    Ok(input)
}


fn main() -> Result<(), io::Error> {
    println!("Enter the Expression : ");
    let input = expr_input()?;
    let expr = parse_expr(input.trim());
    truth_table(&expr);
    Ok(())
}
