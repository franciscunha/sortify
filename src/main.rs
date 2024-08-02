use std::{collections::HashMap, ops::ControlFlow};

pub mod audio;
mod services;
mod spotify;
mod ui;

fn main() {
    let spotify = spotify::authenticate();
    let playlists = spotify::my_playlists(&spotify);
    let mut image_cache: HashMap<String, String> = HashMap::new();

    ui::welcome();
    let source_playlist_index = ui::choose_source(&playlists);
    let source_playlist_id = playlists[source_playlist_index].id.clone_static();

    let tracks = spotify::tracks_in_playlist(&spotify, source_playlist_id.clone_static());

    for track in tracks {
        if let ControlFlow::Break(_) = services::handle_track(
            track,
            &playlists,
            &mut image_cache,
            &source_playlist_id,
            &spotify,
        ) {
            return;
        }
    }

    ui::goodbye(Some(&playlists[source_playlist_index].name));
}