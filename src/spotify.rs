use std::iter;

use rspotify::{
    model::{FullTrack, PlayableItem, PlaylistId, SimplifiedPlaylist, TrackId},
    prelude::*,
    scopes, AuthCodePkceSpotify, Config, Credentials, OAuth,
};

static APP_ID: &'static str = "9c7a1f7848ba4f5b839b4e199e2ed1a9";
static REDIRECT_URI: &'static str = "http://localhost:8888/callback";
static SCOPES: [&'static str; 4] = [
    "playlist-read-private",
    "playlist-read-collaborative",
    "user-library-read",
    "user-library-modify",
];

pub enum SpotifyPlaylistsError {
    Add(Vec<String>),
    Remove(Vec<String>),
}

pub fn authenticate() -> AuthCodePkceSpotify {
    let creds = Credentials::new_pkce(APP_ID);
    let oauth = OAuth {
        redirect_uri: String::from(REDIRECT_URI),
        scopes: scopes!(&SCOPES.join(" ")),
        ..Default::default()
    };

    let mut spotify = AuthCodePkceSpotify::with_config(
        creds,
        oauth,
        Config {
            token_cached: true,
            ..Default::default()
        },
    );
    let url = spotify.get_authorize_url(None).unwrap();
    spotify.prompt_for_token(&url).unwrap();

    spotify
}

pub fn user_name(spotify: &AuthCodePkceSpotify) -> Option<String> {
    spotify.current_user().unwrap().display_name
}

pub fn my_playlists(spotify: &AuthCodePkceSpotify) -> Vec<SimplifiedPlaylist> {
    let user_id = spotify.current_user().unwrap().id;
    spotify
        .current_user_playlists()
        .filter_map(|playlist| playlist.ok())
        .filter(|playlist| playlist.owner.id == user_id)
        .collect()
}

pub fn tracks_in_playlist(
    spotify: &AuthCodePkceSpotify,
    playlist_id: PlaylistId<'static>,
) -> Vec<FullTrack> {
    spotify
        .playlist_items(playlist_id.clone(), None, None)
        .filter_map(|result| {
            result
                .inspect_err(|e| {
                    log::warn!("Error getting item from playlist {}: {}", playlist_id, e)
                })
                .ok()
                .and_then(|playlist_item| playlist_item.track)
                .and_then(|playable_track| match playable_track {
                    PlayableItem::Track(full_track) => Some(full_track),
                    _ => None,
                })
        })
        .collect()
}

pub fn remove_from_playlist(
    spotify: &AuthCodePkceSpotify,
    track_id: &TrackId,
    playlist_id: &PlaylistId<'static>,
) -> Result<(), SpotifyPlaylistsError> {
    log::info!("Removing track from playlist {}", playlist_id);

    let is_ok = spotify
        .playlist_remove_all_occurrences_of_items(
            playlist_id.clone_static(),
            iter::once(PlayableId::Track(track_id.clone())),
            None,
        )
        .inspect_err(|e| {
            log::error!(
                "Failed to remove track {} from playlist {}: {}",
                track_id,
                playlist_id,
                e
            )
        })
        .is_ok();

    if is_ok {
        return Ok(());
    }

    if let Ok(playlist) = spotify.playlist(playlist_id.clone_static(), Some("name"), None) {
        let name = playlist.name;
        Err(SpotifyPlaylistsError::Remove(vec![name]))
    } else {
        Err(SpotifyPlaylistsError::Remove(vec![format!(
            "Playlist with ID {}",
            playlist_id
        )]))
    }
}

pub fn add_to_playlists(
    spotify: &AuthCodePkceSpotify,
    track_id: &TrackId,
    playlist_ids: &Vec<PlaylistId<'static>>,
) -> Result<(), SpotifyPlaylistsError> {
    // keep track of names of playlists that had an error, to inform user
    let mut errors: Vec<String> = Vec::new();

    // for each playlist
    for playlist_id in playlist_ids {
        log::info!("Adding track to playlist {}", playlist_id);

        // try to add track to playlist
        let is_err = spotify
            .playlist_add_items(
                playlist_id.clone_static(),
                iter::once(PlayableId::Track(track_id.clone())),
                None,
            )
            .inspect_err(|e| log::warn!("Failed to add track to playlist: {}", e))
            .is_err();

        // if failed and reason is not because playlist already contain tracks
        if is_err && !is_track_in_playlist(spotify, playlist_id, track_id) {
            log::error!("Track {} is not in playlist {}", track_id, playlist_id);

            // log the playlist name
            errors.push(
                if let Ok(playlist) =
                    spotify.playlist(playlist_id.clone_static(), Some("name"), None)
                {
                    playlist.name
                } else {
                    // if it doesn't have a name
                    format!("Playlist with ID {}", playlist_id)
                },
            );
        }
    }

    // check if track is in liked songs
    let is_track_in_liked_songs = if let Ok(contains_vec) = spotify
        .current_user_saved_tracks_contains(iter::once(track_id.clone_static()))
        .inspect_err(|e| {
            log::error!(
                "Failed to check if track {} is in user's liked songs: {}",
                track_id,
                e
            )
        }) {
        contains_vec.contains(&true)
    } else {
        false
    };

    if !is_track_in_liked_songs {
        // try to save it if it isn't
        if spotify
            .current_user_saved_tracks_add(iter::once(track_id.clone_static()))
            .inspect_err(|e| {
                log::error!(
                    "Failed to add track {} to user's liked songs: {}",
                    track_id,
                    e
                )
            })
            .is_err()
        {
            errors.push(String::from("Liked Songs"));
        }
    }

    if errors.is_empty() {
        Ok(())
    } else {
        Err(SpotifyPlaylistsError::Add(errors))
    }
}

fn is_track_in_playlist(
    spotify: &AuthCodePkceSpotify,
    playlist_id: &PlaylistId<'static>,
    track_id: &TrackId,
) -> bool {
    for full_track in tracks_in_playlist(spotify, playlist_id.clone()) {
        match full_track.id {
            None => continue,
            Some(other_id) => {
                if other_id == *track_id {
                    return true;
                }
            }
        }
    }
    return false;
}
