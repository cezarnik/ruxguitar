mod music_parser;
mod primitive_parser;
pub mod error;
pub mod song_parser;
pub mod midi_event;
pub mod midi_builder;
pub mod song_parser_tests;

/// First tick of a song
pub const FIRST_TICK: u32 = 1;