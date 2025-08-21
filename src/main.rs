mod expr;
mod parser;
mod eval;
mod truth_table;
mod k_map;
mod logic_gates;

use ratatui::{
    backend::CrosstermBackend,
    layout::{Constraint, Direction, Layout},
    style::{Color, Style},
    text::{Line, Text},
    widgets::{Block, Borders, Paragraph, Tabs},
    Terminal,
};
use crossterm::{
    event::{self, Event, KeyCode, MouseButton, MouseEventKind},
    terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
    execute,
};
use parser::parse_expr;
use truth_table::truth_table;
use k_map::k_map;
use logic_gates::LogicGatesViewer;
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
                    Constraint::Percentage(30),   //Top Padding
                    Constraint::Length(3),        //Input field Height
                    Constraint::Length(6),        //Instructions
                    Constraint::Percentage(30),   //Bottom Padding
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
                .title("Boolean Expression");

            let paragraph = Paragraph::new(Text::from(input.as_str()))
                .block(block)
                .style(Style::default().fg(Color::White));

            f.render_widget(paragraph, input_area);

            // Instructions
            let instructions = Paragraph::new(Text::from(vec![
                Line::from("Enter a boolean expression using:"),
                Line::from("Variables: A, B, C, etc."),
                Line::from("Operators: & (AND), | (OR), ! (NOT), ^ (XOR)"),
                Line::from("Parentheses: ( ) for grouping"),
                Line::from("Example: A & B | !C"),
                Line::from("Press Enter to continue, Esc to exit"),
            ]))
            .block(Block::default().borders(Borders::ALL).title("Instructions"))
            .style(Style::default().fg(Color::Gray));
            
            f.render_widget(instructions, chunks[2]);

        })?;

        //Handle Input
        if let Event::Key(key) = event::read()? {
            match key.code {
                KeyCode::Char(c) => input.push(c),
                KeyCode::Backspace => { input.pop(); },
                KeyCode::Enter => break,
                KeyCode::Esc => {
                    // Clean up and exit
                    disable_raw_mode()?;
                    execute!(terminal.backend_mut(), LeaveAlternateScreen)?;
                    terminal.show_cursor()?;
                    std::process::exit(0);
                }
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

    let tabs = ["Truth Table", "K-Map", "Logic Circuit"];
    let mut active_tab = 0;
    let mut scroll: u16 = 0;     //For scrolling
    let mut logic_gates_viewer = LogicGatesViewer::new();
    
    // Set the expression for the logic gates viewer
    logic_gates_viewer.set_expression(expression.clone());

    loop {
        terminal.draw(|f| {
            let size = f.area();
            let chunks = Layout::default()
                .direction(Direction::Vertical)
                .constraints([
                    Constraint::Length(3),    //Tabs
                    Constraint::Length(2),    //Expression display
                    Constraint::Min(0),       //Content
                ])
                .split(size);

            let titles: Vec<Line> = tabs.iter().map(|t| Line::from(*t)).collect();
            let tabs_widget = Tabs::new(titles)
                .block(Block::default().borders(Borders::ALL).title("Boolean Logic Analyzer"))
                .highlight_style(Style::default().fg(Color::Yellow))
                .select(active_tab);

            f.render_widget(tabs_widget, chunks[0]);

            // Display the input expression
            let expr_display = Paragraph::new(Text::from(format!("Expression: {}", expr_str)))
                .block(Block::default().borders(Borders::ALL))
                .style(Style::default().fg(Color::Cyan));
            f.render_widget(expr_display, chunks[1]);

            match active_tab {
                0 => {
                    let content = Paragraph::new(table_str.clone())
                        .block(Block::default().borders(Borders::ALL).title("Truth Table"))
                        .alignment(ratatui::layout::Alignment::Center)
                        .scroll((scroll, 0));
                    f.render_widget(content, chunks[2]);
                }
                1 => {
                    let content = Paragraph::new(k_map(&expression))
                        .block(Block::default().borders(Borders::ALL).title("K-Map"))
                        .alignment(ratatui::layout::Alignment::Center);
                    f.render_widget(content, chunks[2]);
                }
                2 => {
                    logic_gates_viewer.render(f, chunks[2]);
                }
                _ => {}
            }
        })?;

        if let Event::Key(key) = event::read()? {
            match key.code {
                KeyCode::Left => {
                    if active_tab > 0 { active_tab -= 1; }
                }
                KeyCode::Right => {
                    if active_tab < tabs.len() - 1 {
                        active_tab += 1;
                    }
                }
                KeyCode::Up => {
                    if active_tab == 0 && scroll > 0 {
                        scroll -= 1;
                    }
                }
                KeyCode::Down => {
                    if active_tab == 0 {
                        scroll += 1;
                    }
                }
                KeyCode::Char('r') => {
                    if active_tab == 2 {
                        // Reset pan and zoom
                        logic_gates_viewer.pan_x = 0.0;
                        logic_gates_viewer.pan_y = 0.0;
                        logic_gates_viewer.zoom = 1.0;
                    }
                }
                KeyCode::Char('w') => {
                    if active_tab == 2 {
                        logic_gates_viewer.pan(0.0, -5.0);
                    }
                }
                KeyCode::Char('s') => {
                    if active_tab == 2 {
                        logic_gates_viewer.pan(0.0, 5.0);
                    }
                }
                KeyCode::Char('a') => {
                    if active_tab == 2 {
                        logic_gates_viewer.pan(-5.0, 0.0);
                    }
                }
                KeyCode::Char('d') => {
                    if active_tab == 2 {
                        logic_gates_viewer.pan(5.0, 0.0);
                    }
                }
                KeyCode::Char('+') | KeyCode::Char('=') => {
                    if active_tab == 2 {
                        logic_gates_viewer.zoom_in();
                    }
                }
                KeyCode::Char('-') => {
                    if active_tab == 2 {
                        logic_gates_viewer.zoom_out();
                    }
                }
                KeyCode::Esc => break,
                _ => {}
            }
        }
        
        // Handle mouse events for Logic Gates tab
        if let Event::Mouse(mouse_event) = event::read()? {
            if active_tab == 2 {
                match mouse_event.kind {
                    MouseEventKind::ScrollUp => {
                        logic_gates_viewer.zoom_in();
                    }
                    MouseEventKind::ScrollDown => {
                        logic_gates_viewer.zoom_out();
                    }
                    _ => {}
                }
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
    
    if input.trim().is_empty() {
        println!("No expression entered. Exiting.");
        return Ok(());
    }
    
    // Validate the expression before proceeding
    match std::panic::catch_unwind(|| parse_expr(input.trim())) {
        Ok(_) => {
            show_tabs(&input)?;
            println!("Expression: {}", input);
        }
        Err(_) => {
            println!("Error: Invalid boolean expression format.");
            println!("Please use variables (A, B, C, etc.) and operators (&, |, !, ^)");
            println!("Example: A & B | !C");
        }
    }
    
    Ok(())
}
