use crate::mixxx::library::Library as MixxxLibrary;

use self::{
    library::Library,
    playlists::{Playlist, PlaylistTrack, Playlists},
    tracks::{translate_key, Cue, Tempo, Track, TrackContent, TrackKind, Tracks},
};

pub mod library;
pub mod playlists;
pub mod tracks;

pub fn create_rekordbox_schema(mixxx_library: MixxxLibrary) -> Library {
    // Go through all mixxx tracks and create the respective rekordbox tracks.
    let mut rekordbox_tracks = Vec::new();
    for (_, mixxx_track) in mixxx_library.tracks {
        // At first, we have to create the inner content of the entry field.
        let mut track_inner = Vec::new();
        // First up, create the track's tempo info.
        track_inner.push(TrackContent::Tempo(Tempo {
            inizio: 0.0,
            bpm: mixxx_track.technical_info.bpm as u32,
            metro: "4/4".into(),
            battito: "1".into(),
        }));

        // Now create the cue points.
        for cue in mixxx_track.cues {
            // TODO: Check where the start can be set
            track_inner.push(TrackContent::Cue(Cue::new(cue.label, 10.0, cue.r#type)))
        }

        let rekordbox_track = Track {
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
            rating: 0,                     // TODO
            location: "Lolol path".into(), // TODO
            remixer: "".into(),
            tonality: translate_key(&mixxx_track.technical_info.key).into(),
            label: "".into(),
            mix: "".into(),

            values: track_inner,
        };

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

    Library::new(
        Tracks::new(rekordbox_tracks),
        Playlists::new(rekordbox_playlists),
    )
}
