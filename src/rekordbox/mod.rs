use std::path::PathBuf;

use anyhow::{Context, Result};
use percent_encoding::{percent_encode, NON_ALPHANUMERIC};
use url::Url;

use self::schema::{
    library::Library,
    playlists::{Playlist, PlaylistTrack, Playlists},
    tracks::{translate_key, translate_rating, Cue, Tempo, Track, TrackContent, TrackKind, Tracks},
};
use crate::{
    config::Config,
    mixxx::library::{Library as MixxxLibrary, Track as MixxxTrack, TrackLocation},
};

pub mod schema;

pub fn mixxx_to_rekordbox(config: &Config, mixxx_library: MixxxLibrary) -> Result<Library> {
    // Go through all mixxx tracks and create the respective rekordbox tracks.
    let mut rekordbox_tracks = Vec::new();
    for (_, mixxx_track) in mixxx_library.tracks {
        let rekordbox_track = convert_track(config, mixxx_track)?;
        rekordbox_tracks.push(rekordbox_track);
    }

    let mut rekordbox_playlists = Vec::new();
    // Go through all playlists and create respective rekordbox playlists
    for mixxx_playlist in mixxx_library.playlists {
        // Don't show hidden playlists
        if mixxx_playlist.hidden {
            continue;
        }

        let playlist_tracks = mixxx_playlist
            .track_ids
            .iter()
            .map(|key| PlaylistTrack::new(*key))
            .collect();

        rekordbox_playlists.push(Playlist::new(mixxx_playlist.name, playlist_tracks));
    }

    // Recordbox doesn't have the concept of crates, which is why we treat them
    // to normal playlists as well.
    for mixxx_crate in mixxx_library.crates {
        // Don't show hidden crates
        if mixxx_crate.hidden {
            continue;
        }

        let crate_tracks = mixxx_crate
            .track_ids
            .iter()
            .map(|key| PlaylistTrack::new(*key))
            .collect();

        rekordbox_playlists.push(Playlist::new(mixxx_crate.name, crate_tracks));
    }

    Ok(Library::new(
        Tracks::new(rekordbox_tracks),
        Playlists::new(rekordbox_playlists),
    ))
}

/// Convert a single mixxx track into a rekordbox style track format.
pub fn convert_track(config: &Config, mixxx_track: MixxxTrack) -> Result<Track> {
    // At first, we have to create the inner content of the entry field.
    let mut track_inner = Vec::new();
    // First up, create the track's tempo info.
    track_inner.push(TrackContent::Tempo(Tempo {
        inizio: 0.0,
        bpm: mixxx_track.technical_info.bpm as u32,
        // TODO: There doesn't seem to be a Mixxx equivalent.
        // Either remove it or leave it empty.
        // The rekordbox format is: "4/4"
        metro: "".into(),
        // TODO: No idea what this is or where to get it from in Mixxx.
        battito: "1".into(),
    }));

    // Now create the cue points.
    for cue in mixxx_track.cues {
        // TODO: Check where the start can be set
        track_inner.push(TrackContent::Cue(Cue::new(cue.label, 10.0, cue.r#type)))
    }

    let location = get_track_location(config, mixxx_track.location)?;

    Ok(Track {
        track_id: mixxx_track.id.to_string(),
        name: mixxx_track.title,
        artist: mixxx_track.artist,
        composer: mixxx_track.composer,
        album: mixxx_track.album.unwrap_or_default(),
        grouping: String::new(),
        genre: mixxx_track.genre.unwrap_or_default(),
        kind: TrackKind::from_string(mixxx_track.filetype),
        size: 0.to_string(),
        total_time: mixxx_track.technical_info.duration as u32,
        disc_number: 1,
        track_number: mixxx_track.tracknumber.unwrap_or_default(),
        year: mixxx_track
            .year
            .parse::<u16>()
            .map(|i| i.to_string())
            .unwrap_or_default(),
        average_bpm: mixxx_track.technical_info.bpm.to_string(),
        date_added: mixxx_track.metadata.datetime_added.format("%F").to_string(),
        bit_rate: mixxx_track.technical_info.bitrate,
        sample_rate: mixxx_track.technical_info.bitrate,
        comments: mixxx_track.comment.unwrap_or_default(),
        play_count: mixxx_track.metadata.timesplayed,
        rating: translate_rating(mixxx_track.metadata.rating),
        location,
        remixer: "".into(),
        tonality: translate_key(&mixxx_track.technical_info.key).into(),
        label: "".into(),
        mix: "".into(),

        values: track_inner,
    })
}

#[cfg(not(target_os = "windows"))]
pub fn get_track_location(_config: &Config, mixxx_location: TrackLocation) -> Result<String> {
    // Since we're on unix, we can just use the actual path.
    let path = PathBuf::from(&mixxx_location.location);

    // The path needs to be url-encoded, since it's basically an URL.
    let encoded_path = encode_path(&path)?;

    Ok(encoded_path)
}

/// Windows needs a bit of special handling, since we assume that we're running Mixxx on a unix
/// filesystem.
/// -> We have to convert unix-style paths to Windows style paths.
#[cfg(target_os = "windows")]
pub fn get_track_location(config: &Config, mixxx_location: TrackLocation) -> Result<String> {
    use anyhow::bail;
    use path_slash::PathBufExt;

    let unix_path = PathBuf::from_slash(&mixxx_location.location);
    let source_root = PathBuf::from_slash(&config.source_library_root);

    if !unix_path.starts_with(&source_root) {
        bail!("Mixxx path '{unix_path:?}' is not in source_library_root {source_root:?}");
    }

    let relative_path = unix_path
        .strip_prefix(&source_root)
        .context("Failed to strip prefix {source_root:?} from '{unix_path:?}'")?;

    // Start at the target library root
    let mut path = PathBuf::from(&config.target_library_root);
    // Add the relative path from the library root to the actual track.
    path = path.join(relative_path);

    let encoded_path = encode_path(&path)?;
    Ok(encoded_path)
}

/// the inner workings of get_track_location that takes any pathbuf and converts it to a file URI.
fn encode_path(path: &PathBuf) -> Result<String> {
    // All rekordbox tracks are URLs. Since we're on the local machine, we start with this path.
    let mut url = Url::parse("file:///localhost/").unwrap();

    let dir_path = path
        .parent()
        .context("File doesn't have a parent directory: {path:?}")?;
    let file_name = path
        .file_name()
        .context("File doesn't have a filename: {path:?}")?;

    // Add all parts of the directory containing the actual track, one-by-one.
    for path_part in dir_path.into_iter() {
        if path_part == "/" {
            continue;
        }

        // Url-Encode the directory
        let mut encoded_part =
            percent_encode(path_part.to_string_lossy().as_bytes(), NON_ALPHANUMERIC).to_string();
        // Add a `/` to it, since we want to add at least one other element.
        encoded_part.push('/');

        // Join it to the url
        url = url
            .join(&encoded_part)
            .context("Failed to parse path part: {path_part:?}")?;
    }

    // Add the url-encoded filename
    let encoded_filename =
        percent_encoding::percent_encode(&file_name.to_string_lossy().as_bytes(), NON_ALPHANUMERIC)
            .to_string();
    url = url
        .join(&encoded_filename)
        .context("Failed to parse path part: {path_part:?}")?;

    // The path needs to be url-encoded, since it's basically an URL.
    Ok(url.as_str().to_owned())
}
