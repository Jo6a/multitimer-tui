use std::fs;

use crossterm::{
    event::{self, DisableMouseCapture, Event, KeyCode, KeyEvent},
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
    cursor_position: usize,
}

impl InputField {
    fn new() -> Self {
        Self {
            content: String::new(),
            cursor_position: 0,
        }
    }

    fn move_cursor_left(&mut self) {
        if self.cursor_position > 0 {
            self.cursor_position -= 1;
        }
    }

    fn move_cursor_right(&mut self) {
        if self.cursor_position < self.content.len() {
            self.cursor_position += 1;
        }
    }

    fn insert_char(&mut self, c: char) {
        self.content.insert(self.cursor_position, c);
        self.move_cursor_right();
    }

    fn delete_char(&mut self) {
        if self.cursor_position > 0 {
            self.content.remove(self.cursor_position - 1);
            self.move_cursor_left();
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

fn parse_input(input: &str, config: &mut Configuration) {
    if input.is_empty() {
        return;
    }
    let mut parts = input.split_whitespace();
    let routine = parts.next().unwrap_or("");
    let argument1 = parts.next().unwrap_or("").to_string();
    let mut argument2: String = parts.collect::<Vec<&str>>().join(" ");

    match routine {
        "add" | "add2" => {
            add_timer(&argument1, &mut argument2, routine, config);
        }
        "addp" => {
            add_pomodoro_timer(config);
        }
        "rm" => {
            remove_timer(&argument1, config);
        }
        "clear" => {
            config.timers.clear();
        }
        "move" => {
            move_timer(&argument1, &argument2, config);
        }
        "moveup" => {
            move_timer_up(&argument1, config);
        }
        "movedown" => {
            move_timer_down(&argument1, config);
        }
        "plus" => {
            increase_timer(&argument1, &argument2, config);
        }
        "minus" => {
            decrease_timer(&argument1, &argument2, config);
        }
        "rename" => {
            rename_timer(argument1, config, argument2);
        }
        _ => {}
    }
    config.write_to_file().unwrap();
    configuration::update_timers(&mut config.timers);
}

fn add_timer(
    argument1: &String,
    argument2: &mut String,
    routine: &str,
    config: &mut Configuration,
) {
    let timer = configuration::create_timer_for_input(argument1, argument2, routine == "add");
    configuration::add_timer_to_config(config, timer);
}

fn add_pomodoro_timer(config: &mut Configuration) {
    let timer1 = Timer::new(
        "Pomodoro-Timer".to_string(),
        config.pomodoro_time * 60,
        true,
    );
    let timer2 = Timer::new(
        "Pomodoro-Break".to_string(),
        if !config.timers.is_empty() && config.timers.len() % 6 == 0 {
            config.pomodoro_bigbreak * 60
        } else {
            config.pomodoro_smallbreak * 60
        },
        true,
    );

    configuration::add_timer_to_config(config, timer1);
    configuration::add_timer_to_config(config, timer2);
}

fn remove_timer(argument1: &String, config: &mut Configuration) {
    if let Ok(id) = argument1.parse::<u16>() {
        config.timers.retain(|t| t.id != id);
    }
}

fn move_timer(argument1: &String, argument2: &String, config: &mut Configuration) {
    if let (Ok(id), Ok(id2)) = (
        argument1[..].parse::<usize>(),
        argument2[..].parse::<usize>(),
    ) {
        let t = config.timers.remove(id);
        config.timers.insert(id2, t);
    }
}

fn move_timer_up(argument1: &String, config: &mut Configuration) {
    let id = argument1[..].parse::<usize>().unwrap();
    config.timers.swap(id, id - 1);
}

fn move_timer_down(argument1: &String, config: &mut Configuration) {
    let id = argument1[..].parse::<usize>().unwrap();
    config.timers.swap(id, id + 1);
}

fn increase_timer(argument1: &String, argument2: &String, config: &mut Configuration) {
    let id = argument1[..].parse::<u16>().unwrap();
    let min = argument2[..].parse::<u16>().unwrap();
    for t in &mut config.timers {
        if t.id == id {
            t.timeleft_secs += min * 60;
            break;
        }
    }
}

fn decrease_timer(argument1: &String, argument2: &String, config: &mut Configuration) {
    let id = argument1[..].parse::<u16>().unwrap();
    let min = argument2[..].parse::<u16>().unwrap();
    for t in &mut config.timers {
        if t.id == id {
            if t.timeleft_secs < min * 60 {
                t.timeleft_secs = 0;
            } else {
                t.timeleft_secs -= min * 60;
            }
            break;
        }
    }
}

fn rename_timer(argument1: String, config: &mut Configuration, argument2: String) {
    if let Ok(id) = argument1.parse::<u16>() {
        if let Some(timer) = config.timers.iter_mut().find(|t| t.id == id) {
            timer.description = argument2;
        }
    }
}
fn draw_input_field<B>(terminal: &mut Terminal<B>, input_field: &InputField)
where
    B: Backend,
{
    //let cursor_style = Style::default().fg(Color::Yellow).modifier(Modifier::BOLD);
    let cursor_style = Style::default().fg(Color::Yellow);
    let normal_style = Style::default().fg(Color::White);

    let cursor_pos = input_field.cursor_position;
    let (left, right) = input_field.content.split_at(cursor_pos);

    let text = format!("{}{}{}", left, "█", right);

    let paragraph = Paragraph::new(text.as_ref())
        .style(normal_style)
        .block(Block::default().borders(Borders::ALL));

    terminal.draw(|f| {
        let size = f.size();
        let chunks = Layout::default()
            .direction(Direction::Horizontal)
            .constraints([Constraint::Percentage(100)].as_ref())
            .split(size);

        f.render_widget(paragraph, chunks[0]);
        f.set_cursor(
            // Put cursor past the end of the input text
            chunks[0].x + cursor_pos as u16 + 1,
            // Move one line down, from the border to the input line
            chunks[0].y + 1,
        );
        //f.set_style(cursor_style);
        //f.write_str("█")?;
        //f.set_style(normal_style);
    });
}

fn run_app<B: Backend>(terminal: &mut Terminal<B>, tick_rate: Duration) -> io::Result<()> {
    let mut last_tick = Instant::now();
    let mut input_field = InputField::new();
    let mut config: Configuration = fs::read_to_string("config.json")
        .map(|data| serde_json::from_str(&data).unwrap_or_else(|_| Configuration::new(25, 5, 10)))
        .unwrap_or_else(|_| Configuration::new(25, 5, 10));

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
                    if timer.left_view && !left_view_done && timer.timeleft_secs != 0 {
                        timer.tick();
                        left_view_done = true;
                    } else if !timer.left_view && !right_view_done && timer.timeleft_secs != 0 {
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
        if crossterm::event::poll(timeout)? {
            if let Event::Key(key) = event::read()? {
                if input_field.content.is_empty() && KeyCode::Char('q') == key.code {
                    return Ok(());
                } else if KeyCode::Tab == key.code {
                    config.next();
                } else {
                    handle_key_press(key, &mut config, &mut input_field, &mut pause_flag)?;
                }
            }
            terminal.draw(|f| ui(f, &mut config, &mut input_field))?;

            /* TEST 
                //let cursor_style = Style::default().fg(Color::Yellow).modifier(Modifier::BOLD);
            let normal_style = Style::default().fg(Color::Yellow);

            let cursor_pos = input_field.cursor_position;
            let (left, right) = input_field.content.split_at(cursor_pos);

            let text = format!("{}{}{}", left, "█", right);

            let paragraph = Paragraph::new(text.as_ref())
                .style(normal_style)
                .block(Block::default().borders(Borders::ALL));

            terminal.draw(|f| {
                let size = f.size();
                let chunks = Layout::default()
                    .direction(Direction::Horizontal)
                    .constraints([Constraint::Percentage(90)].as_ref())
                    .split(size);

                f.render_widget(paragraph, chunks[0]);
                f.set_cursor(
                    // Put cursor past the end of the input text
                    chunks[0].x + cursor_pos as u16 + 1,
                    // Move one line down, from the border to the input line
                    chunks[0].y + 1,
                );
                //f.set_style(cursor_style);
                //f.write_str("█")?;
                //f.set_style(normal_style);
            });
             TEST */
        }
        i += 1;
    }
}

fn handle_key_press(
    key: KeyEvent,
    config: &mut Configuration,
    input_field: &mut InputField,
    pause_flag: &mut bool,
) -> Result<(), io::Error> {
    Ok(if config.index == 0 {
        if input_field.content.is_empty() && KeyCode::Char('h') == key.code {
            config.show_popup = !config.show_popup;
        } else if let KeyCode::Left = key.code {
            input_field.move_cursor_left();
        } else if let KeyCode::Right = key.code {
            input_field.move_cursor_right();
        } else if input_field.content.is_empty() && KeyCode::Char(' ') == key.code {
            *pause_flag = !*pause_flag;
        } else if KeyCode::Esc == key.code {
            input_field.content.clear();
            input_field.cursor_position = 0;
        } else if let KeyCode::Enter = key.code {
            parse_input(&input_field.content, config);
            input_field.content.clear();
            input_field.cursor_position = 0;
        } else if let KeyCode::Char(c) = key.code {
            //input_field.content.push(c);
            input_field.insert_char(c);
        } else if let KeyCode::Backspace = key.code {
            //input_field.content.pop();
            input_field.delete_char();
        }
    } else if config.index == 1 {
        if KeyCode::Esc == key.code {
            config.clear_table_entry();
        } else if let KeyCode::Enter = key.code {
            config.save_table_changes();
        } else if let KeyCode::Char(c) = key.code {
            config.write_table_entry(c);
        } else if let KeyCode::Backspace = key.code {
            config.pop_table_entry();
        } else if let KeyCode::Up = key.code {
            config.previous_table_entry()
        } else if let KeyCode::Down = key.code {
            config.next_table_entry();
        }
    })
}

pub fn get_background_color(darkmode: bool) -> Color {
    if darkmode {
        return Color::Black;
    } else {
        return Color::White;
    };
}

pub fn get_foreground_color(darkmode: bool) -> Color {
    if darkmode {
        return Color::White;
    } else {
        return Color::Black;
    };
}

fn ui<B: Backend>(f: &mut Frame<B>, config: &mut Configuration, input_field: &InputField) {
    let mut size = f.size();
    let block = Block::default().style(
        Style::default()
            .fg(get_foreground_color(config.darkmode))
            .bg(get_background_color(config.darkmode)),
    );
    f.render_widget(block, size);
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
            (90.0 / len_left_view_timers as f32) as u16,
        ));
    }
    constraints_vec.push(Constraint::Percentage(7));

