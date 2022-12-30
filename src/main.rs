use std::fs;

use crossterm::{
    event::{self, DisableMouseCapture, Event, KeyCode},
    execute,
    terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
};

use std::{
    error::Error,
    io,
    time::{Duration, Instant},
};
use tui::{
    backend::{Backend, CrosstermBackend},
    layout::{Alignment, Constraint, Direction, Layout, Rect},
    style::{Color, Modifier, Style},
    text::{Span, Spans},
    widgets::{Block, Borders, Cell, Clear, Paragraph, Row, Table, Tabs, Wrap},
    Frame, Terminal,
};

pub mod configuration;

use configuration::Configuration;
use configuration::Timer;

struct InputField {
    content: String,
}

impl InputField {
    fn new() -> Self {
        Self {
            content: String::new(),
        }
    }
}

fn main() -> Result<(), Box<dyn Error>> {
    enable_raw_mode()?;
    let mut stdout = io::stdout();
    execute!(stdout, EnterAlternateScreen, DisableMouseCapture)?;
    let backend = CrosstermBackend::new(stdout);
    let mut terminal = Terminal::new(backend)?;

    let tick_rate = Duration::from_millis(1000);
    let res = run_app(&mut terminal, tick_rate);
    disable_raw_mode()?;

    execute!(
        terminal.backend_mut(),
        LeaveAlternateScreen,
        DisableMouseCapture
    )?;
    terminal.show_cursor()?;

    if let Err(err) = res {
        println!("{:?}", err)
    }

    Ok(())
}

fn parse_input(input: &String, config: &mut Configuration) {
    if input.is_empty() {
        return;
    }
    let mut routine = String::new();
    let mut argument1 = String::new();
    let mut argument2 = String::new();
    let mut routine_flag = true;
    let mut i = 0;
    for c in input.chars() {
        if routine_flag && c.is_whitespace() {
            routine_flag = false;
        } else if c.is_whitespace() {
            i += 1;
            break;
        } else {
            if routine_flag {
                routine.push(c);
            } else {
                argument1.push(c);
            }
        }
        i += 1;
    }

    match routine.as_str() {
        "add" => {
            argument2.push_str(&input[i..].to_string());
            let hours: u16;
            let minutes: u16;
            let seconds: u16;
            if argument1.len() == 8 {
                hours = argument1[0..2].parse::<u16>().unwrap_or_default();
                minutes = argument1[3..5].parse::<u16>().unwrap_or_default();
                seconds = argument1[6..8].parse::<u16>().unwrap_or_default();
            } else {
                let min_entered = argument1[..].parse::<u16>().unwrap_or_default();
                hours = min_entered / 60;
                minutes = min_entered % 60;
                seconds = 0;
            }

            config
                .timers
                .push(Timer::new(argument2, hours, minutes, seconds, true));
        }
        "add2" => {
            argument2.push_str(&input[i..].to_string());
            let hours: u16;
            let minutes: u16;
            let seconds: u16;
            if argument1.len() == 8 {
                hours = argument1[0..2].parse::<u16>().unwrap_or_default();
                minutes = argument1[3..5].parse::<u16>().unwrap_or_default();
                seconds = argument1[6..8].parse::<u16>().unwrap_or_default();
            } else {
                let min_entered = argument1[..].parse::<u16>().unwrap_or_default();
                hours = min_entered / 60;
                minutes = min_entered % 60;
                seconds = 0;
            }

            config
                .timers
                .push(Timer::new(argument2, hours, minutes, seconds, false));
        }
        "addp" => {
            config.timers.push(Timer::new(
                "Pomodoro-Timer".to_string(),
                0,
                config.pomodoro_time,
                0,
                true,
            ));
            config.timers.push(Timer::new(
                "Pomodoro-Break".to_string(),
                0,
                if !config.timers.is_empty() && config.timers.len() % 6 == 0 {
                    config.pomodoro_bigbreak
                } else {
                    config.pomodoro_smallbreak
                },
                0,
                true,
            ));
        }
        "rm" => {
            let id = argument1[..].parse::<u16>().unwrap();
            for (i, t) in config.timers.iter().enumerate() {
                if t.id == id {
                    config.timers.remove(i);
                    break;
                }
            }
        }
        "clear" => {
            config.timers.clear();
        }
        "move" => {
            let id = argument1[..].parse::<usize>().unwrap();
            argument2.push_str(&input[i..].to_string());
            let id2 = argument2[..].parse::<usize>().unwrap();
            let t = config.timers.remove(id);
            config.timers.insert(id2, t);
        }
        "moveup" => {
            let id = argument1[..].parse::<usize>().unwrap();
            config.timers.swap(id, id - 1);
        }
        "movedown" => {
            let id = argument1[..].parse::<usize>().unwrap();
            config.timers.swap(id, id + 1);
        }
        "plus" => {
            let id = argument1[..].parse::<u16>().unwrap();
            argument2.push_str(&input[i..].to_string());
            let min = argument2[..].parse::<u16>().unwrap();
            for t in &mut config.timers {
                if t.id == id {
                    if t.minutes + min > 59 {
                        t.hours += (t.minutes + min) / 60;
                        t.minutes = (t.minutes + min) % 60;
                    } else {
                        t.minutes += min;
                    }
                    break;
                }
            }
        }
        "minus" => {
            let id = argument1[..].parse::<u16>().unwrap();
            argument2.push_str(&input[i..].to_string());
            let min = argument2[..].parse::<u16>().unwrap();
            for t in &mut config.timers {
                if t.id == id {
                    if t.minutes < min {
                        let diff = min - t.minutes;
                        t.hours -= diff / 60 + 1;
                        t.minutes = 60 - (diff % 60);
                    } else {
                        t.minutes -= min;
                    }
                    break;
                }
            }
        }
        _ => {}
    }
    config.write_to_file().unwrap();
    configuration::update_timers(&mut config.timers);
}

