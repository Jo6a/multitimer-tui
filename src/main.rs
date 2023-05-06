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
    Terminal,
};

use multitimer_tui::configuration::Configuration;
use multitimer_tui::input_field::InputField;
use multitimer_tui::ui;

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

fn run_app<B: Backend>(terminal: &mut Terminal<B>, tick_rate: Duration) -> io::Result<()> {
    let mut last_tick = Instant::now();
    let mut input_field = InputField::new();
    let mut config: Configuration = fs::read_to_string("config.json")
        .map(|data| serde_json::from_str(&data).unwrap_or_else(|_| Configuration::new(25, 5, 10)))
        .unwrap_or_else(|_| Configuration::new(25, 5, 10));

    config.titles = vec!["Timer [1]", "Config [2]"];
    config.update_timers();

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
                config.update_timers();
            }

            terminal.draw(|f| ui::ui(f, &mut config, &input_field))?;

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
                    ui::handle_key_press(key, &mut config, &mut input_field, &mut pause_flag)?;
                }
            }
            terminal.draw(|f| ui::ui(f, &mut config, &mut input_field))?;
        }
        i += 1;
    }
}
