use crate::configuration::Configuration;
use crate::timer::Timer;

pub fn add_timer(
    argument1: &String,
    argument2: &mut String,
    routine: &str,
    config: &mut Configuration,
) {
    let timer = config.create_timer_for_input(argument1, argument2, routine != "add2");
    config.add_timer_to_config(timer);
}

pub fn add_pomodoro_timer(config: &mut Configuration) {
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

    config.add_timer_to_config(timer1);
    config.add_timer_to_config(timer2);
}

pub fn remove_timer(argument1: &String, config: &mut Configuration) {
    if let Ok(id) = argument1.parse::<u16>() {
        config.timers.retain(|t| t.id != id);
    }
}

pub fn move_timer(argument1: &String, argument2: &String, config: &mut Configuration) {
    if let (Ok(id), Ok(id2)) = (
        argument1[..].parse::<usize>(),
        argument2[..].parse::<usize>(),
    ) {
        let t = config.timers.remove(id);
        config.timers.insert(id2, t);
    }
}

pub fn move_timer_up(argument1: &String, config: &mut Configuration) {
    let id = argument1[..].parse::<usize>().unwrap();
    config.timers.swap(id, id - 1);
}

pub fn move_timer_down(argument1: &String, config: &mut Configuration) {
    let id = argument1[..].parse::<usize>().unwrap();
    config.timers.swap(id, id + 1);
}

pub fn increase_timer(argument1: &String, argument2: &String, config: &mut Configuration) {
    let id = argument1[..].parse::<u16>().unwrap();
    let min = argument2[..].parse::<u16>().unwrap();
    for t in &mut config.timers {
        if t.id == id {
            t.timeleft_secs += min * 60;
            break;
        }
    }
}

pub fn decrease_timer(argument1: &String, argument2: &String, config: &mut Configuration) {
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

pub fn rename_timer(argument1: String, config: &mut Configuration, argument2: String) {
    if let Ok(id) = argument1.parse::<u16>() {
        if let Some(timer) = config.timers.iter_mut().find(|t| t.id == id) {
            timer.description = argument2;
        }
    }
}

pub fn parse_input(input: &str, config: &mut Configuration) {
    if input.is_empty() {
        return;
    }
    let mut parts = input.split_whitespace();
    let routine = parts.next().unwrap_or("");
    let argument1 = parts.next().unwrap_or("").to_string();
    let mut argument2: String = parts.collect::<Vec<&str>>().join(" ");

    match routine {
        "a" | "add" | "add2" => {
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
        "mv" | "move" => {
            move_timer(&argument1, &argument2, config);
        }
        "mu" | "moveup" => {
            move_timer_up(&argument1, config);
        }
        "md" | "movedown" => {
            move_timer_down(&argument1, config);
        }
        "p" | "plus" => {
            increase_timer(&argument1, &argument2, config);
        }
        "m" | "minus" => {
            decrease_timer(&argument1, &argument2, config);
        }
        "rn" | "rename" => {
            rename_timer(argument1, config, argument2);
        }
        _ => {}
    }
    config.write_to_file().unwrap();
    config.update_timers();
}
