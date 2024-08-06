use std::{collections::HashMap, ops::ControlFlow};

use rspotify::{
    model::{FullTrack, PlaylistId, SimplifiedPlaylist},
    AuthCodePkceSpotify,
};

use crate::{
    audio,
    spotify::{self, SpotifyPlaylistsError},
    ui,
};

pub enum TrackAction {
    Add(Vec<PlaylistId<'static>>),
    Remove(PlaylistId<'static>),
    Skip,
}

impl TrackAction {
    pub fn from_ui_track_action(
        ui_track_action: &ui::TrackAction,
        playlists: &Vec<SimplifiedPlaylist>,
        source_playlist_id: PlaylistId<'static>,
    ) -> TrackAction {
        match ui_track_action {
            ui::TrackAction::Add(indices) => TrackAction::Add(
                indices
                    .iter()
                    .map(|index| playlists[*index].id.clone_static())
                    .collect(),
            ),
            ui::TrackAction::Remove => TrackAction::Remove(source_playlist_id),
            ui::TrackAction::Skip => TrackAction::Skip,
            ui::TrackAction::Quit => panic!("request to quit application was passed to services"),
        }
    }
}

fn handle_track_action(
    spotify: &AuthCodePkceSpotify,
    track: &FullTrack,
    action: TrackAction,
    source_playlist_id: PlaylistId<'static>,
) -> Result<TrackAction, SpotifyPlaylistsError> {
    // get track's id
    let track_id = track.id.clone().unwrap();

    match action {
        TrackAction::Add(ref playlist_ids) => {
            // call api to add to playlists
            spotify::add_to_playlists(spotify, &track_id, playlist_ids)
                .inspect(|_| {
                    // if it worked, also remove from source
                    _ = spotify::remove_from_playlist(spotify, &track_id, &source_playlist_id);
                })
                // map to this function's return type
                .map(|_| action)
        }
        TrackAction::Remove(ref playlist_id) => {
            // confirm desctructive action
            if ui::utils::confirmation(format!(
                "Do you wish to remove {} from the source playlist?",
                ui::track::summary(track)
            )) {
                // on confirmation, call spotify api to remove from playlist
                spotify::remove_from_playlist(spotify, &track_id, playlist_id).map(|_| action)
            } else {
                // on cancel, treat action as a skip
                Ok(TrackAction::Skip)
            }
        }
        // skip doesn't error
        TrackAction::Skip => Ok(action),
    }
}

pub fn handle_track(
    track: FullTrack,
    playlists: &Vec<SimplifiedPlaylist>,
    image_cache: &mut HashMap<String, String>,
    source_playlist_id: &PlaylistId<'_>,
    spotify: &AuthCodePkceSpotify,
) -> ControlFlow<()> {
    if let None = track.id {
        return ControlFlow::Continue(());
    }

    // start playing track preview in separate thread while other things load
    let audio_player = audio::play_track_preview(&track);

    // spin up ui for a track and get user's interaction
    let ui_action = ui::handle_track(&track, playlists, image_cache);

    // if user chose to quit
    if let ui::TrackAction::Quit = ui_action {
        // stop audio
        if let Some(audio) = audio_player {
            audio.stop();
        }
        // and let caller know
        return ControlFlow::Break(());
    }

    // interact with spotify api
    let result = handle_track_action(
        spotify,
        &track,
        TrackAction::from_ui_track_action(&ui_action, playlists, source_playlist_id.clone_static()),
        source_playlist_id.clone_static(),
    );

    // inform user of success or failure
    ui::track_action_feedback(&track, result);

    // stop playing track preview before moving on to next one
    if let Some(audio) = audio_player {
        audio.stop();
    }

    ControlFlow::Continue(())
}

pub fn log_out() -> bool {
    let path = std::path::Path::new(".spotify_token_cache.json");
    path.exists() && std::fs::remove_file(path).is_ok()
}
