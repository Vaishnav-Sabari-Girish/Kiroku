mod expr;
mod parser;
mod eval;
mod truth_table;

use ratatui::{
    backend::CrosstermBackend,
    layout::{Constraint, Direction, Layout},
    style::{Color, Style},
    text::{Line, Text},
    widgets::{Block, Borders, Paragraph, Tabs},
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

fn show_tabs(expr_str: &str) -> Result<(), io::Error> {
    let expression = parse_expr(expr_str.trim());
    let table_str = truth_table(&expression);

    enable_raw_mode()?;

    let mut stdout = io::stdout();
    execute!(stdout, EnterAlternateScreen)?;
    let backend = CrosstermBackend::new(stdout);
    let mut terminal = Terminal::new(backend)?;

    let tabs = ["Truth Table", "K-Map", "Circuit"];
    let mut active_tab = 0;
    let mut scroll: u16 = 0;     //For scrolling

    loop {
        terminal.draw(|f| {
            let size = f.area();
            let chunks = Layout::default()
                .direction(Direction::Vertical)
                .constraints([
                    Constraint::Length(3),    //Tabs
                    Constraint::Min(0),       //Content
                ])
                .split(size);

            let titles: Vec<Line> = tabs.iter().map(|t| Line::from(*t)).collect();
            let tabs_widget = Tabs::new(titles)
                .block(Block::default().borders(Borders::ALL).title("Menu"))
                .highlight_style(Style::default().fg(Color::Yellow))
                .select(active_tab);

            f.render_widget(tabs_widget, chunks[0]);

            let content = match  active_tab {
                0 => Paragraph::new(table_str.clone())
                    .block(Block::default().borders(Borders::ALL).title("Truth Table"))
                    .alignment(ratatui::layout::Alignment::Center)
                    .scroll((scroll, 0)),
                1 => Paragraph::new("K-Map (Coming Soon)")
                    .block(Block::default().borders(Borders::ALL).title("K-Map")),
                2 => Paragraph::new("Circuit (Coming Soon)")
                    .block(Block::default().borders(Borders::ALL).title("Circuit")),
                _ => Paragraph::new(""),
            };

            f.render_widget(content, chunks[1]);
        })?;

        if let  Event::Key(key) = event::read()? {
            match key.code {
                KeyCode::Left => {
                    if active_tab > 0 { active_tab -= 1; }
                }
                KeyCode::Right => {
                    if  active_tab < tabs.len() - 1 {
                        active_tab += 1;
                    }
                }
                KeyCode::Up => if scroll > 0 {
                    scroll -= 1;
                },
                KeyCode::Down => scroll += 1,

                KeyCode::Esc => break,
                _ => {}
            }
        } 
    }

    disable_raw_mode()?;
    execute!(terminal.backend_mut(), LeaveAlternateScreen)?;
    terminal.show_cursor()?;

    Ok(())
}

fn main() -> Result<(), io::Error> {
    let input = expr_input()?;
    show_tabs(&input)?;
    Ok(())
}
