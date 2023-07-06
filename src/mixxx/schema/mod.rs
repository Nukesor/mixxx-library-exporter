pub mod cue;
pub mod mcrate;
pub mod playlist;
pub mod track;

// Include the `beats` module, which is generated from beats.proto.
pub mod beats {
    include!(concat!(env!("OUT_DIR"), "/mixxx.beats.rs"));
}
