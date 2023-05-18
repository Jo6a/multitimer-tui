use std::fs;

use crossterm::event::{KeyCode, KeyEvent};

use std::io;
use tui::{
    backend::Backend,
    layout::{Alignment, Constraint, Direction, Layout, Rect},
    style::{Color, Modifier, Style},
    text::{Span, Spans},
    widgets::{Block, Borders, Cell, Clear, Paragraph, Row, Table, Tabs, Wrap},
    Frame, Terminal,
};

use crate::color::{get_active_color, get_background_color, get_foreground_color};
use crate::configuration::Configuration;
use crate::input_field::InputField;
use crate::timer::Timer;
use crate::timer_logic::parse_input;

pub fn draw_input_field<B>(terminal: &mut Terminal<B>, input_field: &InputField)
where
    B: Backend,
{
    let normal_style = Style::default().fg(Color::White);

    let cursor_pos = input_field.cursor_position;
    let (left, right) = input_field.content.split_at(cursor_pos);

    let text = format!("{}{}{}", left, "█", right);

    let paragraph = Paragraph::new(text)
        .style(normal_style)
        .block(Block::default().borders(Borders::ALL));

    terminal
        .draw(|f| {
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
        })
        .unwrap();
}

pub fn handle_key_press(
    key: KeyEvent,
    config: &mut Configuration,
    input_field: &mut InputField,
    pause_flag: &mut bool,
) -> Result<(), io::Error> {
    if config.index == 0 {
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
    };
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
                .bg(get_active_color(&config.activecolor)),
        );

    if len_right_view_timers > 0 {
        size.width /= 2;
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
        size.width *= 2;
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
                            get_active_color(&config.activecolor)
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
        /* loop for timers2 */
        for i in 1..chunks2.len() - 1 {
            let paragraph = Paragraph::new(right_view_timers[i - 1].formatted())
                .block(Block::default().borders(Borders::TOP | Borders::LEFT))
                .style(
                    Style::default()
                        .fg(if right_view_timers[i - 1].is_active {
                            get_active_color(&config.activecolor)
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
                .fg(get_active_color(&config.activecolor))
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
        .style(Style::default().bg(get_active_color(&config.activecolor)))
        .height(1)
        .bottom_margin(2);
    if config.state.selected().is_none() {
        config.darkmode_str = config.darkmode.to_string();
        config.activecolor_str = config.activecolor.clone();
        config.reverseadding_str = config.reverseadding.to_string();
        config.action_timeout_str = config.action_timeout.clone();
        config.pomodoro_time_table_str = config.pomodoro_time.to_string();
        config.pomodoro_smallbreak_table_str = config.pomodoro_smallbreak.to_string();
        config.pomodoro_bigbreak_table_str = config.pomodoro_bigbreak.to_string();
    }
    let items = vec![
        vec![
            "darkmode [true, false]".to_string(),
            config.darkmode_str.to_owned(),
        ],
        vec![
            "active color [Red, Green, Blue, etc.]".to_string(),
            config.activecolor_str.to_owned(),
        ],
        vec![
            "reverse adding of timers [true, false]".to_string(),
            config.reverseadding_str.to_owned(),
        ],
        vec![
            "action after timers done [None, Hibernate, Shutdown]".to_string(),
            config.action_timeout_str.to_owned(),
        ],
        vec![
            "pomodoro_time [int]".to_string(),
            config.pomodoro_time_table_str.to_owned(),
        ],
        vec![
            "pomodoro_smallbreak [int]".to_string(),
            config.pomodoro_smallbreak_table_str.to_owned(),
        ],
        vec![
            "pomodoro_bigbreak [int]".to_string(),
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
            .fg(get_active_color(&config.activecolor))
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
