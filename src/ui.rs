use std::{fs, str::FromStr};

use crossterm::event::{KeyCode, KeyEvent};

use ratatui::{
    backend::Backend,
    layout::{Alignment, Constraint, Direction, Layout, Rect},
    style::{Color, Modifier, Style},
    text::{Line, Span},
    widgets::{Block, Borders, Cell, Clear, Paragraph, Row, Table, Tabs, Wrap},
    Frame,
};
use std::io;

use crate::color::{get_background_color, get_foreground_color, AcceptedColors};
use crate::configuration::Configuration;
use crate::input_field::InputField;
use crate::timer::Timer;
use crate::timer_logic::parse_input;
use crate::ui_states::UiState;

pub fn handle_key_press(
    key: KeyEvent,
    config: &mut Configuration,
    input_field: &mut InputField,
    pause_flag: &mut bool,
) -> Result<(), io::Error> {
    let current_ui = UiState::get_current_ui(config.index);

    match current_ui {
        UiState::TimerUi => match key.code {
            KeyCode::Tab => config.next(),
            KeyCode::Left => input_field.move_cursor_left(),
            KeyCode::Right => input_field.move_cursor_right(),
            KeyCode::Esc => {
                input_field.content.clear();
                input_field.cursor_position = 0;
            }
            KeyCode::Enter => {
                parse_input(&input_field.content, config);
                input_field.content.clear();
                input_field.cursor_position = 0;
            }
            KeyCode::Char(c) => match c {
                'h' => {
                    if input_field.content.is_empty() {
                        config.show_popup = !config.show_popup;
                    } else {
                        input_field.insert_char(c)
                    }
                }
                ' ' => {
                    if input_field.content.is_empty() {
                        *pause_flag = !*pause_flag;
                    } else {
                        input_field.insert_char(c)
                    }
                }
                _ => input_field.insert_char(c),
            },
            KeyCode::Backspace => input_field.delete_char(),
            _ => {}
        },
        UiState::ConfigUi => match key.code {
            KeyCode::Tab => config.next(),
            KeyCode::Esc => config.clear_table_entry(),
            KeyCode::Enter => config.save_table_changes(),
            KeyCode::Backspace => {
                config.pop_table_entry();
            }
            KeyCode::Up => config.previous_table_entry(),
            KeyCode::Down => config.next_table_entry(),
            KeyCode::Right => config.move_value_right(),
            KeyCode::Left => config.move_value_left(),
            _ => {}
        },
    }
    Ok(())
}

