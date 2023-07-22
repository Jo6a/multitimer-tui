use crate::configuration::Configuration;
use crate::timer::Timer;

pub fn add_timer(
    argument1: &String,
    argument2: &mut String,
    routine: &str,
    config: &mut Configuration,
    reverse_adding: bool,
    color_input: Option<String>,
) {
    let timer = config.create_timer_for_input(argument1, argument2, routine != "add2", color_input);
    config.add_timer_to_config(timer, reverse_adding);
}

pub fn add_pomodoro_timer(config: &mut Configuration) {
    let timer1 = Timer::new(
        "Pomodoro-Timer".to_string(),
        config.pomodoro_time * 60,
        true,
        Some(config.timer_colors["focus"].to_owned()),
    );
    let timer2 = Timer::new(
        "Pomodoro-Break".to_string(),
        if !config.timers.is_empty() && config.timers.len() % 6 == 0 {
            config.pomodoro_bigbreak * 60
        } else {
            config.pomodoro_smallbreak * 60
        },
        true,
        Some(config.timer_colors["break"].to_owned()),
    );

    config.add_timer_to_config(timer1, false);
    config.add_timer_to_config(timer2, false);
}

pub fn remove_timer(argument1: &str, config: &mut Configuration) {
    if let Ok(id) = argument1.parse::<u16>() {
        config.timers.retain(|t| t.id != id);
    }
}

pub fn move_timer(argument1: &str, argument2: &str, config: &mut Configuration) {
    if let (Ok(id), Ok(id2)) = (
        argument1[..].parse::<usize>(),
        argument2[..].parse::<usize>(),
    ) {
        let t = config.timers.remove(id);
        config.timers.insert(id2, t);
    }
}

pub fn move_timer_up(argument1: &str, config: &mut Configuration) {
    let id = match argument1[..].parse::<usize>() {
        Ok(id) => id,
        Err(_) => {
            return;
        }
    };
    config.timers.swap(id, id - 1);
}

pub fn move_timer_down(argument1: &str, config: &mut Configuration) {
    let id = match argument1[..].parse::<usize>() {
        Ok(id) => id,
        Err(_) => {
            return;
        }
    };
    config.timers.swap(id, id + 1);
}

pub fn merge_timers(argument1: &str, argument2: &str, config: &mut Configuration) {
    if let (Ok(id), Ok(id2)) = (
        argument1[..].parse::<usize>(),
        argument2[..].parse::<usize>(),
    ) {
        let mut id_1_found = false;
        let mut id_2_found = false;

        for i in &config.timers {
            if i.id as usize == id {
                id_1_found = true;
            }
            if i.id as usize == id2 {
                id_2_found = true;
            }
        }

        if id_1_found && id_2_found {
            let t = config.timers.remove(id2);
            config.timers[id].description += &format!(" ({})", t.description);
            config.timers[id].timeleft_secs += t.timeleft_secs;
            config.timers[id].initial_time += t.timeleft_secs;
        }
    }
}

pub fn increase_timer(argument1: &str, argument2: &str, config: &mut Configuration) {
    let id = match argument1[..].parse::<u16>() {
        Ok(id) => id,
        Err(_) => {
            return;
        }
    };
    let min = match argument2[..].parse::<u64>() {
        Ok(min) => min,
        Err(_) => {
            return;
        }
    };
    for t in &mut config.timers {
        if t.id == id {
            t.timeleft_secs += min * 60;
            t.initial_time += min * 60;
            break;
        }
    }
}

pub fn decrease_timer(argument1: &str, argument2: &str, config: &mut Configuration) {
    let id = match argument1[..].parse::<u16>() {
        Ok(id) => id,
        Err(_) => {
            return;
        }
    };
    let min = match argument2[..].parse::<u64>() {
        Ok(min) => min,
        Err(_) => {
            return;
        }
    };
    for t in &mut config.timers {
        if t.id == id {
            if t.timeleft_secs < min * 60 {
                t.timeleft_secs = 0;
            } else {
                t.timeleft_secs -= min * 60;
                t.initial_time -= min * 60;
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
    let mut collected_argument2 = parts.collect::<Vec<&str>>();

    // check if the 3rd argument is a valid color
    let color_input = if collected_argument2.len() > 0
        && config
            .timer_colors
            .contains_key(&collected_argument2[0].to_lowercase())
    {
        let color = Some(config.timer_colors[&collected_argument2[0].to_lowercase()].to_owned());
        // remove the color argument from the input
        collected_argument2.remove(0);
        color
    } else {
        None
    };

    let mut argument2 = collected_argument2.join(" ");

    match routine {
        "a" | "add" | "add2" => {
            add_timer(
                &argument1,
                &mut argument2,
                routine,
                config,
                false,
                color_input,
            );
        }
        "ar" | "addr" => {
            add_timer(
                &argument1,
                &mut argument2,
                routine,
                config,
                true,
                color_input,
            );
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
        "merge" => {
            merge_timers(&argument1, &argument2, config);
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
