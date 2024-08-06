use std::{collections::HashMap, ops::ControlFlow};

pub mod audio;
mod services;
mod spotify;
mod ui;

fn main() {
    ui::welcome();

    let spotify = spotify::authenticate();
    let playlists = spotify::my_playlists(&spotify);
    let mut image_cache: HashMap<String, String> = HashMap::new();

    if !ui::confirm_account(spotify::user_name(&spotify)) {
        ui::goodbye(None);
        return;
    }

    let source_playlist_index = ui::choose_source(&playlists);
    let source_playlist_id = playlists[source_playlist_index].id.clone_static();

    let tracks = spotify::tracks_in_playlist(&spotify, source_playlist_id.clone_static());

    let mut audio_player = audio::AudioPlayer::new();

    for track in tracks {
        if let ControlFlow::Break(_) = services::handle_track(
            track,
            &playlists,
            &mut image_cache,
            &source_playlist_id,
            &spotify,
            &mut audio_player,
        ) {
            ui::goodbye(None);
            return;
        }
    }

    ui::goodbye(Some(&playlists[source_playlist_index].name));
}