    if len_right_view_timers > 0 {
        constraints_vec2.push(Constraint::Percentage(3));
        for _ in 0..len_right_view_timers {
            constraints_vec2.push(Constraint::Percentage(
                (90.0 / len_right_view_timers as f32) as u16,
            ));
        }
        constraints_vec2.push(Constraint::Percentage(7));
    }

    let titles = config
        .titles
        .iter()
        .map(|t| {
            let (first, rest) = t.split_at(1);
            Spans::from(vec![
                Span::styled(
                    first,
                    Style::default()
                        .fg(get_foreground_color(config.darkmode))
                        .bg(get_background_color(config.darkmode)),
                ),
                Span::styled(
                    rest,
                    Style::default()
                        .fg(get_foreground_color(config.darkmode))
                        .bg(get_background_color(config.darkmode)),
                ),
            ])
        })
        .collect();
    let tabs = Tabs::new(titles)
        .select(config.index)
        .style(
            Style::default()
                .fg(Color::Gray)
                .bg(get_background_color(config.darkmode)),
        )
        .highlight_style(
            Style::default()
                .add_modifier(Modifier::BOLD)
                .bg(Color::Yellow),
        );

    if len_right_view_timers > 0 {
        size.width = size.width / 2;
    }

    let chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints(constraints_vec)
        .split(size);
    f.render_widget(tabs, chunks[0]);

