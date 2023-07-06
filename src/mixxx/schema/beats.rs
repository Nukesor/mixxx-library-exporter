#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Beat {
    #[prost(int32, optional, tag = "1")]
    pub frame_position: ::core::option::Option<i32>,
    #[prost(bool, optional, tag = "2", default = "true")]
    pub enabled: ::core::option::Option<bool>,
    #[prost(enumeration = "Source", optional, tag = "3", default = "Analyzer")]
    pub source: ::core::option::Option<i32>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Bpm {
    #[prost(double, optional, tag = "1")]
    pub bpm: ::core::option::Option<f64>,
    #[prost(enumeration = "Source", optional, tag = "2", default = "Analyzer")]
    pub source: ::core::option::Option<i32>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct BeatMap {
    #[prost(message, repeated, tag = "1")]
    pub beat: ::prost::alloc::vec::Vec<Beat>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct BeatGrid {
    #[prost(message, optional, tag = "1")]
    pub bpm: ::core::option::Option<Bpm>,
    #[prost(message, optional, tag = "2")]
    pub first_beat: ::core::option::Option<Beat>,
}
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
#[repr(i32)]
pub enum Source {
    Analyzer = 0,
    FileMetadata = 1,
    User = 2,
}
impl Source {
    /// String value of the enum field names used in the ProtoBuf definition.
    ///
    /// The values are not transformed in any way and thus are considered stable
    /// (if the ProtoBuf definition does not change) and safe for programmatic use.
    pub fn as_str_name(&self) -> &'static str {
        match self {
            Source::Analyzer => "ANALYZER",
            Source::FileMetadata => "FILE_METADATA",
            Source::User => "USER",
        }
    }
    /// Creates an enum from field names used in the ProtoBuf definition.
    pub fn from_str_name(value: &str) -> ::core::option::Option<Self> {
        match value {
            "ANALYZER" => Some(Self::Analyzer),
            "FILE_METADATA" => Some(Self::FileMetadata),
            "USER" => Some(Self::User),
            _ => None,
        }
    }
}
