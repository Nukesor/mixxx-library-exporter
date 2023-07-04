set dotenv-load := true

run:
    cargo run

# Query the relevant raw data for a given track
track_info track_title:
    #!/bin/zsh
    source .env
    sqlite3 $DATABASE_PATH \
        -init "$SQLITE_CONFIG" \
        -header "
        SELECT
            id, artist, title, duration, bitrate, samplerate, cuepoint, bpm, channels, filetype,
            replaygain, rating, key, bpm_lock, replaygain_peak, tracktotal, last_played_at, source_synchronized_ms
        FROM library
        WHERE
            title='{{ track_title }}'
    "

cue_info track_title:
    #!/bin/zsh
    source .env
    sqlite3 $DATABASE_PATH \
        -init "$SQLITE_CONFIG" \
        -header "
        SELECT
            library.artist,
            library.title,
            cues.*
            FROM cues
        JOIN library ON library.id = cues.track_id
        WHERE
            library.title='{{ track_title }}'
    "