    let chunks_index1 = Layout::default()
        .direction(Direction::Vertical)
        .constraints(vec![Constraint::Percentage(3), Constraint::Percentage(80)])
        .split(size);

    let mut chunks2: Vec<Rect> = Vec::new();
    if len_right_view_timers > 0 {
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
                .block(Block::default().borders(Borders::TOP))
                .style(
                    Style::default()
                        .fg(if left_view_timers[i - 1].is_active {
                            Color::Yellow
                        } else {
                            Color::DarkGray
                        })
                        .bg(get_background_color(config.darkmode)),
                );
            f.render_widget(paragraph, chunks[i]);
        }
        timertab_rendering(
            len_right_view_timers,
            chunks2,
            right_view_timers,
            f,
            input_field,
            &chunks,
            config,
            size,
        );
    } else if config.index == 1 {
        configtab_rendering(config, f, chunks_index1);
    }
}

fn timertab_rendering<B: Backend>(
    len_right_view_timers: usize,
    chunks2: Vec<Rect>,
    right_view_timers: Vec<&Timer>,
    f: &mut Frame<B>,
    input_field: &InputField,
    chunks: &Vec<Rect>,
    config: &Configuration,
    size: Rect,
) {
    if len_right_view_timers > 0 {
        /* loop for timers2 */
        for i in 1..chunks2.len() - 1 {
            let paragraph = Paragraph::new(right_view_timers[i - 1].formatted())
                .block(Block::default().borders(Borders::TOP | Borders::LEFT))
                .style(
                    Style::default()
                        .fg(if right_view_timers[i - 1].is_active {
                            Color::Yellow
                        } else {
                            Color::DarkGray
                        })
                        .bg(get_background_color(config.darkmode)),
                );
            f.render_widget(paragraph, chunks2[i]);
        }
    }
    let input = Paragraph::new(input_field.content.as_ref())
        .style(
            Style::default()
                .fg(Color::Yellow)
                .bg(get_background_color(config.darkmode)),
        )
        .block(Block::default().borders(Borders::ALL).title("Input"));
    f.set_cursor(
        // Put cursor past the end of the input text
        chunks[0].x + input_field.cursor_position as u16 + 1,
        // Move one line down, from the border to the input line
        chunks[chunks.len() - 1].y + 1,
    );
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
        let helptext = fs::read_to_string("helptext.txt").expect("Unable to read helptext file");
        let paragraph = Paragraph::new(helptext)
            .block(Block::default().borders(Borders::ALL))
            .style(
                Style::default()
                    .fg(Color::LightRed)
                    .bg(get_background_color(config.darkmode)),
            );
        let area = centered_rect(80, 50, size);
        f.render_widget(Clear, area); //this clears out the background
        f.render_widget(paragraph, area);
    }
}