fn run_app<B: Backend>(terminal: &mut Terminal<B>, tick_rate: Duration) -> io::Result<()> {
    let mut last_tick = Instant::now();
    let mut input_field = InputField::new();
    let data = fs::read_to_string("config.json");
    let mut config: Configuration = match data {
        Ok(data) => serde_json::from_str(&data).unwrap_or_else(|_| Configuration::new(25, 5, 10)),
        Err(_) => Configuration::new(25, 5, 10),
    };
    config.titles = vec!["Timer [1]", "Config [2]"];
    configuration::update_timers(&mut config.timers);

    let mut pause_flag: bool = false;

    let mut i = 0;
    loop {
        if last_tick.elapsed() >= tick_rate {
            last_tick = Instant::now();
            if !pause_flag {
                let mut left_view_done = false;
                let mut right_view_done = false;
                for timer in &mut config.timers {
                    if timer.left_view
                        && !left_view_done
                        && (timer.seconds != 0 || timer.minutes != 0 || timer.hours != 0)
                    {
                        timer.tick();
                        left_view_done = true;
                    } else if !timer.left_view
                        && !right_view_done
                        && (timer.seconds != 0 || timer.minutes != 0 || timer.hours != 0)
                    {
                        timer.tick();
                        right_view_done = true;
                    }
                }
            } else {
                configuration::update_timers(&mut config.timers);
            }

            terminal.draw(|f| ui(f, &mut config, &input_field))?;

            if i % 30 == 0 {
                config.write_to_file().unwrap();
            }
        }

        let timeout = tick_rate
            .checked_sub(last_tick.elapsed())
            .unwrap_or_else(|| Duration::from_secs(0));
        if config.index == 0 && crossterm::event::poll(timeout)? {
            if let Event::Key(key) = event::read()? {
                if input_field.content.is_empty() && KeyCode::Char('q') == key.code {
                    return Ok(());
                } else if input_field.content.is_empty() && KeyCode::Char('h') == key.code {
                    config.show_popup = !config.show_popup;
                } else if input_field.content.is_empty() && KeyCode::Char(' ') == key.code {
                    pause_flag = !pause_flag;
                } else if KeyCode::Esc == key.code {
                    input_field.content.clear();
                } else if let KeyCode::Enter = key.code {
                    parse_input(&input_field.content, &mut config);
                    input_field.content.clear();
                } else if KeyCode::Right == key.code || KeyCode::Tab == key.code {
                    config.next();
                } else if KeyCode::Left == key.code {
                    config.previous()
                } else if let KeyCode::Char(c) = key.code {
                    input_field.content.push(c);
                } else if let KeyCode::Backspace = key.code {
                    input_field.content.pop();
                }
                terminal.draw(|f| ui(f, &mut config, &input_field))?;
            }
        } else if config.index == 1 && crossterm::event::poll(timeout)? {
            if let Event::Key(key) = event::read()? {
                if input_field.content.is_empty() && KeyCode::Char('q') == key.code {
                    return Ok(());
                } else if KeyCode::Esc == key.code {
                    config.clear_table_entry();
                } else if let KeyCode::Enter = key.code {
                    config.save_table_changes();
                } else if KeyCode::Right == key.code || KeyCode::Tab == key.code {
                    config.next();
                } else if KeyCode::Left == key.code {
                    config.previous()
                } else if let KeyCode::Char(c) = key.code {
                    config.write_table_entry(c);
                } else if let KeyCode::Backspace = key.code {
                    config.pop_table_entry();
                } else if let KeyCode::Up = key.code {
                    config.previous_table_entry()
                } else if let KeyCode::Down = key.code {
                    config.next_table_entry();
                }
                terminal.draw(|f| ui(f, &mut config, &input_field))?;
            }
        }
        i += 1;
    }
}

