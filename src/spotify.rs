use std::iter;

use rspotify::{
    model::{FullTrack, PlayableItem, PlaylistId, SimplifiedPlaylist, TrackId},
    prelude::*,
    scopes, AuthCodePkceSpotify, Config, Credentials, OAuth,
};

static APP_ID: &'static str = "9c7a1f7848ba4f5b839b4e199e2ed1a9";
static REDIRECT_URI: &'static str = "http://localhost:8888/callback";
static SCOPES: [&'static str; 2] = ["playlist-read-private", "playlist-read-collaborative"];

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
        .playlist_items(playlist_id, None, None)
        .filter_map(|result| {
            result
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
    playlist_id: PlaylistId<'static>,
) -> Result<(), ()> {
    // TODO simplify error handling here

    let is_ok = spotify
        .playlist_remove_all_occurrences_of_items(
            playlist_id,
            iter::once(PlayableId::Track(track_id.clone())),
            None,
        )
        .is_ok();

    if is_ok {
        Ok(())
    } else {
        Err(())
    }
}

pub fn add_to_playlists(
    spotify: &AuthCodePkceSpotify,
    track_id: &TrackId,
    playlist_ids: Vec<PlaylistId<'static>>,
) -> Result<(), Vec<PlaylistId<'static>>> {
    // TODO simplify error handling or use the IDs on error feedback

    let mut errors: Vec<PlaylistId<'static>> = Vec::new();

    for playlist_id in playlist_ids {
        let is_err = spotify
            .playlist_add_items(
                playlist_id.clone_static(),
                iter::once(PlayableId::Track(track_id.clone())),
                None,
            )
            .is_err();
        // TODO add to liked songs

        if is_err {
            errors.push(playlist_id.clone_static());
        }
    }

    if errors.is_empty() {
        Ok(())
    } else {
        Err(errors)
    }
}
