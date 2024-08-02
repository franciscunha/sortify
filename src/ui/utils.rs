use terminal_size::{terminal_size, Height, Width};
use text_io::read;

pub fn confirmation(prompt: String) -> bool {
    loop {
        println!("{}", prompt);
        println!("y - Confirm");
        println!("n - Cancel");

        println!();
        print!("Choice: ");
        let user_input: char = read!();
        println!();

        match user_input {
            'y' => break true,
            'n' => break false,
            _ => {
                println!("option {} is invalid, please try again", user_input);
                continue;
            }
        }
    }
}

pub fn choose_one(options: &Vec<String>, prompt: String) -> usize {
    println!("{}", prompt);

    for i in 0..options.len() {
        println!("{} - {}", i + 1, options[i]);
    }

    let mut user_input: usize;
    loop {
        println!();
        print!("Choice: ");
        user_input = read!();

        if user_input >= 1 && user_input <= options.len() {
            break;
        }
        println!("Option {} is invalid, please try again", user_input);
    }
    user_input - 1
}

pub fn screen_width() -> usize {
    if let Some((Width(w), Height(_))) = terminal_size() {
        if w >= 48 {
            48
        } else {
            32
        }
    } else {
        48
    }
}

pub fn center_string(str: &String) -> String {
    let pad_size = screen_width().saturating_sub(str.chars().count()) / 2;
    let pad = " ".repeat(pad_size);

    format!("{}{}{}", pad, str, pad)
}

fn pad_string_right(str: &String, n: usize) -> String {
    let pad_size = n.saturating_sub(str.chars().count());
    let pad = " ".repeat(pad_size);

    format!("{}{}", str, pad)
}

fn clip_string(str: &String, n: usize) -> String {
    format!("{}...", str.chars().take(n - 3).collect::<String>())
}

pub fn string_to_half_screen(str: &String) -> String {
    let half_screen_width = screen_width() / 2;
    if str.chars().count() > half_screen_width {
        clip_string(str, half_screen_width)
    } else {
        pad_string_right(str, half_screen_width)
    }
}
