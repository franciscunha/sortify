use rodio::{Decoder, OutputStream, Sink};
use rspotify::model::FullTrack;
use std::io::{Cursor, Read};

pub struct AudioPlayer {
    _stream: OutputStream,
    sink: Sink,
}

impl AudioPlayer {
    pub fn new(track: &FullTrack) -> Option<AudioPlayer> {
        // make request for audio file, saving it in buffer
        let mut buffer: Vec<u8> = Vec::new();
        _ = ureq::get(&track.preview_url.clone()?)
            .call()
            .ok()?
            .into_reader()
            .read_to_end(&mut buffer);

        // Get an output stream handle to the default physical sound device
        let (stream, stream_handle) = OutputStream::try_default().ok()?;

        // Decoder requires its source to implement both Read and Seek, add them to the bytes via Cursor
        let source = Decoder::new_looped(Cursor::new(buffer)).ok()?;

        // we play the sound using a sink instead of play_raw to be able to later stop it
        let sink = Sink::try_new(&stream_handle).ok()?;
        sink.append(source);

        Some(AudioPlayer {
            _stream: stream,
            sink,
        })
    }

    pub fn stop(self) {
        self.sink.stop();
    }

    pub fn volume_up(&self) {
        self.sink.set_volume(self.sink.volume() + 0.1);
    }

    pub fn volume_down(&self) {
        self.sink.set_volume(self.sink.volume() - 0.1);
    }

    pub fn volume(&self) -> f32 {
        self.sink.volume()
    }
}

pub fn play_track_preview(track: &FullTrack) -> Option<AudioPlayer> {
    AudioPlayer::new(track)
}
