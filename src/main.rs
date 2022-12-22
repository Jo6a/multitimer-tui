use chrono::{DateTime, Local};
use std::fs;

use crossterm::{
    event::{self, DisableMouseCapture, Event, KeyCode},
    execute,
    terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
};
use serde::{Deserialize, Serialize};
use std::process::Command;
use std::{
    borrow::Borrow,
    error::Error,
    io,
    time::{Duration, Instant},
};
use tui::{
    backend::{Backend, CrosstermBackend},
    layout::{Alignment, Constraint, Direction, Layout, Rect},
    style::{Color, Modifier, Style},
    text::{Span, Spans},
    widgets::{Block, Borders, Clear, Paragraph, Wrap},
    Frame, Terminal,
};

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

#[derive(Serialize, Deserialize)]
struct Configuration {
    pomodoro_time: u16,
    pomodoro_smallbreak: u16,
    pomodoro_bigbreak: u16,
    timers: Vec<Timer>,
    #[serde(skip_serializing, skip_deserializing)]
    show_popup: bool,
}

impl Configuration {
    fn new(pomodoro_time: u16, pomodoro_smallbreak: u16, pomodoro_bigbreak: u16) -> Self {
        Self {
            pomodoro_time,
            pomodoro_smallbreak,
            pomodoro_bigbreak,
            timers: Vec::new(),
            show_popup: false,
        }
    }

    fn write_to_file(&self) -> Result<(), std::io::Error> {
        std::fs::write("config.json", serde_json::to_string_pretty(self).unwrap())
    }
}

#[derive(Serialize, Deserialize)]
struct Timer {
    #[serde(skip_serializing, skip_deserializing)]
    id: u16,
    #[serde(skip_serializing, skip_deserializing)]
    is_active: bool,
    description: String,
    hours: u16,
    minutes: u16,
    seconds: u16,
    endtime: DateTime<Local>,
}

impl Timer {
    fn new(description: String, hours: u16, minutes: u16, seconds: u16) -> Self {
        Self {
            id: 0,
            is_active: false,
            description,
            hours,
            minutes,
            seconds,
            endtime: Local::now(),
        }
    }

    fn formatted(&self) -> String {
        format!(
            "{:02}:{:02}:{:02} ({})         @{}:{}",
            self.hours,
            self.minutes,
            self.seconds,
            self.endtime.format("%Y-%m-%d %H:%M:%S"),
            self.id.to_string(),
            self.description
        )
    }