fn configtab_rendering<B: Backend>(
    config: &mut Configuration,
    f: &mut Frame<B>,
    chunks: Vec<Rect>,
) {
    let selected_style = Style::default().add_modifier(Modifier::REVERSED);
    let header_cells = ["Configuration", "Value"]
        .iter()
        .map(|h| Cell::from(*h).style(Style::default().fg(get_foreground_color(config.darkmode))));
    let header = Row::new(header_cells)
        .style(Style::default().bg(Color::Yellow))
        .height(1)
        .bottom_margin(2);
    if config.state.selected() == None {
        config.darkmode_str = config.darkmode.to_string();
        config.reverseadding_str = config.reverseadding.to_string();
        config.pomodoro_time_table_str = config.pomodoro_time.to_string();
        config.pomodoro_smallbreak_table_str = config.pomodoro_smallbreak.to_string();
        config.pomodoro_bigbreak_table_str = config.pomodoro_bigbreak.to_string();
    }
    let items = vec![
        vec!["darkmode".to_string(), config.darkmode_str.to_owned()],
        vec![
            "reverse adding of timers".to_string(),
            config.reverseadding_str.to_owned(),
        ],
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
            .fg(Color::Yellow)
            .bg(get_background_color(config.darkmode)),
    ))
    .alignment(Alignment::Center)
    .wrap(Wrap { trim: true });
    f.render_widget(paragraph, chunks[chunks.len() - 1]);
}

/// helper function to create a centered rect using up certain percentage of the available rect `r`
fn centered_rect(percent_x: u16, percent_y: u16, r: Rect) -> Rect {
    let v_margin = r.height * (100 - percent_y) / 200;
    let h_margin = r.width * (100 - percent_x) / 200;

    Rect {
        x: r.x + h_margin,
        y: r.y + v_margin,
        width: r.width - 2 * h_margin,
        height: r.height - 2 * v_margin,
    }
}
