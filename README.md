# multitimer-tui 🕒

multitimer-tui is a productivity tool that lets you attach timers to a To-Do List with a TUI (Terminal User Interface). It is built with Rust, Crossterm and TUI as the main dependencies. The project gives you the ability to create, edit and delete multiple timers with different descriptions, time lengths and actions after they run out. It also supports Pomodoro timers with adjustable breaks.

## Features

This project has several features that make it a useful and versatile tool for managing timers. Some of these features are:

- ✏️ Modify existing timers in various ways. 
- ⏯️ Pause or resume all timers by pressing Space.
- 🍅 Pomodoro timers are supported, which are a popular technique for time management and productivity.
- ⚡ Actions after all timers done: None, Hibernate or Shutdown.
- 🔊 Sound is played and a notification is displayed (if supported by the system) when a timer expires.
- 💾 Saving the configuration and timers to preserve timers and settings across different sessions.
- 🎨 Choose a color theme for active timers.
- 🌑 Switch between dark mode and light mode.

![multitimer_tui](https://github.com/Jo6a/multitimer-tui/assets/18258350/1810d222-a1cd-4c66-ac04-9c8110c88a0f)

## Usage

You will see a TUI with two tabs: Timer and Config. You can switch between them by pressing Tab. You can create new timers by entering commands in the input line at the bottom of the screen. The syntax for creating timers is:

- `add [minutes] [description]`: adds a timer to the left column with the given minutes and description.
- `add2 [minutes] [description]`: adds a timer to the right column with the given minutes and description.
- `addr [minutes] [description]`: adds a timer to the left column in reverse order with the given minutes and description.
- `addp`: adds a pair of Pomodoro timers to the left column with the durations specified in the Config tab.

You can also edit or delete existing timers by using these commands:

- `rm [id]`: removes the timer with the given id.
- `clear`: removes all timers.
- `move [id1] [id2]`: moves the timer with id1 to the position of id2.
- `moveup [id]`: moves the timer with id up by one position.
- `movedown [id]`: moves the timer with id down by one position.
- `merge [id1] [id2]`: merges the timer with id1 with the timer with id2.
- `plus [id] [minutes]`: increases the time left of the timer with id by minutes.
- `minus [id] [minutes]`: decreases the time left of the timer with id by minutes.
- `rename [id] [description]`: changes the description of the timer with id to description.

You can also pause or resume all timers by pressing Space.

In the Config tab, you can see a table with various configuration options that you can change.

- darkmode: whether to use dark mode or not (true or false).
- active color: the color of active timers (Red, Green, Blue, etc.).
- reverse adding of timers: whether to add new timers to the top or bottom of the column (true or false).
- action after timers done: what action to perform when all timers are done (None, Hibernate, Shutdown).
- pomodoro_time: how long a Pomodoro timer should last in minutes (int).
- pomodoro_smallbreak: how long a small break after a Pomodoro timer should last in minutes (int).
- pomodoro_bigbreak: how long a big break after four Pomodoro timers should last in minutes (int).

The configuration is saved in a file called config.json in the project directory.

To quit the application, you can press q.

## Installation

To install the project, you need to have Rust and Cargo installed on your system. You can follow the instructions [here](https://www.rust-lang.org/tools/install) to install them. Then, you can clone this repository and run `cargo build --release` in the project directory. The executable file will be located in `target/release/multitimer-tui`.

This project is compatible with Windows and Linux operating systems. It uses different commands for playing sounds, displaying notifications and executing actions depending on the OS.

## TODOs

[ ] Add compatibility for taskwarrior

[ ] Add support for macOS

## License

This project is licensed under the MIT License.