    fn tick(&mut self) {
        self.is_active = true;
        if self.seconds > 0 {
            self.seconds -= 1;
        } else if self.minutes > 0 {
            self.minutes -= 1;
            self.seconds = 59;
        } else if self.hours > 0 {
            self.hours -= 1;
            self.minutes = 59;
            self.seconds = 59;
        }

        if self.seconds == 0 && self.minutes == 0 && self.hours == 0 {
            Command::new("bash")
                .args(&["-c", "echo -e \"\\a\" "])
                .spawn()
                .expect("Playing sound failed");
            self.is_active = false;
        }
    }
}
fn update_timers(timers: &mut Vec<Timer>) {
    let mut dt = Local::now();
    for (i, timer) in timers.into_iter().enumerate() {
        if timer.seconds != 0 || timer.minutes != 0 || timer.hours != 0 {
            dt += chrono::Duration::seconds(timer.seconds as i64)
                + chrono::Duration::minutes(timer.minutes as i64)
                + chrono::Duration::hours(timer.hours as i64);

            timer.endtime = dt;
            timer.id = i as u16;
            timer.is_active = false;
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

fn parse_input(input: &String, config: &mut Configuration, pause_flag: &mut bool) {
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
                .push(Timer::new(argument2, hours, minutes, seconds));
        }
        "addp" => {
            config.timers.push(Timer::new(
                "Pomodoro-Timer".to_string(),
                0,
                config.pomodoro_time,
                0,
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
    config.write_to_file();
    update_timers(&mut config.timers);
}

fn run_app<B: Backend>(terminal: &mut Terminal<B>, tick_rate: Duration) -> io::Result<()> {
    let mut last_tick = Instant::now();
    let mut input_field = InputField::new();
    let data = fs::read_to_string("config.json");
    let mut config: Configuration = match data {
        Ok(data) => serde_json::from_str(&data).unwrap_or_else(|_| Configuration::new(25, 5, 10)),
        Err(_) => Configuration::new(25, 5, 10),
    };
    update_timers(&mut config.timers);
    let mut pause_flag: bool = false;

    let mut i = 0;
    loop {
        if last_tick.elapsed() >= tick_rate {
            last_tick = Instant::now();
            if !pause_flag {
                for timer in &mut config.timers {
                    if timer.seconds != 0 || timer.minutes != 0 || timer.hours != 0 {
                        timer.tick();
                        break;
                    }
                }
            } else {
                update_timers(&mut config.timers);
            }

            terminal.draw(|f| ui(f, &config, &input_field))?;

            if i % 30 == 0 {
                config.write_to_file();
            }
        }

        let timeout = tick_rate
            .checked_sub(last_tick.elapsed())
            .unwrap_or_else(|| Duration::from_secs(0));
        if crossterm::event::poll(timeout)? {
            if let Event::Key(key) = event::read()? {
                if input_field.content.is_empty() && KeyCode::Char('q') == key.code {
                    return Ok(());
                } else if input_field.content.is_empty() && KeyCode::Char('h') == key.code {
                    config.show_popup = !config.show_popup;
                } else if input_field.content.is_empty() && KeyCode::Char('p') == key.code {
                    if pause_flag == false {
                        pause_flag = true;
                    } else {
                        pause_flag = false;
                    }
                } else if KeyCode::Esc == key.code {
                    input_field.content.clear();
                } else if let KeyCode::Enter = key.code {
                    parse_input(&input_field.content, &mut config, &mut pause_flag);
                    input_field.content.clear();
                } else if let KeyCode::Char(c) = key.code {
                    input_field.content.push(c);
                } else if let KeyCode::Backspace = key.code {
                    input_field.content.pop();
                }
                terminal.draw(|f| ui(f, &config, &input_field))?;
            }
        }
        i += 1;
    }
}

fn ui<B: Backend>(f: &mut Frame<B>, config: &Configuration, input_field: &InputField) {
    let size = f.size();
    let mut constraints_vec = Vec::new();
    for _ in 0..config.timers.len() {
        constraints_vec.push(Constraint::Percentage(
            (85.0 / config.timers.len() as f32) as u16,
        ));
    }
    constraints_vec.push(Constraint::Percentage(0));
    constraints_vec.push(Constraint::Percentage(15));
    let chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints(constraints_vec)
        .split(size);

    for i in 0..chunks.len() - 2 {
        if config.timers[i].is_active == true {
            let paragraph = Paragraph::new(config.timers[i].formatted())
                .block(Block::default().borders(Borders::TOP))
                .style(Style::default().fg(Color::LightCyan));
            f.render_widget(paragraph, chunks[i]);
        } else {
            let paragraph = Paragraph::new(config.timers[i].formatted())
                .block(Block::default().borders(Borders::TOP))
                .style(Style::default().fg(Color::DarkGray));
            f.render_widget(paragraph, chunks[i]);
        }
    }
    let input = Paragraph::new(input_field.content.as_ref())
        .style(Style::default().fg(Color::Yellow))
        .block(Block::default().borders(Borders::ALL).title("Input"));
    f.render_widget(input, chunks[chunks.len() - 1]);

    let text = if !input_field.content.is_empty() {
        "Press <ESC> to clear the input field"
    } else if config.show_popup {
        "Press h to close the help-popup; Press q to exit the application; Press p to pause the timers"
    } else {
        "Press h to show the help-popup; Press q to exit the application; Press p to pause the timers"
    };



    let paragraph = Paragraph::new(Span::styled(
        text,
        Style::default().add_modifier(Modifier::ITALIC),
    ))
    .alignment(Alignment::Center)
    .wrap(Wrap { trim: true });
    f.render_widget(paragraph, chunks[chunks.len() - 1]);

    if config.show_popup {
        let helptext = fs::read_to_string("helptext.txt").expect("Unable to read helptext file");
        let paragraph = Paragraph::new(helptext)
            .block(Block::default().borders(Borders::ALL))
            .style(Style::default().fg(Color::LightRed));
        let area = centered_rect(80, 50, size);
        f.render_widget(Clear, area); //this clears out the background
        f.render_widget(paragraph, area);
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
