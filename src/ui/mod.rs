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
    Quit,
}

pub fn welcome() {
    println!("Welcome to spotify sorter!");
    println!();
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
        println!("q - Quit");
        println!();

        print!("Choice: ");
        let user_input: String = read!();
        println!();

        match user_input.trim() {
            "r" => break TrackAction::Remove,
            "s" => break TrackAction::Skip,
            "q" => break TrackAction::Quit,
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

        _ = clearscreen::clear(); // workaround because for some reason the inner
                                  // loop (that parses input) doesn't run fully
                                  // before printing the list again
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
