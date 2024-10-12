# Sortify

A command line application to organize Spotify playlists.

Using Sortify, you can choose a playlist as the *source playlist*, and then have the option to *sort* each track in it. When sorting, you choose *destination playlists* to add that track to. When sorted, tracks are added to the other playlists, removed from the source, and added to your liked songs.

![Screenshot](https://raw.githubusercontent.com/franciscunha/sortify/main/screenshot.png)

This is mostly a personal project to help in the very specific way I use Spotify:
- I have a playlist called *buffer* to which I add all songs I hear about
- After a while of listening to *buffer*, I go through a bunch of the songs in it and for each either
  - Discard it if I didn't like it very much; or
  - Add it to my liked songs and to some playlist which matches its vibe.
  
So if you'd like to adhere to this Spoti-flow, this app should help out!

## Usage

On first usage, the app should redirect you to log into Spotify. If that doesn't happen, follow the link shown. After logging in, paste the URL Spotify redirected you to into the app, and you should be logged in. Subsequent usages shouldn't need the same process.

After the initial log in, the app should be self-explanatory.

## Dependencies (Linux only)

Sortify uses [rodio](https://github.com/RustAudio/rodio) to play audio, so it shares its [dependencies](https://github.com/RustAudio/rodio?tab=readme-ov-file#dependencieslinux-only) on Linux.

> [*Install the*] `libasound2-dev` package on Debian and Ubuntu distributions and `alsa-lib-devel` on Fedora.

[Rodio has an issue](https://github.com/RustAudio/rodio/issues/200) that discusses this further.

## Installation

### Cargo

You must have [Rust](https://www.rust-lang.org/) and [Cargo](https://doc.rust-lang.org/cargo/) installed.

```sh
# Install binary from crates.io
cargo install sortify

# Run the application 
sortify
# (if this doesn't work, your shell might not know to look for the binary in ~/.cargo/bin/)
```

### Build from source

You must have [Rust](https://www.rust-lang.org/) and [Cargo](https://doc.rust-lang.org/cargo/) installed.

```sh
# Clone the repository
git clone https://github.com/franciscunha/sortify

# Navigate to the project directory
cd sortify

# Build the project
cargo build --release

# Run the application
./target/release/sortify
```
### Binary release

Will be added in the future.

## Contributing

I wrote this over a week to learn the basics of Rust. There are likely many improvements to be made to the codebase, but since it's just a personal project I'm not setting up a robust contributing system. Let me know if you'd like to contribute to it and we can set something up!