use std::collections::HashMap;

use clearscreen::clear as clearscreen;
use rspotify::model::{FullTrack, SimplifiedPlaylist};
use text_io::read;
use utils::{string_to_half_screen, wrap_text_to_screen};
use yansi::Paint;

use crate::{services, spotify::SpotifyPlaylistsError};

pub(crate) mod track;
pub(crate) mod utils;

pub enum TrackAction {
    Add(Vec<usize>),
    Remove,
    Skip,
    ChangeVolume(bool),
    Quit,
}

pub fn welcome() {
    println!("♪♫♪ {}", "Welcome to Sortify!".bold().italic());
}

pub fn confirm_account(user_name: Option<String>) -> bool {
    println!();

    if let Some(name) = user_name {
        println!(
            "{}",
            wrap_text_to_screen(&format!(
                "{} {}. {}",
                "Logged into Spotify as".italic(),
                name.green(),
                "Press 'l' to log out, 'q' to quit, or any other key to continue.".italic()
            ))
        );

        print!("Choice: ");
        let user_input: char = read!();
        println!();

        if user_input == 'l' {
            if services::log_out() {
                println!("Succesfully logged out. Please restart the program to log in with a different account.");
            } else {
                println!("Failed to log out. Please restart the program to try again.");
            }
        }
        if user_input == 'l' || user_input == 'q' {
            return false;
        }
    } else {
        println!(
            "Error logging into your Spotify account. Please restart the program to try again."
        );
        return false;
    }

    true
}

pub fn choose_source(playlists: &Vec<SimplifiedPlaylist>) -> usize {
    println!("Choose source playlist");
    println!(
        "{}",
        wrap_text_to_screen(&"You'll have the option to 'sort' each track in this playlist. When sorted, tracks are added to other playlists, removed from the source, and added to your liked songs.".to_string()).italic().dim()
    );
    println!();

    let source_index = utils::choose_one(
        &playlists
            .iter()
            .map(|playlist| playlist.name.clone())
            .collect(),
    );

    _ = clearscreen();

    println!(
        "Source playlist is {}",
        playlists[source_index].name.clone()
    );
    println!();

    source_index
}

pub fn handle_track(
    track: &FullTrack,
    playlists: &Vec<SimplifiedPlaylist>,
    image_cache: &mut HashMap<String, String>,
    volume: Option<f32>,
) -> TrackAction {
    let mut selected: Vec<bool> = vec![false; playlists.len()];
    let playlist_names: Vec<&String> = playlists.iter().map(|playlist| &playlist.name).collect();

    loop {
        println!(
            "{}\n\n{}",
            track::display(track, image_cache),
            "Choose playlists to add track to"
        );
        println!();

        for i in 0..playlist_names.len() {
            if selected[i] {
                let line = format!("[✓] {} - {}", i + 1, playlist_names[i]);
                print!("{} ", string_to_half_screen(&line).green());
            } else {
                let line = format!("{} - {}", i + 1, playlist_names[i]);
                print!("{} ", string_to_half_screen(&line));
            };

            if i % 2 != 0 || i == playlist_names.len() - 1 {
                println!();
            }
        }

        println!();
        println!("a - Confirm and add to playlists");
        println!("s - Skip track");
        println!("r - Remove from source without adding");
        if let Some(vol) = volume {
            println!();
            println!(
                "u - Volume up | d - Volume down | Current volume: {:.1}",
                vol
            );
            println!();
        }
        println!("q - Quit");
        println!();

        print!("Choice: ");
        let user_input: String = read!();
        println!();

        match user_input.trim() {
            "r" => break TrackAction::Remove,
            "s" => break TrackAction::Skip,
            "q" => break TrackAction::Quit,
            "u" => {
                if let Some(_) = volume {
                    // causes loop without feedback, so screen needs clearing
                    _ = clearscreen();
                    break TrackAction::ChangeVolume(true);
                }
                ()
            }
            "d" => {
                if let Some(_) = volume {
                    // causes loop without feedback, so screen needs clearing
                    _ = clearscreen();
                    break TrackAction::ChangeVolume(false);
                }
                ()
            }
            "a" => {
                break {
                    let mut indexes: Vec<usize> = Vec::new();
                    for i in 0..selected.len() {
                        if selected[i] {
                            indexes.push(i)
                        }
                    }
                    TrackAction::Add(indexes)
                }
            }
            _ => (),
        }

        for maybe_number in user_input.split_ascii_whitespace() {
            if let Ok(number) = maybe_number.parse::<usize>() {
                if number >= 1 && number <= playlists.len() {
                    selected[number - 1] = !selected[number - 1];
                }
            }
        }

        _ = clearscreen(); // workaround because for some reason the above loop
                           // doesn't run fully before printing the list again
    }
}

pub fn track_action_feedback(
    track: &FullTrack,
    result: Result<services::TrackAction, SpotifyPlaylistsError>,
) {
    _ = clearscreen();

    let track_summary = track::summary(track);

    let msg = match result {
        Ok(action) => match action {
            services::TrackAction::Add(_) => format!("Sucessfully sorted {}", track_summary),
            services::TrackAction::Remove(_) => format!(
                "Removed {} from source playlist without sorting it",
                track_summary
            ),
            services::TrackAction::Skip => format!("Skipped {}", track_summary),
        },
        Err(playlists_error) => match playlists_error {
            SpotifyPlaylistsError::Add(playlists) => format!(
                "Failed to add {} to the playlist(s) {}",
                track_summary,
                playlists.join(", ")
            ),
            SpotifyPlaylistsError::Remove(playlists) => format!(
                "Failed to remove {} from the playlist(s) {}",
                track_summary,
                playlists.join(", ")
            ),
        },
    };

    println!("{}", msg);
    println!();
}

pub fn goodbye(source_playlist_name: Option<&String>) {
    let bye = "See you next time ♪♫♪";

    if let Some(name) = source_playlist_name {
        println!("You've sorted all the tracks in {}! {}", name, bye);
    } else {
        println!("{}", bye);
    }
}