fn ui<B: Backend>(f: &mut Frame<B>, config: &mut Configuration, input_field: &InputField) {
    let mut size = f.size();
    let len_right_view_timers = configuration::num_rightview_timers(&config.timers);
    let len_left_view_timers = config.timers.len() - len_right_view_timers;
    let left_view_timers: Vec<&Timer> = config
        .timers
        .iter()
        .filter(|t| t.left_view == true)
        .collect();
    let right_view_timers: Vec<&Timer> = config
        .timers
        .iter()
        .filter(|t| t.left_view == false)
        .collect();
    let mut constraints_vec = Vec::new();
    let mut constraints_vec2 = Vec::new();
    constraints_vec.push(Constraint::Percentage(3));
    for _ in 0..len_left_view_timers {
        constraints_vec.push(Constraint::Percentage(
            (92.0 / len_left_view_timers as f32) as u16,
        ));
    }
    constraints_vec.push(Constraint::Percentage(5));

    if len_right_view_timers > 0 {
        constraints_vec2.push(Constraint::Percentage(3));
        for _ in 0..len_right_view_timers {
            constraints_vec2.push(Constraint::Percentage(
                (92.0 / len_right_view_timers as f32) as u16,
            ));
        }
        constraints_vec2.push(Constraint::Percentage(5));
    }

    let titles = config
        .titles
        .iter()
        .map(|t| {
            let (first, rest) = t.split_at(1);
            Spans::from(vec![
                Span::styled(first, Style::default().fg(Color::Gray)),
                Span::styled(rest, Style::default().fg(Color::Gray)),
            ])
        })
        .collect();
    let tabs = Tabs::new(titles)
        .select(config.index)
        .style(Style::default().fg(Color::Gray))
        .highlight_style(
            Style::default()
                .add_modifier(Modifier::BOLD)
                .bg(Color::Green),
        );

    let chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints(constraints_vec)
        .split(size);
    f.render_widget(tabs, chunks[0]);
    let mut chunks2: Vec<Rect> = Vec::new();
    if len_right_view_timers > 0 {
        size.width = size.width / 2;
        size.x = size.width;
        chunks2 = Layout::default()
            .direction(Direction::Vertical)
            .constraints(constraints_vec2)
            .split(size);
        size.width = size.width * 2;
        size.x = 0;
    }

    if config.index == 0 {
        /* loop for timers */
        for i in 1..chunks.len() - 1 {
            let paragraph = Paragraph::new(left_view_timers[i - 1].formatted())
                .block(Block::default().borders(
                    Borders::TOP
                        | if len_right_view_timers > 0 {
                            Borders::RIGHT
                        } else {
                            Borders::NONE
                        },
                ))
                .style(Style::default().fg(if left_view_timers[i - 1].is_active {
                    Color::LightCyan
                } else {
                    Color::DarkGray
                }));
            f.render_widget(paragraph, chunks[i]);
        }
        if len_right_view_timers > 0 {
            /* loop for timers2 */
            for i in 1..chunks2.len() - 1 {
                let paragraph = Paragraph::new(right_view_timers[i - 1].formatted())
                    .block(Block::default().borders(Borders::TOP | Borders::LEFT))
                    .style(Style::default().fg(if right_view_timers[i - 1].is_active {
                        Color::LightCyan
                    } else {
                        Color::DarkGray
                    }));
                f.render_widget(paragraph, chunks2[i]);
            }
        }
        let input = Paragraph::new(input_field.content.as_ref())
            .style(Style::default().fg(Color::Yellow))
            .block(Block::default().borders(Borders::ALL).title("Input"));
        f.render_widget(input, chunks[chunks.len() - 1]);

        let text = if !input_field.content.is_empty() {
            "Press <ESC> to clear the input field"
        } else if config.show_popup {
            "Press <SPACE> to pause the timers; Press h to close the help-popup; Press q to quit the application"
        } else {
            "Press <SPACE> to pause the timers; Press h to show the help-popup; Press q to quit the application"
        };

        let paragraph = Paragraph::new(Span::styled(
            text,
            Style::default().add_modifier(Modifier::ITALIC),
        ))
        .alignment(Alignment::Center)
        .wrap(Wrap { trim: true });
        f.render_widget(paragraph, chunks[chunks.len() - 1]);

        if config.show_popup {
            let helptext =
                fs::read_to_string("helptext.txt").expect("Unable to read helptext file");
            let paragraph = Paragraph::new(helptext)
                .block(Block::default().borders(Borders::ALL))
                .style(Style::default().fg(Color::LightRed));
            let area = centered_rect(80, 50, size);
            f.render_widget(Clear, area); //this clears out the background
            f.render_widget(paragraph, area);
        }
    } else if config.index == 1 {
        let selected_style = Style::default().add_modifier(Modifier::REVERSED);
        let normal_style = Style::default().bg(Color::Green);
        let header_cells = ["Configuration", "Value"]
            .iter()
            .map(|h| Cell::from(*h).style(Style::default().fg(Color::Black)));
        let header = Row::new(header_cells)
            .style(normal_style)
            .height(1)
            .bottom_margin(1);
        if config.state.selected() == None {
            config.pomodoro_time_table_str = config.pomodoro_time.to_string();
            config.pomodoro_smallbreak_table_str = config.pomodoro_smallbreak.to_string();
            config.pomodoro_bigbreak_table_str = config.pomodoro_bigbreak.to_string();
        }
        let items = vec![
            vec![
                "pomodoro_time".to_string(),
                config.pomodoro_time_table_str.to_owned(),
            ],
            vec![
                "pomodoro_smallbreak".to_string(),
                config.pomodoro_smallbreak_table_str.to_owned(),
            ],
            vec![
                "pomodoro_bigbreak".to_string(),
                config.pomodoro_bigbreak_table_str.to_owned(),
            ],
        ];

        let rows = items.iter().map(|item| {
            let height = item
                .iter()
                .map(|content| content.chars().filter(|c| *c == '\n').count())
                .max()
                .unwrap_or(0)
                + 1;
            let cells = item.iter().map(|c| Cell::from(&c[..]));
            Row::new(cells).height(height as u16).bottom_margin(1)
        });
        let t = Table::new(rows)
            .header(header)
            .block(Block::default().borders(Borders::ALL))
            .highlight_style(selected_style)
            .highlight_symbol(">> ")
            .widths(&[
                Constraint::Percentage(50),
                Constraint::Length(30),
                Constraint::Min(10),
            ]);
        f.render_stateful_widget(t, chunks[1], &mut config.state);
        //* */
        let text = "Press <ENTER> to save the configuration";

        let paragraph = Paragraph::new(Span::styled(
            text,
            Style::default()
                .add_modifier(Modifier::ITALIC)
                .fg(Color::Yellow),
        ))
        .alignment(Alignment::Center)
        .wrap(Wrap { trim: true });
        f.render_widget(paragraph, chunks[chunks.len() - 1]);
    }
}

/// helper function to create a centered rect using up certain percentage of the available rect `r`
fn centered_rect(percent_x: u16, percent_y: u16, r: Rect) -> Rect {
    let popup_layout = Layout::default()
        .direction(Direction::Vertical)
        .constraints(
            [
                Constraint::Percentage((100 - percent_y) / 2),
                Constraint::Percentage(percent_y),
                Constraint::Percentage((100 - percent_y) / 2),
            ]
            .as_ref(),
        )
        .split(r);

    Layout::default()
        .direction(Direction::Horizontal)
        .constraints(
            [
                Constraint::Percentage((100 - percent_x) / 2),
                Constraint::Percentage(percent_x),
                Constraint::Percentage((100 - percent_x) / 2),
            ]
            .as_ref(),
        )
        .split(popup_layout[1])[1]
}