pub fn ui<B: Backend>(f: &mut Frame<B>, config: &mut Configuration, input_field: &InputField) {
    let mut size = f.size();
    let block = Block::default().style(
        Style::default()
            .fg(get_foreground_color(config.darkmode))
            .bg(get_background_color(config.darkmode)),
    );
    f.render_widget(block, size);

    let len_right_view_timers = config.num_rightview_timers();
    let len_left_view_timers = config.timers.len() - len_right_view_timers;
    let left_view_timers: Vec<&Timer> = config.timers.iter().filter(|t| t.left_view).collect();
    let right_view_timers: Vec<&Timer> = config.timers.iter().filter(|t| !t.left_view).collect();
    let mut constraints_vec = Vec::new();
    let mut constraints_vec2 = Vec::new();
    constraints_vec.push(Constraint::Percentage(3));
    for _ in 0..len_left_view_timers {
        constraints_vec.push(Constraint::Length(3));
    }
    // Constraints to control all the empty spaces below the timer and the input field.
    // Min 0 so if no space, it won't have any size
    // Max 3 so ensure the input field doesn't over extend
    constraints_vec.push(Constraint::Min(0));
    constraints_vec.push(Constraint::Max(3));

    if len_right_view_timers > 0 {
        constraints_vec2.push(Constraint::Percentage(3));
        for _ in 0..len_right_view_timers {
            constraints_vec2.push(Constraint::Length(3));
        }
        // Constraints to control all the empty spaces below the timer2 and the input field.
        // Min 0 so if no space, it won't have any size
        // Max 3 so ensure the input field doesn't over extend
        constraints_vec2.push(Constraint::Min(0));
        constraints_vec2.push(Constraint::Max(3));
    }

    let titles = config
        .titles
        .iter()
        .map(|t| {
            let (first, rest) = t.split_at(1);
            Line::from(vec![
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
                .bg(AcceptedColors::from_str(&config.activecolor)
                    .unwrap()
                    .to_color()),
        );

    let chunks_index1 = Layout::default()
        .direction(Direction::Vertical)
        .constraints(vec![Constraint::Percentage(3), Constraint::Percentage(80)])
        .split(size)
        .to_vec();

    let mut chunks2 = Vec::new();
    if len_right_view_timers > 0 {
        let mut size2 = size;
        size2.x = size.width / 2;
        size2.width /= 2;
        chunks2 = Layout::default()
            .direction(Direction::Vertical)
            .constraints(constraints_vec2)
            .split(size2)
            .to_vec();
    }

    if len_right_view_timers > 0 {
        size.width /= 2;
    }
    let chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints(&*constraints_vec)
        .split(size);

    f.render_widget(tabs, chunks[0]);

    if config.index == 0 {
        // loop for timers
        // len -2 because last 2 are used for rendering the empty fields and the input field
        for i in 1..chunks.len() - 2 {
            let paragraph = Paragraph::new(left_view_timers[i - 1].formatted())
                .block(Block::default().borders(Borders::ALL))
                .style(
                    Style::default()
                        .fg(if left_view_timers[i - 1].is_active {
                            AcceptedColors::from_str(&config.activecolor)
                                .unwrap()
                                .to_color()
                        } else {
                            Color::DarkGray
                        })
                        .bg(get_background_color(config.darkmode)),
                );
            f.render_widget(paragraph, chunks[i]);
        }

        // Renders the empty spaces below the timer with nothing
        f.render_widget(Paragraph::new(""), chunks[chunks.len() - 2]);

        if len_right_view_timers > 0 {
            size.width *= 2;
        }
        let chunks = Layout::default()
            .direction(Direction::Vertical)
            .constraints(constraints_vec)
            .split(size)
            .to_vec();

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

pub fn timertab_rendering<B: Backend>(
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
        // loop for timers2
        // len -2 because last 2 are used for rendering the empty fields and the input field
        for i in 1..chunks2.len() - 2 {
            let paragraph = Paragraph::new(right_view_timers[i - 1].formatted())
                .block(Block::default().borders(Borders::ALL))
                .style(
                    Style::default()
                        .fg(if right_view_timers[i - 1].is_active {
                            AcceptedColors::from_str(&config.activecolor)
                                .unwrap()
                                .to_color()
                        } else {
                            Color::DarkGray
                        })
                        .bg(get_background_color(config.darkmode)),
                );
            f.render_widget(paragraph, chunks2[i]);
        }
        // Renders the empty spaces below the timer with nothing
        f.render_widget(Paragraph::new(""), chunks2[chunks2.len() - 2]);
    }

    let input = Paragraph::new(&*input_field.content)
        .style(
            Style::default()
                .fg(AcceptedColors::from_str(&config.activecolor)
                    .unwrap()
                    .to_color())
                .bg(get_background_color(config.darkmode)),
        )
        .block(Block::default().borders(Borders::ALL).title("Input"));
    f.set_cursor(
        chunks[0].x + input_field.cursor_position as u16 + 1,
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

pub fn configtab_rendering<B: Backend>(
    config: &mut Configuration,
    f: &mut Frame<B>,
    chunks: Vec<Rect>,
) {
    let selected_style = Style::default().add_modifier(Modifier::REVERSED);
    let header_cells = ["Configuration", "Value"]
        .iter()
        .map(|h| Cell::from(*h).style(Style::default().fg(get_foreground_color(config.darkmode))));
    let header = Row::new(header_cells)
        .style(
            Style::default().bg(AcceptedColors::from_str(&config.activecolor)
                .unwrap()
                .to_color()),
        )
        .height(1)
        .bottom_margin(1);
    if config.state.selected().is_none() {
        config.darkmode_str = config.darkmode.to_string();
        config.activecolor_str = config.activecolor.clone();
        config.reverseadding_str = config.reverseadding.to_string();
        config.move_finished_timer_str = config.move_finished_timer.to_string();
        config.action_timeout_str = config.action_timeout.clone();
        config.pomodoro_time_table_str = config.pomodoro_time.to_string();
        config.pomodoro_smallbreak_table_str = config.pomodoro_smallbreak.to_string();
        config.pomodoro_bigbreak_table_str = config.pomodoro_bigbreak.to_string();
    }
    let items = vec![
        vec!["Darkmode".to_string(), config.darkmode_str.to_owned()],
        vec![
            "Active Color".to_string(),
            config.activecolor_str.to_owned(),
        ],
        vec![
            "Reverse Adding of Timers".to_string(),
            config.reverseadding_str.to_owned(),
        ],
        vec![
            "Move Finished Timer to End".to_string(),
            config.move_finished_timer_str.to_owned(),
        ],
        vec![
            "Action After Timers Done".to_string(),
            config.action_timeout_str.to_owned(),
        ],
        vec![
            "Pomodoro Time".to_string(),
            config.pomodoro_time_table_str.to_owned(),
        ],
        vec![
            "Pomodoro Small Break Time".to_string(),
            config.pomodoro_smallbreak_table_str.to_owned(),
        ],
        vec![
            "Pomodoro Big Break Time".to_string(),
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
    let t: Table<'_> = Table::new(rows)
        .header(header)
        .block(Block::default().borders(Borders::ALL))
        .highlight_style(selected_style)
        .highlight_symbol(">> ")
        .widths(&[
            Constraint::Percentage(50),
            Constraint::Length(30),
            Constraint::Min(10),
        ]);

    // prevent selecting nothing on Config tab
    if config.state.selected().is_none() {
        config.state.select(Some(0))
    }
    f.render_stateful_widget(t, chunks[1], &mut config.state);
    //* */
    let text = "Press <ENTER> to save the configuration";
    let paragraph = Paragraph::new(Span::styled(
        text,
        Style::default()
            .add_modifier(Modifier::ITALIC)
            .fg(AcceptedColors::from_str(&config.activecolor)
                .unwrap()
                .to_color())
            .bg(get_background_color(config.darkmode)),
    ))
    .alignment(Alignment::Center)
    .wrap(Wrap { trim: true });
    f.render_widget(paragraph, chunks[chunks.len() - 1]);
}

/// helper function to create a centered rect using up certain percentage of the available rect `r`
pub fn centered_rect(percent_x: u16, percent_y: u16, r: Rect) -> Rect {
    let v_margin = r.height * (100 - percent_y) / 200;
    let h_margin = r.width * (100 - percent_x) / 200;

    Rect {
        x: r.x + h_margin,
        y: r.y + v_margin,
        width: r.width - 2 * h_margin,
        height: r.height - 2 * v_margin,
    }
}
